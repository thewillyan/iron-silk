use burn::tensor::TensorData;
use std::collections::HashMap;

use super::message::{MsgId, NodeId};

pub struct Node {
    id: String,
    weights: TensorData,
    weights_map: HashMap<MsgId, TensorData>,
    propagation_map: HashMap<NodeId, MsgId>,
}
