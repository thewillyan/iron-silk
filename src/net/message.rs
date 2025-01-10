use serde::{Deserialize, Serialize};

pub type MsgId = uuid::Uuid;
pub type NodeId = String;

#[derive(Serialize, Deserialize)]
struct Msg {
    src: NodeId,
    dst: NodeId,
    body: Body,
}

#[derive(Serialize, Deserialize)]
struct Body {
    #[serde(rename = "type")]
    kind: String,
    #[serde(rename = "msg_id")]
    id: MsgId,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_reply_to: Option<MsgId>,
    #[serde(flatten)]
    data: Data,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum Data {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
    Error {
        code: u32,
        text: String,
    },
}
