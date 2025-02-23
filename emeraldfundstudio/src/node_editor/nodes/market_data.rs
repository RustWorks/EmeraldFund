use crate::{
    node_editor::node_trait::{EFNodeFn, NodeDataType, NodeDataTypeWithValue},
    traits::IntoArc,
    types::candles::generate_candles,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MarketDataNode;

impl EFNodeFn for MarketDataNode {
    fn get_name(&self) -> &'static str {
        "MarketDataNode"
    }

    fn get_inputs(&self) -> &[(&'static str, crate::node_editor::node_trait::NodeDataType)] {
        return &[];
    }

    fn get_outputs(&self) -> &[(&'static str, crate::node_editor::node_trait::NodeDataType)] {
        return &[("Candles", NodeDataType::Candles)];
    }
    fn process_data(
        &self,
        input_args: &[crate::node_editor::node_trait::CheapCloneNodeDataTypeWithValue],
    ) -> Result<Vec<crate::node_editor::node_trait::CheapCloneNodeDataTypeWithValue>> {
        let candles = generate_candles(21, 500)?;
        return Ok(vec![NodeDataTypeWithValue::Candles(candles).into_arc()]);
    }
}
