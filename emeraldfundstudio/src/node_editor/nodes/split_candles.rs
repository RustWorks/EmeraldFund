use crate::{
    node_editor::node_trait::{
        CheapCloneNodeDataTypeWithValue, EFNodeFn, NodeDataType, NodeDataTypeWithValue,
    },
    traits::IntoArc,
};
use anyhow::{anyhow, Result};
use polars::prelude::{ChunkedArray, Float64Type};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SplitCandlesNode;

fn nd_column_to_decimal_sequence(
    series: &ChunkedArray<Float64Type>,
) -> CheapCloneNodeDataTypeWithValue {
    NodeDataTypeWithValue::DecimalSequence(series.clone()).into_arc()
}

impl EFNodeFn for SplitCandlesNode {
    fn get_name(&self) -> &'static str {
        "SplitCandlesNode"
    }

    fn get_inputs(&self) -> &[(&'static str, crate::node_editor::node_trait::NodeDataType)] {
        return &[("Candles", NodeDataType::Candles)];
    }

    fn get_outputs(&self) -> &[(&'static str, crate::node_editor::node_trait::NodeDataType)] {
        return &[
            ("Open", NodeDataType::DecimalSequence),
            ("High", NodeDataType::DecimalSequence),
            ("Low", NodeDataType::DecimalSequence),
            ("Close", NodeDataType::DecimalSequence),
            ("Volume", NodeDataType::DecimalSequence),
        ];
    }

    fn process_data(
        &self,
        input_args: &[crate::node_editor::node_trait::CheapCloneNodeDataTypeWithValue],
    ) -> Result<Vec<crate::node_editor::node_trait::CheapCloneNodeDataTypeWithValue>> {
        if input_args.len() < 1 {
            return Err(anyhow!("should have 1 input!"));
        }

        if let NodeDataTypeWithValue::Candles(df) = &*input_args[0] {
            let opens = df.column("open").unwrap().f64().unwrap();
            let highs = df.column("high").unwrap().f64().unwrap();
            let lows = df.column("low").unwrap().f64().unwrap();
            let closes = df.column("close").unwrap().f64().unwrap();
            let volumes = df.column("volume").unwrap().f64().unwrap();
            // let timestamps = df.column("timestamp").unwrap().u64().unwrap();

            return Ok(vec![
                nd_column_to_decimal_sequence(opens),
                nd_column_to_decimal_sequence(highs),
                nd_column_to_decimal_sequence(lows),
                nd_column_to_decimal_sequence(closes),
                nd_column_to_decimal_sequence(volumes),
            ]);
        }

        Err(anyhow!("First argument must be a DataFrame"))
    }
}
