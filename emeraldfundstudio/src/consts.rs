use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde_json::Value;

use crate::node_editor::{
    node_trait::EFNodeFn,
    nodes::{
        compare::CompareNode, execute_position::ExecutePositionNode, fuse_signals::FuseSignalsNode,
        market_data::MarketDataNode, sma::SMANode, split_candles::SplitCandlesNode,
        to_signal::ToSignalNode,
    },
};

pub static NODE_DEFAULT_VALUES: Lazy<HashMap<&'static str, Value>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("SMANode", SMANode::default().export_data());
    m.insert("CompareNode", CompareNode::default().export_data());
    m.insert("MarketDataNode", MarketDataNode::default().export_data());
    m.insert("ToSignalNode", ToSignalNode::default().export_data());
    m.insert("FuseSignalsNode", FuseSignalsNode::default().export_data());
    m.insert(
        "SplitCandlesNode",
        SplitCandlesNode::default().export_data(),
    );
    m.insert(
        "ExecutePositionNode",
        ExecutePositionNode::default().export_data(),
    );
    m
});
