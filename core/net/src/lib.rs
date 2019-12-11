use awc::Client;
use serde::{Deserialize, Serialize};
use std::{future::Future, pin::Pin};
use ya_service_bus::{BusMessage, RpcHandler, RpcMessage};

pub type NodeID = String; /* TODO: proper NodeID */

// handler: send message to other node

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MessageAddress {
    BroadcastAddress { distance: u32 },
    Node(NodeID),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MessageType {
    Request,
    Reply,
    Error,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub destination: MessageAddress,
    pub module: String,
    pub method: String,
    pub reply_to: NodeID,
    pub request_id: u64,
    pub message_type: MessageType,
}

#[derive(Serialize, Deserialize, Clone)]
struct SendMessage {
    message: Message,
}

impl BusMessage for SendMessage {}

impl RpcMessage for SendMessage {
    const ID: &'static str = "send-message";
    type Item = SendMessage; /* TODO should we use the same type for replies? */
    type Error = (); /* TODO */
}

struct SendMessageHandler {}

impl RpcHandler<SendMessage> for SendMessageHandler {
    type Result = Pin<Box<dyn Future<Output = Result<SendMessage, ()>>>>;

    fn handle(&mut self, _caller: &str, _msg: SendMessage) -> Self::Result {
        unimplemented!()
        /* TODO */
        //futures::future::ready(Ok(NetworkStatus::NotConnected))
        /*Box::pin(
            Client::default()
                .get("http://localhost:8000")
                .send()
                .and_then(|response| futures::future::ready(Ok(SendMessage { message: None }))),
        )*/
    }
}

// handler: network status

#[derive(Serialize, Deserialize, Clone)]
enum NetworkStatus {
    ConnectedToCentralizedServer,
    ConnectedToP2PNetwork,
    NotConnected,
}

impl BusMessage for NetworkStatus {}

#[derive(Serialize, Deserialize, Clone)]
struct GetNetworkStatus {}

impl BusMessage for GetNetworkStatus {}

impl RpcMessage for GetNetworkStatus {
    const ID: &'static str = "get-network-status";
    type Item = NetworkStatus;
    type Error = ();
}

struct GetNetworkStatusHandler {}

impl RpcHandler<GetNetworkStatus> for GetNetworkStatusHandler {
    type Result = futures::future::Ready<Result<NetworkStatus, ()>>; /* dynamic version: Pin<Box<dyn Future<Output = NetworkStatus>>>*/

    fn handle(&mut self, _caller: &str, _msg: GetNetworkStatus) -> Self::Result {
        /* TODO get real network status */
        futures::future::ready(Ok(NetworkStatus::NotConnected)) /* dynamic version: add Box::pin(...) */
    }
}

#[cfg(test)]
mod tests {
    use crate::MessageAddress::BroadcastAddress;

    #[test]
    fn test_serialization() {
        use crate::{Message, MessageAddress, MessageType};
        let m: Message = Message {
            //destination: MessageAddress::Node("0x123".into()),
            destination: BroadcastAddress { distance: 5 },
            module: "module".into(),
            method: "method".into(),
            reply_to: "0x999".into(),
            request_id: 1000,
            message_type: MessageType::Request,
        };
        eprintln!("{}", serde_json::to_string(&m).unwrap())
    }
}
