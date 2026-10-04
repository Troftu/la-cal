# LA-CAL Messaging

The domain language for Services communicating through a Language-Agnostic Common API Layer and an Avionics Service Bus.

## Language

**Service**:
An OMS participant with a Service identity and lifecycle. A Service is distinct from the communications server through which it exchanges messages.

**OWP Client**:
A participant on the client side of an OMS WebSocket Protocol connection to an LA-CAL Server.
_Avoid_: Service, when referring only to its communications role

**LA-CAL Server**:
The Language-Agnostic Common API Layer endpoint between OWP Clients and an Avionics Service Bus. This communications role does not itself identify the recipient responsible for executing a lifecycle command.

**ASB**:
The Avionics Service Bus through which OMS participants exchange messages via the Common API Layer.

**OWP Publication Acknowledgment**:
Confirmation that an OWP publication operation completed successfully. It is distinct from subscriber receipt, command execution, and Lifecycle Command Status.

**Lifecycle Command**:
A request to control a Service's lifecycle, represented by `ServiceLifecycleCommand` in UCI.

**Lifecycle Command Status**:
A correlated report about processing a Lifecycle Command, represented by `ServiceLifecycleCommandStatus` in UCI.
_Avoid_: Command echo, publication acknowledgment

**Commander**:
The UCI participant that initiates a command interaction and receives its correlated command status.

**Orchestrator**:
A Service that coordinates other Services by issuing commands and observing their status. It has its own lifecycle and Service Status, distinct from the Services it coordinates.

**Commandee**:
The UCI participant that receives and processes a command and responds with command status. The Commander and Commandee are interaction roles, not necessarily distinct Services.

**Effecting Service**:
The Service designated to carry out a Lifecycle Command, which may be different from the Service affected by it.

**Affected Service**:
A Service whose lifecycle is targeted by a Lifecycle Command.

**Service Status**:
A report of a Service's current state and runtime information, represented by `ServiceStatus` in UCI. It is distinct from the correlated result of a Lifecycle Command.
