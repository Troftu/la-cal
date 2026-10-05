//! Message router for fan-out delivery of OMS messages to subscribers.
//!
//! Modified 2026-10-04 from SLEET e3c2d798a229766571ed7be1f5b28d2dcdb51868:
//! adapt selected routing tests to message-type topics and avoid a mandated
//! round-robin test oracle. See ../../../NOTICE.
//! Modified 2026-10-05 (ticket 19): fail overflowing connections, invalidate
//! queued registrations, and synchronously clean up connection guards on abort.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TrySendError;
use tokio_util::sync::CancellationToken;

use sleet_types::owp::ServerOp;

type SubscriptionKey = (u64, String);

/// A queued message tied to the registration that selected it, not just its SID.
#[derive(Debug)]
pub struct Delivery {
    op: ServerOp,
    registration: CancellationToken,
}

impl Delivery {
    /// Discard delivery selected before its registration was removed. A reused
    /// SID has a different token and cannot inherit the old queued message.
    pub fn into_op(self) -> Option<ServerOp> {
        (!self.registration.is_cancelled()).then_some(self.op)
    }
}

#[derive(Debug)]
struct Registration {
    sub: Subscription,
    active: CancellationToken,
}

/// Connection-owned registrations are disposed synchronously even on task abort.
pub struct ConnectionGuard {
    router: Arc<Router>,
    connection_id: u64,
    failed: CancellationToken,
}

impl ConnectionGuard {
    /// Cancellation signal used to interrupt a blocked socket write on overflow.
    pub fn failure_token(&self) -> CancellationToken {
        self.failed.clone()
    }
}

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        self.router.remove_connection_now(self.connection_id);
    }
}

/// A registered subscription for receiving messages on a topic.
#[derive(Debug)]
pub struct Subscription {
    /// Subscription identifier scoped to the owning connection.
    pub sid: String,
    /// Connection that owns this subscription.
    pub connection_id: u64,
    /// Topic to subscribe to.
    pub topic: String,
    /// Message name (element) to subscribe to.
    pub message_name: String,
    /// Optional queue group for load-balanced delivery.
    pub group: Option<String>,
    /// Channel for delivering messages to the connection task.
    pub sender: mpsc::Sender<Delivery>,
}

/// State for a single queue group.
#[derive(Debug)]
struct GroupState {
    /// Ordered subset of subscription keys in this group.
    keys: BTreeSet<SubscriptionKey>,
    /// Round-robin delivery index.
    counter: AtomicUsize,
}

impl Default for GroupState {
    fn default() -> Self {
        Self {
            keys: BTreeSet::new(),
            counter: AtomicUsize::new(0),
        }
    }
}

/// Only short, nonblocking index/enqueue operations run under this mutex.
#[derive(Debug)]
struct RouterState {
    /// All active subscriptions keyed by (`connection_id`, SID).
    by_sid: HashMap<SubscriptionKey, Registration>,
    /// Failure signals retained until the owning connection guard is dropped.
    connections: HashMap<u64, CancellationToken>,
    /// SIDs keyed by `connection_id` for fast limits and cleanup.
    by_connection: HashMap<u64, HashSet<String>>,
    /// Direct (non-group) subscription keys keyed by topic, then `message_name`.
    direct_by_topic: HashMap<String, HashMap<String, HashSet<SubscriptionKey>>>,
    /// Grouped subscription state keyed by topic, `message_name`, then group.
    grouped_by_message: HashMap<String, HashMap<String, HashMap<String, GroupState>>>,
}

struct DeliveryTarget {
    connection_id: u64,
    sid: String,
    sender: mpsc::Sender<Delivery>,
    registration: CancellationToken,
}

/// Shared message router that dispatches published messages to subscribers.
///
/// Subscription identifiers are unique within a connection. PUB fan-out reads
/// the subscription map; SUB/UNSUB write it. Queue groups use round-robin delivery.
#[derive(Debug)]
pub struct Router {
    state: Mutex<RouterState>,
}

/// Errors returned by [`Router`] subscription operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouterError {
    /// A subscription with this SID already exists on the connection.
    DuplicateSid(String),
    /// No subscription with this SID was found.
    UnknownSid(String),
    /// The connection has reached the maximum allowed subscriptions.
    TooManySubscriptions,
    /// The connection has already failed and cannot add registrations.
    ConnectionClosed,
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

// Preserve the adapter/frontend's existing async seam. The operations themselves
// never suspend, allowing Drop to perform the same cleanup synchronously.
#[allow(clippy::unused_async, clippy::unused_async_trait_impl)]
impl Router {
    /// Create an empty router with no subscriptions.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Mutex::new(RouterState {
                by_sid: HashMap::new(),
                connections: HashMap::new(),
                by_connection: HashMap::new(),
                direct_by_topic: HashMap::new(),
                grouped_by_message: HashMap::new(),
            }),
        }
    }

    fn lock(&self) -> MutexGuard<'_, RouterState> {
        // Cleanup remains available after a panic in a connection task.
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Own this connection's routing lifetime independently of async unwinding.
    pub fn connection_guard(self: &Arc<Self>, connection_id: u64) -> ConnectionGuard {
        let failed = CancellationToken::new();
        self.lock()
            .connections
            .insert(connection_id, failed.clone());
        ConnectionGuard {
            router: Arc::clone(self),
            connection_id,
            failed,
        }
    }

    /// Register a subscription. Returns [`RouterError::DuplicateSid`] if the SID is already used by the connection.
    pub async fn subscribe(&self, sub: Subscription) -> Result<(), RouterError> {
        let mut state = self.lock();
        if state
            .connections
            .get(&sub.connection_id)
            .is_some_and(CancellationToken::is_cancelled)
        {
            return Err(RouterError::ConnectionClosed);
        }

        let connection_subs = state
            .by_connection
            .get(&sub.connection_id)
            .map_or(0, std::collections::HashSet::len);
        if connection_subs >= 2048 {
            return Err(RouterError::TooManySubscriptions);
        }

        let sid = sub.sid.clone();
        let connection_id = sub.connection_id;
        let topic = sub.topic.clone();
        let message_name = sub.message_name.clone();
        let group = sub.group.clone();
        let key = (connection_id, sid.clone());

        if state.by_sid.contains_key(&key) {
            return Err(RouterError::DuplicateSid(sid));
        }

        state.by_sid.insert(
            key.clone(),
            Registration {
                sub,
                active: CancellationToken::new(),
            },
        );
        state
            .by_connection
            .entry(connection_id)
            .or_default()
            .insert(sid);

        if let Some(g) = group {
            state
                .grouped_by_message
                .entry(topic.clone())
                .or_default()
                .entry(message_name)
                .or_default()
                .entry(g)
                .or_default()
                .keys
                .insert(key);
        } else {
            state
                .direct_by_topic
                .entry(topic.clone())
                .or_default()
                .entry(message_name)
                .or_default()
                .insert(key);
        }

        metrics::gauge!("sleet_subscriptions_active", "topic" => topic).increment(1.0);

        drop(state);
        Ok(())
    }

    /// Remove a subscription by SID for a connection.
    #[allow(clippy::significant_drop_tightening)]
    pub async fn unsubscribe(&self, connection_id: u64, sid: &str) -> Result<(), RouterError> {
        let mut state = self.lock();
        let key = (connection_id, sid.to_owned());
        let registration = state
            .by_sid
            .remove(&key)
            .ok_or_else(|| RouterError::UnknownSid(sid.to_owned()))?;
        if let Some(conn_sids) = state.by_connection.get_mut(&connection_id) {
            conn_sids.remove(sid);
            if conn_sids.is_empty() {
                state.by_connection.remove(&connection_id);
            }
        }

        finalize_registration(&mut state, &key, registration);
        Ok(())
    }

    /// Fan out a message to all matching subscribers.
    ///
    /// Direct (non-group) subscribers each receive a copy. Within each queue
    /// group, exactly one member receives the message via round-robin.
    #[allow(clippy::significant_drop_tightening)]
    pub async fn publish(&self, topic: &str, message_name: &str, payload: &str) {
        let mut direct_targets: Vec<DeliveryTarget> = Vec::new();
        let mut grouped_targets: Vec<DeliveryTarget> = Vec::new();

        {
            let state = self.lock();

            // Gather direct targets using zero-allocation lookups
            if let Some(topics) = state.direct_by_topic.get(topic)
                && let Some(sids) = topics.get(message_name)
            {
                for sid in sids {
                    if let Some(registration) = state.by_sid.get(sid) {
                        let sub = &registration.sub;
                        direct_targets.push(DeliveryTarget {
                            connection_id: sub.connection_id,
                            sid: sub.sid.clone(),
                            sender: sub.sender.clone(),
                            registration: registration.active.clone(),
                        });
                    }
                }
            }

            // Gather group targets
            if let Some(topics) = state.grouped_by_message.get(topic)
                && let Some(groups_map) = topics.get(message_name)
            {
                for group_state in groups_map.values() {
                    let len = group_state.keys.len();
                    if len > 0 {
                        let current_idx = group_state.counter.fetch_add(1, Ordering::Relaxed);
                        if let Some(selected_key) = group_state.keys.iter().nth(current_idx % len)
                            && let Some(registration) = state.by_sid.get(selected_key)
                        {
                            let sub = &registration.sub;
                            grouped_targets.push(DeliveryTarget {
                                connection_id: sub.connection_id,
                                sid: sub.sid.clone(),
                                sender: sub.sender.clone(),
                                registration: registration.active.clone(),
                            });
                        }
                    }
                }
            }
        }

        if direct_targets.is_empty() && grouped_targets.is_empty() {
            return;
        }

        let payload_arc: std::sync::Arc<str> = std::sync::Arc::from(payload);

        let mut failed = HashSet::new();
        for target in direct_targets.iter().chain(&grouped_targets) {
            if !failed.contains(&target.connection_id) && !deliver(target, &payload_arc) {
                failed.insert(target.connection_id);
                let mut state = self.lock();
                if let Some(stop) = state.connections.get(&target.connection_id) {
                    stop.cancel();
                }
                remove_registrations(&mut state, target.connection_id);
            }
        }
    }

    /// Remove all subscriptions belonging to a connection (cleanup on disconnect).
    #[allow(clippy::significant_drop_tightening)]
    pub async fn remove_connection(&self, connection_id: u64) {
        self.remove_connection_now(connection_id);
    }

    fn remove_connection_now(&self, connection_id: u64) {
        let mut state = self.lock();
        if let Some(stop) = state.connections.remove(&connection_id) {
            stop.cancel();
        }
        remove_registrations(&mut state, connection_id);
        drop(state);
    }
}

fn remove_registrations(state: &mut RouterState, connection_id: u64) {
    if let Some(sids) = state.by_connection.remove(&connection_id) {
        for sid in sids {
            let key = (connection_id, sid);
            if let Some(registration) = state.by_sid.remove(&key) {
                finalize_registration(state, &key, registration);
            }
        }
    }
}

fn finalize_registration(
    state: &mut RouterState,
    key: &SubscriptionKey,
    registration: Registration,
) {
    registration.active.cancel();
    let removed = registration.sub;
    metrics::gauge!("sleet_subscriptions_active", "topic" => removed.topic.clone()).decrement(1.0);

    if let Some(group) = removed.group {
        remove_group_subscription(state, &removed.topic, &removed.message_name, &group, key);
    } else {
        remove_direct_subscription(state, &removed.topic, &removed.message_name, key);
    }
}

fn remove_direct_subscription(
    state: &mut RouterState,
    topic: &str,
    message_name: &str,
    key: &SubscriptionKey,
) {
    let topic_empty = state.direct_by_topic.get_mut(topic).is_some_and(|topics| {
        let msg_empty = topics.get_mut(message_name).is_some_and(|sids| {
            sids.remove(key);
            sids.is_empty()
        });

        if msg_empty {
            topics.remove(message_name);
        }
        topics.is_empty()
    });

    if topic_empty {
        state.direct_by_topic.remove(topic);
    }
}

fn remove_group_subscription(
    state: &mut RouterState,
    topic: &str,
    message_name: &str,
    group: &str,
    key: &SubscriptionKey,
) {
    let group_empty = state
        .grouped_by_message
        .get_mut(topic)
        .is_some_and(|topics| {
            let msg_empty = topics.get_mut(message_name).is_some_and(|groups| {
                let grp_empty = groups.get_mut(group).is_some_and(|group_state| {
                    group_state.keys.remove(key);
                    group_state.keys.is_empty()
                });

                if grp_empty {
                    groups.remove(group);
                }
                groups.is_empty()
            });

            if msg_empty {
                topics.remove(message_name);
            }
            topics.is_empty()
        });

    if group_empty {
        state.grouped_by_message.remove(topic);
    }
}

fn deliver(target: &DeliveryTarget, payload: &std::sync::Arc<str>) -> bool {
    if target.registration.is_cancelled() {
        return true;
    }
    let op = Delivery {
        op: ServerOp::Msg {
            sid: target.sid.clone(),
            payload: std::sync::Arc::clone(payload),
        },
        registration: target.registration.clone(),
    };

    let max_cap = target.sender.max_capacity();
    let current_cap = target.sender.capacity();
    let depth = max_cap.saturating_sub(current_cap);
    #[allow(clippy::cast_precision_loss)]
    metrics::histogram!("sleet_client_queue_depth").record(depth as f64);

    match target.sender.try_send(op) {
        Ok(()) => {
            metrics::counter!("sleet_messages_delivered_total").increment(1);
            true
        }
        Err(TrySendError::Full(_)) => {
            tracing::error!(connection_id = target.connection_id, sid = %target.sid,
                "outbound queue overflow; failing slow connection and removing all registrations");
            metrics::counter!(
                "sleet_connections_failed_total",
                "error" => "outbound_queue_overflow"
            )
            .increment(1);
            false
        }
        Err(TrySendError::Closed(_)) => {
            tracing::debug!(sid = %target.sid, "dropping message: subscriber closed");
            metrics::counter!(
                "sleet_messages_dropped_total",
                "reason" => "subscriber_closed"
            )
            .increment(1);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc;

    use super::*;

    #[tokio::test]
    async fn basic_delivery() {
        let router = Router::new();
        let (tx, mut rx) = mpsc::channel(4);

        router
            .subscribe(Subscription {
                sid: "s1".to_owned(),
                connection_id: 1,
                topic: "ServiceStatus".to_owned(),
                message_name: "ServiceStatus".to_owned(),
                group: None,
                sender: tx,
            })
            .await
            .unwrap();

        router
            .publish("ServiceStatus", "ServiceStatus", "{\"ServiceStatus\":{}}")
            .await;

        let msg = rx.recv().await.unwrap().into_op().unwrap();
        match msg {
            ServerOp::Msg { sid, payload } => {
                assert_eq!(sid, "s1");
                assert_eq!(&*payload, "{\"ServiceStatus\":{}}");
            }
            other => panic!("expected MSG, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn message_name_filtering() {
        let router = Router::new();
        let (tx, mut rx) = mpsc::channel(4);

        router
            .subscribe(Subscription {
                sid: "s1".to_owned(),
                connection_id: 1,
                topic: "mission".to_owned(),
                message_name: "AccessAssessment".to_owned(),
                group: None,
                sender: tx,
            })
            .await
            .unwrap();

        router.publish("mission", "Other", "{}").await;
        assert!(rx.try_recv().is_err());
    }

    #[tokio::test]
    async fn group_receives_one_copy_per_publication() {
        let router = Router::new();
        let (tx1, mut rx1) = mpsc::channel(4);
        let (tx2, mut rx2) = mpsc::channel(4);

        router
            .subscribe(Subscription {
                sid: "a".to_owned(),
                connection_id: 1,
                topic: "ServiceStatus".to_owned(),
                message_name: "ServiceStatus".to_owned(),
                group: Some("g1".to_owned()),
                sender: tx1,
            })
            .await
            .unwrap();

        router
            .subscribe(Subscription {
                sid: "b".to_owned(),
                connection_id: 2,
                topic: "ServiceStatus".to_owned(),
                message_name: "ServiceStatus".to_owned(),
                group: Some("g1".to_owned()),
                sender: tx2,
            })
            .await
            .unwrap();

        for payload in ["{\"n\":1}", "{\"n\":2}"] {
            router
                .publish("ServiceStatus", "ServiceStatus", payload)
                .await;
            let deliveries: Vec<_> = [rx1.try_recv().ok(), rx2.try_recv().ok()]
                .into_iter()
                .flatten()
                .filter_map(Delivery::into_op)
                .collect();
            assert_eq!(deliveries.len(), 1);
            assert!(
                matches!(&deliveries[0], ServerOp::Msg { payload: received, .. } if &**received == payload)
            );
        }
    }

    #[tokio::test]
    async fn group_identity_is_topic_scoped_and_delivery_is_message_filtered() {
        let router = Router::new();
        let (mission_msg_tx, mut mission_msg_rx) = mpsc::channel(4);
        let (mission_other_tx, mut mission_other_rx) = mpsc::channel(4);
        let (other_msg_tx, mut other_msg_rx) = mpsc::channel(4);

        router
            .subscribe(Subscription {
                sid: "mission-msg".to_owned(),
                connection_id: 1,
                topic: "mission".to_owned(),
                message_name: "Msg".to_owned(),
                group: Some("workers".to_owned()),
                sender: mission_msg_tx,
            })
            .await
            .unwrap();
        router
            .subscribe(Subscription {
                sid: "mission-other".to_owned(),
                connection_id: 2,
                topic: "mission".to_owned(),
                message_name: "Other".to_owned(),
                group: Some("workers".to_owned()),
                sender: mission_other_tx,
            })
            .await
            .unwrap();
        router
            .subscribe(Subscription {
                sid: "other-msg".to_owned(),
                connection_id: 3,
                topic: "other".to_owned(),
                message_name: "Msg".to_owned(),
                group: Some("workers".to_owned()),
                sender: other_msg_tx,
            })
            .await
            .unwrap();

        router.publish("mission", "Msg", "{\"n\":1}").await;
        router.publish("mission", "Other", "{\"n\":2}").await;
        router.publish("other", "Msg", "{\"n\":3}").await;

        assert!(matches!(
            mission_msg_rx.recv().await.and_then(Delivery::into_op),
            Some(ServerOp::Msg { sid, .. }) if sid == "mission-msg"
        ));
        assert!(matches!(
            mission_other_rx.recv().await.and_then(Delivery::into_op),
            Some(ServerOp::Msg { sid, .. }) if sid == "mission-other"
        ));
        assert!(matches!(
            other_msg_rx.recv().await.and_then(Delivery::into_op),
            Some(ServerOp::Msg { sid, .. }) if sid == "other-msg"
        ));

        assert!(mission_msg_rx.try_recv().is_err());
        assert!(mission_other_rx.try_recv().is_err());
        assert!(other_msg_rx.try_recv().is_err());
    }

    #[tokio::test]
    async fn same_sid_on_different_connections_is_independent() {
        let router = Router::new();
        let (tx1, mut rx1) = mpsc::channel(4);
        let (tx2, mut rx2) = mpsc::channel(4);

        router
            .subscribe(Subscription {
                sid: "shared".to_owned(),
                connection_id: 1,
                topic: "ServiceStatus".to_owned(),
                message_name: "ServiceStatus".to_owned(),
                group: None,
                sender: tx1,
            })
            .await
            .unwrap();
        router
            .subscribe(Subscription {
                sid: "shared".to_owned(),
                connection_id: 2,
                topic: "ServiceStatus".to_owned(),
                message_name: "ServiceStatus".to_owned(),
                group: None,
                sender: tx2,
            })
            .await
            .unwrap();

        router
            .publish("ServiceStatus", "ServiceStatus", "{\"n\":1}")
            .await;
        assert!(
            matches!(rx1.recv().await.and_then(Delivery::into_op), Some(ServerOp::Msg { sid, .. }) if sid == "shared")
        );
        assert!(
            matches!(rx2.recv().await.and_then(Delivery::into_op), Some(ServerOp::Msg { sid, .. }) if sid == "shared")
        );

        router.unsubscribe(1, "shared").await.unwrap();
        router
            .publish("ServiceStatus", "ServiceStatus", "{\"n\":2}")
            .await;

        assert!(rx1.try_recv().is_err());
        assert!(
            matches!(rx2.recv().await.and_then(Delivery::into_op), Some(ServerOp::Msg { sid, .. }) if sid == "shared")
        );
    }

    #[tokio::test]
    async fn unsubscribe_stops_delivery() {
        let router = Router::new();
        let (tx, mut rx) = mpsc::channel(4);

        router
            .subscribe(Subscription {
                sid: "s1".to_owned(),
                connection_id: 1,
                topic: "mission".to_owned(),
                message_name: "Msg".to_owned(),
                group: None,
                sender: tx,
            })
            .await
            .unwrap();

        router.unsubscribe(1, "s1").await.unwrap();
        router.publish("mission", "Msg", "{}").await;
        assert!(rx.try_recv().is_err());
    }

    #[tokio::test]
    async fn queued_delivery_cannot_be_attributed_to_a_reused_sid() {
        let router = Router::new();
        let (tx, mut rx) = mpsc::channel(4);
        for sid in ["reused", "retained"] {
            router
                .subscribe(Subscription {
                    sid: sid.to_owned(),
                    connection_id: 1,
                    topic: "ServiceStatus".to_owned(),
                    message_name: "ServiceStatus".to_owned(),
                    group: None,
                    sender: tx.clone(),
                })
                .await
                .unwrap();
        }
        router
            .publish("ServiceStatus", "ServiceStatus", "old")
            .await;
        router.unsubscribe(1, "reused").await.unwrap();
        router
            .subscribe(Subscription {
                sid: "reused".to_owned(),
                connection_id: 1,
                topic: "ServiceStatus".to_owned(),
                message_name: "ServiceStatus".to_owned(),
                group: None,
                sender: tx,
            })
            .await
            .unwrap();
        let old: Vec<_> = [rx.try_recv().unwrap(), rx.try_recv().unwrap()]
            .into_iter()
            .filter_map(Delivery::into_op)
            .collect();
        assert_eq!(old.len(), 1);
        assert!(
            matches!(&old[0], ServerOp::Msg { sid, payload } if sid == "retained" && &**payload == "old")
        );
        router
            .publish("ServiceStatus", "ServiceStatus", "new")
            .await;
        let mut sids = HashSet::new();
        for _ in 0..2 {
            let ServerOp::Msg { sid, payload } = rx.try_recv().unwrap().into_op().unwrap() else {
                panic!("expected message");
            };
            assert_eq!(&*payload, "new");
            sids.insert(sid);
        }
        assert_eq!(sids, ["reused".to_owned(), "retained".to_owned()].into());
    }

    #[tokio::test]
    async fn aborting_connection_owner_disposes_all_registrations() {
        let router = Arc::new(Router::new());
        let (tx, mut rx) = mpsc::channel(4);
        let (installed, ready) = tokio::sync::oneshot::channel();
        let task_router = Arc::clone(&router);
        let owner = tokio::spawn(async move {
            let _guard = task_router.connection_guard(1);
            for (sid, group) in [("direct", None), ("grouped", Some("workers"))] {
                task_router
                    .subscribe(Subscription {
                        sid: sid.to_owned(),
                        connection_id: 1,
                        topic: "ServiceStatus".to_owned(),
                        message_name: "ServiceStatus".to_owned(),
                        group: group.map(str::to_owned),
                        sender: tx.clone(),
                    })
                    .await
                    .unwrap();
            }
            installed.send(()).unwrap();
            std::future::pending::<()>().await;
        });
        ready.await.unwrap();
        router
            .publish("ServiceStatus", "ServiceStatus", "queued before abort")
            .await;
        owner.abort();
        assert!(owner.await.unwrap_err().is_cancelled());
        for sid in ["direct", "grouped"] {
            assert_eq!(
                router.unsubscribe(1, sid).await,
                Err(RouterError::UnknownSid(sid.to_owned()))
            );
            assert!(rx.try_recv().unwrap().into_op().is_none());
        }
        assert!(rx.is_closed());
    }

    #[tokio::test]
    async fn sixty_fifth_delivery_fails_every_registration_on_only_the_slow_connection() {
        let router = Arc::new(Router::new());
        let guard = router.connection_guard(1);
        let failed = guard.failure_token();
        let (slow_tx, mut slow_rx) = mpsc::channel(64);
        let (healthy_tx, mut healthy_rx) = mpsc::channel(64);
        for (connection_id, sender) in [(1, slow_tx.clone()), (2, healthy_tx)] {
            router
                .subscribe(Subscription {
                    sid: "status".to_owned(),
                    connection_id,
                    topic: "ServiceStatus".to_owned(),
                    message_name: "ServiceStatus".to_owned(),
                    group: None,
                    sender,
                })
                .await
                .unwrap();
        }
        // A different topic and group on the same connection must also be removed.
        router
            .subscribe(Subscription {
                sid: "command".to_owned(),
                connection_id: 1,
                topic: "ServiceLifecycleCommand".to_owned(),
                message_name: "ServiceLifecycleCommand".to_owned(),
                group: Some("workers".to_owned()),
                sender: slow_tx.clone(),
            })
            .await
            .unwrap();
        for _ in 0..64 {
            router
                .publish("ServiceStatus", "ServiceStatus", "status")
                .await;
            assert!(healthy_rx.try_recv().unwrap().into_op().is_some());
        }
        assert!(!failed.is_cancelled(), "64 deliveries are allowed");
        router
            .publish("ServiceStatus", "ServiceStatus", "overflow")
            .await;
        assert!(
            failed.is_cancelled(),
            "65th delivery visibly fails the connection"
        );
        assert!(healthy_rx.try_recv().unwrap().into_op().is_some());
        for sid in ["status", "command"] {
            assert_eq!(
                router.unsubscribe(1, sid).await,
                Err(RouterError::UnknownSid(sid.to_owned()))
            );
        }
        assert_eq!(
            router
                .subscribe(Subscription {
                    sid: "new".to_owned(),
                    connection_id: 1,
                    topic: "ServiceStatus".to_owned(),
                    message_name: "ServiceStatus".to_owned(),
                    group: None,
                    sender: slow_tx,
                })
                .await,
            Err(RouterError::ConnectionClosed)
        );
        for _ in 0..64 {
            assert!(slow_rx.try_recv().unwrap().into_op().is_none());
        }
        router
            .publish("ServiceStatus", "ServiceStatus", "healthy continues")
            .await;
        assert!(
            matches!(healthy_rx.try_recv().unwrap().into_op(), Some(ServerOp::Msg { payload, .. }) if &*payload == "healthy continues")
        );
        assert!(slow_rx.try_recv().is_err());
    }

    #[tokio::test]
    async fn unsubscribe_grouped_subscription_stops_delivery() {
        let router = Router::new();
        let (tx, mut rx) = mpsc::channel(4);

        router
            .subscribe(Subscription {
                sid: "grouped".to_owned(),
                connection_id: 1,
                topic: "mission".to_owned(),
                message_name: "Msg".to_owned(),
                group: Some("workers".to_owned()),
                sender: tx,
            })
            .await
            .unwrap();

        router.unsubscribe(1, "grouped").await.unwrap();
        router.publish("mission", "Msg", "{}").await;
        assert!(rx.try_recv().is_err());
    }

    #[tokio::test]
    async fn connection_cleanup_removes_grouped_subscriptions() {
        let router = Router::new();
        let (tx, mut rx) = mpsc::channel(4);

        router
            .subscribe(Subscription {
                sid: "grouped".to_owned(),
                connection_id: 42,
                topic: "mission".to_owned(),
                message_name: "Msg".to_owned(),
                group: Some("workers".to_owned()),
                sender: tx,
            })
            .await
            .unwrap();

        router.remove_connection(42).await;
        router.publish("mission", "Msg", "{}").await;
        assert!(rx.try_recv().is_err());
    }

    #[tokio::test]
    async fn connection_cleanup_removes_subscriptions() {
        let router = Router::new();
        let (tx, mut rx) = mpsc::channel(4);

        router
            .subscribe(Subscription {
                sid: "s1".to_owned(),
                connection_id: 42,
                topic: "mission".to_owned(),
                message_name: "Msg".to_owned(),
                group: None,
                sender: tx,
            })
            .await
            .unwrap();

        router.remove_connection(42).await;
        router.publish("mission", "Msg", "{}").await;
        assert!(rx.try_recv().is_err());
    }

    #[tokio::test]
    async fn cleanup_preserves_other_connections_with_same_sids_and_group() {
        let router = Router::new();
        let (removed_tx, mut removed_rx) = mpsc::channel(4);
        let (remaining_tx, mut remaining_rx) = mpsc::channel(4);
        for (connection_id, sender) in [(1, removed_tx), (2, remaining_tx)] {
            for (sid, group) in [("ordinary", None), ("grouped", Some("workers"))] {
                router
                    .subscribe(Subscription {
                        sid: sid.to_owned(),
                        connection_id,
                        topic: "ServiceStatus".to_owned(),
                        message_name: "ServiceStatus".to_owned(),
                        group: group.map(str::to_owned),
                        sender: sender.clone(),
                    })
                    .await
                    .unwrap();
            }
        }
        router.remove_connection(1).await;
        router
            .publish("ServiceStatus", "ServiceStatus", "{\"ServiceStatus\":{}}")
            .await;
        assert!(removed_rx.try_recv().is_err());
        let mut sids = HashSet::new();
        for _ in 0..2 {
            let ServerOp::Msg { sid, payload } = remaining_rx
                .try_recv()
                .expect("surviving subscription receives")
                .into_op()
                .unwrap()
            else {
                panic!("expected message");
            };
            assert_eq!(&*payload, "{\"ServiceStatus\":{}}");
            assert!(sids.insert(sid));
        }
        assert_eq!(sids, ["ordinary".to_owned(), "grouped".to_owned()].into());
        assert!(remaining_rx.try_recv().is_err());
    }

    #[tokio::test]
    async fn connection_subscription_limit_is_enforced_and_unsubscribe_frees_capacity() {
        let router = Router::new();
        let (tx, _rx) = mpsc::channel(1);

        for n in 0..2048 {
            router
                .subscribe(Subscription {
                    sid: format!("s{n}"),
                    connection_id: 7,
                    topic: "mission".to_owned(),
                    message_name: "Msg".to_owned(),
                    group: None,
                    sender: tx.clone(),
                })
                .await
                .unwrap();
        }

        let result = router
            .subscribe(Subscription {
                sid: "overflow".to_owned(),
                connection_id: 7,
                topic: "mission".to_owned(),
                message_name: "Msg".to_owned(),
                group: None,
                sender: tx.clone(),
            })
            .await;
        assert_eq!(result, Err(RouterError::TooManySubscriptions));

        router.unsubscribe(7, "s0").await.unwrap();
        router
            .subscribe(Subscription {
                sid: "replacement".to_owned(),
                connection_id: 7,
                topic: "mission".to_owned(),
                message_name: "Msg".to_owned(),
                group: None,
                sender: tx,
            })
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn connection_cleanup_frees_subscription_capacity_and_indexes() {
        let router = Router::new();
        let (tx, mut rx) = mpsc::channel(1);

        for n in 0..2048 {
            router
                .subscribe(Subscription {
                    sid: format!("s{n}"),
                    connection_id: 7,
                    topic: "mission".to_owned(),
                    message_name: "Msg".to_owned(),
                    group: None,
                    sender: tx.clone(),
                })
                .await
                .unwrap();
        }

        assert_eq!(
            router
                .subscribe(Subscription {
                    sid: "overflow".to_owned(),
                    connection_id: 7,
                    topic: "mission".to_owned(),
                    message_name: "Msg".to_owned(),
                    group: None,
                    sender: tx.clone(),
                })
                .await,
            Err(RouterError::TooManySubscriptions)
        );

        router.remove_connection(7).await;
        router.publish("mission", "Msg", "{}").await;
        assert!(rx.try_recv().is_err());

        router
            .subscribe(Subscription {
                sid: "replacement".to_owned(),
                connection_id: 7,
                topic: "mission".to_owned(),
                message_name: "Msg".to_owned(),
                group: None,
                sender: tx,
            })
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn saturated_direct_subscriber_closes_connection_without_blocking_others() {
        let router = Router::new();
        let (slow_tx, mut slow_rx) = mpsc::channel(1);
        let (fast_tx, mut fast_rx) = mpsc::channel(1);

        slow_tx
            .send(Delivery {
                op: ServerOp::Msg {
                    sid: "preexisting".to_owned(),
                    payload: std::sync::Arc::from("{}"),
                },
                registration: CancellationToken::new(),
            })
            .await
            .expect("prefill slow subscriber queue");

        router
            .subscribe(Subscription {
                sid: "slow".to_owned(),
                connection_id: 1,
                topic: "mission".to_owned(),
                message_name: "Msg".to_owned(),
                group: None,
                sender: slow_tx,
            })
            .await
            .unwrap();
        router
            .subscribe(Subscription {
                sid: "fast".to_owned(),
                connection_id: 2,
                topic: "mission".to_owned(),
                message_name: "Msg".to_owned(),
                group: None,
                sender: fast_tx,
            })
            .await
            .unwrap();

        router.publish("mission", "Msg", "{\"n\":1}").await;

        let fast_msg = fast_rx
            .recv()
            .await
            .expect("fast subscriber receives message")
            .into_op()
            .unwrap();
        match fast_msg {
            ServerOp::Msg { sid, payload } => {
                assert_eq!(sid, "fast");
                assert_eq!(&*payload, "{\"n\":1}");
            }
            other => panic!("expected MSG, got {other:?}"),
        }

        assert!(
            slow_rx.try_recv().is_ok(),
            "preexisting slow message remains"
        );
        assert!(slow_rx.try_recv().is_err(), "new slow message was dropped");
        assert_eq!(
            router.unsubscribe(1, "slow").await,
            Err(RouterError::UnknownSid("slow".to_owned()))
        );
        router.publish("mission", "Msg", "{\"n\":2}").await;
        assert!(
            matches!(fast_rx.recv().await.and_then(Delivery::into_op), Some(ServerOp::Msg { payload, .. }) if &*payload == "{\"n\":2}")
        );
    }

    #[tokio::test]
    async fn saturated_group_subscriber_closes_connection_without_blocking_publish() {
        let router = Router::new();
        let (tx, mut rx) = mpsc::channel(1);

        tx.send(Delivery {
            op: ServerOp::Msg {
                sid: "preexisting".to_owned(),
                payload: std::sync::Arc::from("{}"),
            },
            registration: CancellationToken::new(),
        })
        .await
        .expect("prefill group subscriber queue");

        router
            .subscribe(Subscription {
                sid: "grouped".to_owned(),
                connection_id: 1,
                topic: "mission".to_owned(),
                message_name: "Msg".to_owned(),
                group: Some("g1".to_owned()),
                sender: tx,
            })
            .await
            .unwrap();

        router.publish("mission", "Msg", "{\"n\":1}").await;

        assert!(rx.try_recv().is_ok(), "preexisting message remains");
        assert!(
            rx.try_recv().is_err(),
            "saturated group delivery was dropped"
        );
        assert_eq!(
            router.unsubscribe(1, "grouped").await,
            Err(RouterError::UnknownSid("grouped".to_owned()))
        );
    }
}
