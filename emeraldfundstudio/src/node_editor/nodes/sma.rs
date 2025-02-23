use crate::node_editor::node_trait::{EFNodeFn, NodeDataType};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SMANode;

impl EFNodeFn for SMANode {
    fn get_name(&self) -> &'static str {
        "SMANode"
    }

    fn get_inputs(&self) -> &[(&'static str, crate::node_editor::node_trait::NodeDataType)] {
        return &[("Input", NodeDataType::DecimalSequence)];
    }

    fn get_outputs(&self) -> &[(&'static str, crate::node_editor::node_trait::NodeDataType)] {
        return &[("Output", NodeDataType::DecimalSequence)];
    }

    fn process_data(
        &self,
        input_args: &[crate::node_editor::node_trait::CheapCloneNodeDataTypeWithValue],
    ) -> Result<Vec<crate::node_editor::node_trait::CheapCloneNodeDataTypeWithValue>> {
        return Ok(vec![input_args[0].clone()]);
    }
}
