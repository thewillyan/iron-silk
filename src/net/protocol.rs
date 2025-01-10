use serde::{Deserialize, Serialize};

pub type MsgId = uuid::Uuid;
pub type NodeId = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Msg {
    pub src: NodeId,
    pub dst: NodeId,
    pub body: Body,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "msg_id")]
    pub id: MsgId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<MsgId>,
    #[serde(flatten)]
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Data {
    Init {
        node_id: NodeId,
        node_ids: Vec<NodeId>,
    },
    InitOk,
    Error {
        code: u32,
        text: String,
    },
}
