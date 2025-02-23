use super::nodes::{
    compare::CompareNode, execute_position::ExecutePositionNode, fuse_signals::FuseSignalsNode,
    market_data::MarketDataNode, sma::SMANode, split_candles::SplitCandlesNode,
    to_signal::ToSignalNode,
};
use crate::{
    create_nodes,
    types::{decimal_sequence::DecimalSequence, mask::Mask, signal::Signal},
};
use anyhow::{anyhow, Result};
use egui::{TextBuffer, Ui};
use egui_snarl::{InPin, NodeId, OutPin};
use polars::frame::DataFrame;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, sync::Arc};

pub enum NodeDataType {
    Mask,
    Signal,
    DecimalSequence,
    Candles,
}

#[derive(Clone, Debug)]
pub enum NodeDataTypeWithValue {
    Mask(Mask),
    Signal(Signal),
    DecimalSequence(DecimalSequence),
    Candles(DataFrame),
}

pub type CheapCloneNodeDataTypeWithValue = Arc<NodeDataTypeWithValue>;

#[derive(Serialize, Deserialize)]
pub struct EFNodeFNSerialized<'a> {
    pub node_name: Cow<'a, str>,
    pub arguments: serde_json::Value,
    #[serde(skip)]
    pub loaded_node: Option<Box<dyn EFNodeFn>>,
}

impl EFNodeFNSerialized<'_> {
    pub fn load_node(&mut self) -> Result<()> {
        let loaded_node: Box<dyn EFNodeFn> = create_nodes!(
            self,
            SMANode,
            CompareNode,
            MarketDataNode,
            ExecutePositionNode,
            SplitCandlesNode,
            ToSignalNode,
            FuseSignalsNode
        );
        self.loaded_node = Some(loaded_node);
        Ok(())
    }

    pub fn save_node(&mut self) {
        self.arguments = self.get_node().export_data();
    }

    pub fn get_node_mut(&mut self) -> &mut Box<dyn EFNodeFn> {
        return self
            .loaded_node
            .as_mut()
            .expect("Node should be loaded when calling get_node!");
    }

    pub fn get_node(&self) -> &Box<dyn EFNodeFn> {
        return self
            .loaded_node
            .as_ref()
            .expect("Node should be loaded when calling get_node!");
    }
}

pub trait EFNodeFn: Send + Sync {
    fn export_data(&self) -> serde_json::Value {
        serde_json::Value::Null
    }
    fn get_name(&self) -> &'static str;
    fn get_inputs(&self) -> &[(&'static str, NodeDataType)];
    fn get_outputs(&self) -> &[(&'static str, NodeDataType)];
    fn show_header(
        &mut self,
        node_id: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        scale: f32,
    ) -> bool {
        false
    }
    fn process_data(
        &self,
        input_args: &[crate::node_editor::node_trait::CheapCloneNodeDataTypeWithValue],
    ) -> Result<Vec<crate::node_editor::node_trait::CheapCloneNodeDataTypeWithValue>> {
        Ok(vec![])
    }
}
