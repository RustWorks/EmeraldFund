use crate::{
    node_editor::node_trait::{EFNodeFn, NodeDataType, NodeDataTypeWithValue},
    traits::IntoArc,
};
use anyhow::{anyhow, Result};
use egui::ComboBox;
use polars::{prelude::ChunkCompareIneq, series::ChunkCompareEq};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, IntoEnumIterator};

#[derive(
    Debug, Serialize, Deserialize, Default, AsRefStr, EnumIter, PartialEq, Eq, Clone, Display,
)]
pub enum CompareMode {
    #[strum(serialize = "Bigger Than")]
    BiggerThan,
    #[strum(serialize = "Less Than")]
    LessThan,
    #[strum(serialize = "Equal")]
    #[default]
    Equal,
    #[strum(serialize = "Not Equal")]
    NotEqual,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CompareNode {
    pub mode: CompareMode,
}

impl EFNodeFn for CompareNode {
    fn get_name(&self) -> &'static str {
        "Compare"
    }

    fn get_inputs(&self) -> &[(&'static str, crate::node_editor::node_trait::NodeDataType)] {
        return &[
            ("Seq 1", NodeDataType::DecimalSequence),
            ("Seq 2", NodeDataType::DecimalSequence),
        ];
    }

    fn get_outputs(&self) -> &[(&'static str, crate::node_editor::node_trait::NodeDataType)] {
        return &[("Mask", NodeDataType::Mask)];
    }

    fn process_data(
        &self,
        input_args: &[crate::node_editor::node_trait::CheapCloneNodeDataTypeWithValue],
    ) -> Result<Vec<crate::node_editor::node_trait::CheapCloneNodeDataTypeWithValue>> {
        if input_args.len() < 1 {
            return Err(anyhow!("should have 1 input!"));
        }

        if let NodeDataTypeWithValue::DecimalSequence(df0) = &*input_args[0] {
            if let NodeDataTypeWithValue::DecimalSequence(df1) = &*input_args[1] {
                let result = match self.mode {
                    CompareMode::Equal => df0.equal(df1),
                    CompareMode::NotEqual => df0.not_equal(df1),
                    CompareMode::LessThan => df0.lt(df1),
                    CompareMode::BiggerThan => df0.gt(df1),
                };
                return Ok(vec![NodeDataTypeWithValue::Mask(
                    result.iter().map(|x| x.unwrap()).collect(),
                )
                .into_arc()]);
            }
        }
        return Err(anyhow!("Unknown input"));
    }

    fn show_header(
        &mut self,
        node_id: egui_snarl::NodeId,
        _inputs: &[egui_snarl::InPin],
        _outputs: &[egui_snarl::OutPin],
        ui: &mut egui::Ui,
        scale: f32,
    ) -> bool {
        let mut result = false;
        ComboBox::from_id_salt(0)
            .selected_text(self.mode.to_string())
            .show_ui(ui, |ui| {
                for v in CompareMode::iter() {
                    let value = ui.selectable_value(&mut self.mode, v.clone(), v.to_string());
                    if value.changed() {
                        result = true;
                    }
                }
            });
        result
    }

    fn export_data(&self) -> serde_json::Value {
        return serde_json::to_value(self).unwrap();
    }
}
