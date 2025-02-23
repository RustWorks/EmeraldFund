use crate::node_editor::node_trait::{EFNodeFn, NodeDataType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExecutePositionNode;

impl EFNodeFn for ExecutePositionNode {
    fn get_name(&self) -> &'static str {
        "ExecutePositionNode"
    }

    fn get_inputs(&self) -> &[(&'static str, crate::node_editor::node_trait::NodeDataType)] {
        return &[("Signal", NodeDataType::Signal)];
    }

    fn get_outputs(&self) -> &[(&'static str, crate::node_editor::node_trait::NodeDataType)] {
        return &[];
    }
}
