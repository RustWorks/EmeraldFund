use crate::{
    node_editor::node_trait::{EFNodeFn, NodeDataType, NodeDataTypeWithValue},
    traits::IntoArc,
    types::signal::Signal,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FuseSignalsNode {}

impl EFNodeFn for FuseSignalsNode {
    fn get_name(&self) -> &'static str {
        "FuseSignalsNode"
    }

    fn get_inputs(&self) -> &[(&'static str, crate::node_editor::node_trait::NodeDataType)] {
        return &[
            ("Signal 1", NodeDataType::Signal),
            ("Signal 2", NodeDataType::Signal),
        ];
    }

    fn get_outputs(&self) -> &[(&'static str, crate::node_editor::node_trait::NodeDataType)] {
        return &[("Fused Signal", NodeDataType::Signal)];
    }

    fn process_data(
        &self,
        input_args: &[crate::node_editor::node_trait::CheapCloneNodeDataTypeWithValue],
    ) -> Result<Vec<crate::node_editor::node_trait::CheapCloneNodeDataTypeWithValue>> {
        if input_args.len() != 2 {
            return Err(anyhow!("should have 2 inputs!"));
        }

        if let NodeDataTypeWithValue::Signal(sig0) = &*input_args[0] {
            if let NodeDataTypeWithValue::Signal(sig1) = &*input_args[1] {
                let fused_signal: Signal = sig0
                    .iter()
                    .zip(sig1.iter())
                    .map(|(sig0, sig1)| {
                        if *sig0 == 0 {
                            return *sig1;
                        } else {
                            return *sig0;
                        }
                    })
                    .collect();
                return Ok(vec![NodeDataTypeWithValue::Signal(fused_signal).into_arc()]);
            }
        }
        return Err(anyhow!("Unknown input"));
    }
}
