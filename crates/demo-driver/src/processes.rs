//! Ownership, unexpected-exit observation and bounded cleanup of native children.

use std::{path::Path, process::Stdio, time::Duration};

use tokio::{
    process::Command,
    sync::{mpsc, oneshot},
    task::JoinHandle,
    time::timeout,
};

pub struct Processes {
    owned: Vec<Owned>,
    exits: mpsc::UnboundedSender<String>,
    wait: Duration,
}

struct Owned {
    stop: oneshot::Sender<()>,
    task: JoinHandle<Result<(), String>>,
    name: &'static str,
    log: std::path::PathBuf,
}

impl Processes {
    pub fn new(wait: Duration) -> (Self, mpsc::UnboundedReceiver<String>) {
        let (exits, receiver) = mpsc::unbounded_channel();
        (
            Self {
                owned: vec![],
                exits,
                wait,
            },
            receiver,
        )
    }

    pub fn launch(
        &mut self,
        name: &'static str,
        mut command: Command,
        directory: &Path,
    ) -> Result<(), String> {
        let log = directory.join(format!("{name}.log"));
        let file = std::fs::File::create(&log).map_err(|e| format!("{name} log: {e}"))?;
        let mut child = command
            .stdin(Stdio::null())
            .stdout(file.try_clone().map_err(|e| e.to_string())?)
            .stderr(file)
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| format!("launch {name}: {e}"))?;
        let pid = child.id().ok_or_else(|| format!("{name} has no PID"))?;
        println!("launched {name} pid={pid}");
        let (stop, stopped) = oneshot::channel();
        let exits = self.exits.clone();
        let wait = self.wait;
        let task = tokio::spawn(async move {
            tokio::select! {
                status = child.wait() => {
                    let message = format!("unexpected {name} exit: {status:?}");
                    let _ = exits.send(message);
                    status.map_err(|e| format!("reap {name}: {e}"))?;
                    println!("reaped {name} pid={pid}");
                    return Ok(());
                }
                _ = stopped => (),
            }
            // Do not signal a PID after it has been reaped. A child which exited
            // during this race is still owned until this task waits for it.
            if child.try_wait().map_err(|e| e.to_string())?.is_none() {
                let mut signal = Command::new("/bin/kill");
                signal.args(["-TERM", &pid.to_string()]).kill_on_drop(true);
                let _ = timeout(wait, signal.status()).await;
                if let Ok(status) = timeout(wait, child.wait()).await {
                    status.map_err(|e| format!("reap {name}: {e}"))?;
                } else {
                    eprintln!("{name}: shutdown deadline; forcing termination");
                    child
                        .start_kill()
                        .map_err(|e| format!("terminate {name}: {e}"))?;
                    timeout(wait, child.wait())
                        .await
                        .map_err(|_| format!("{name} reap deadline"))?
                        .map_err(|e| format!("reap {name}: {e}"))?;
                }
            }
            println!("reaped {name} pid={pid}");
            Ok(())
        });
        self.owned.push(Owned {
            stop,
            task,
            name,
            log,
        });
        Ok(())
    }

    pub async fn cleanup(self, failed: bool) -> Result<(), String> {
        // Request application cleanup first, then infrastructure cleanup; every
        // bounded wait runs concurrently so a stall cannot retain other children.
        let tasks: Vec<_> = self
            .owned
            .into_iter()
            .rev()
            .map(|owned| {
                let _ = owned.stop.send(());
                (owned.name, owned.task, owned.log)
            })
            .collect();
        let mut errors = vec![];
        for (name, task, log) in tasks {
            match task.await {
                Ok(Ok(())) => (),
                Ok(Err(error)) => errors.push(error),
                Err(error) => errors.push(format!("{name} cleanup task: {error}")),
            }
            if failed || !errors.is_empty() {
                eprintln!(
                    "{name} diagnostics:\n{}",
                    std::fs::read_to_string(log).unwrap_or_default()
                );
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("; "))
        }
    }
}
