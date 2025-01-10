use burn::tensor::TensorData;
use std::collections::HashMap;

use super::protocol::{MsgId, NodeId};

pub struct Node {
    id: NodeId,
    neighbors: Vec<NodeId>,
    weights: Option<TensorData>,
    weights_map: HashMap<MsgId, TensorData>,
    propagation_map: HashMap<NodeId, MsgId>,
}
