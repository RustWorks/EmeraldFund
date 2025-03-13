use std::any::Any;

use crate::node_editor::node_trait::{EFNodeFn, NodeDataType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PreviewNode {
    pub(crate) output_color: [u8; 3],
}

impl EFNodeFn for PreviewNode {
    fn get_name(&self) -> &'static str {
        "PreviewNode"
    }

    fn get_inputs(&self) -> &[(&'static str, crate::node_editor::node_trait::NodeDataType)] {
        return &[("Input", NodeDataType::DecimalSequence)];
    }

    fn get_outputs(&self) -> &[(&'static str, crate::node_editor::node_trait::NodeDataType)] {
        return &[];
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
        let response = ui.color_edit_button_srgb(&mut self.output_color);
        if response.changed() {
            result = true;
        }
        result
    }

    fn export_data(&self) -> serde_json::Value {
        return serde_json::to_value(self).unwrap();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
