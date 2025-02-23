use crate::{
    node_editor::node_trait::{EFNodeFn, NodeDataType, NodeDataTypeWithValue},
    traits::IntoArc,
};
use anyhow::{anyhow, Result};
use egui::ComboBox;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, IntoEnumIterator};

#[derive(
    Debug, Serialize, Deserialize, Default, AsRefStr, EnumIter, PartialEq, Eq, Clone, Display,
)]
pub enum ToSignalMode {
    #[default]
    Buy,
    Sell,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ToSignalNode {
    mode: ToSignalMode,
}

impl EFNodeFn for ToSignalNode {
    fn get_name(&self) -> &'static str {
        "ToSignalNode"
    }

    fn get_inputs(&self) -> &[(&'static str, crate::node_editor::node_trait::NodeDataType)] {
        return &[("Mask", NodeDataType::Mask)];
    }

    fn get_outputs(&self) -> &[(&'static str, crate::node_editor::node_trait::NodeDataType)] {
        return &[("Signal", NodeDataType::Signal)];
    }

    fn process_data(
        &self,
        input_args: &[crate::node_editor::node_trait::CheapCloneNodeDataTypeWithValue],
    ) -> Result<Vec<crate::node_editor::node_trait::CheapCloneNodeDataTypeWithValue>> {
        if input_args.len() != 1 {
            return Err(anyhow!("should have 1 inputs!"));
        }

        if let NodeDataTypeWithValue::Mask(mask) = &*input_args[0] {
            let result: Vec<i8> = mask
                .iter()
                .map(|m| {
                    if *m {
                        match self.mode {
                            ToSignalMode::Buy => 1,
                            ToSignalMode::Sell => -1,
                        }
                    } else {
                        0
                    }
                })
                .collect();
            return Ok(vec![NodeDataTypeWithValue::Signal(result).into_arc()]);
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
                for v in ToSignalMode::iter() {
                    let response = ui.selectable_value(&mut self.mode, v.clone(), v.to_string());
                    if response.changed() {
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
