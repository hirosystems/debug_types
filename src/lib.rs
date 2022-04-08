/*!

Debug Adapter Protocol types for Rust.

Based on: <https://microsoft.github.io/debug-adapter-protocol/specification>

*/

use events::Event;
use requests::RequestCommand;
use responses::Response;
use serde::{Deserialize, Serialize};

mod events;
mod requests;
mod responses;
mod types;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NumberOrString {
    Number(i32),
    String(String),
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct ProtocolMessage {
    pub seq: i64,
    #[serde(flatten)]
    pub message: MessageKind,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum MessageKind {
    Request(RequestCommand),
    Response(Response),
    Event(Event),
}
