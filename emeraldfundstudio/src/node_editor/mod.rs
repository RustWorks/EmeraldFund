#![allow(clippy::use_self)]

pub mod node_trait;
pub mod nodes;
pub mod style;

use egui::{Align, Color32, Layout, RichText, Ui};
use egui_snarl::{
    ui::{PinInfo, SnarlViewer},
    InPin, NodeId, OutPin, OutPinId, Snarl,
};
use log::debug;
use node_trait::{EFNodeFNSerialized, NodeDataType};
use strum::{Display, EnumIter};

use crate::{
    consts::NODE_DEFAULT_VALUES,
    node_runners::realtime::{
        clear_cache_from_node_onward, is_node_realtime_executable, run_nodes,
    },
};

const DECIMAL_SEQUENCE_COLOR: Color32 = Color32::from_rgb(0x00, 0xb0, 0x00);
const SIGNAL_COLOR: Color32 = Color32::from_rgb(0x00, 0x00, 0xb0);
const MASK_COLOR: Color32 = Color32::from_rgb(0xb0, 0x00, 0xb0);
const CANDLES_COLOR: Color32 = Color32::from_rgb(0x00, 0xb0, 0xb0);
const DEBUG_COLOR_EXECUTABLE: Color32 = Color32::from_rgba_premultiplied(32, 128, 0, 128);
const DEBUG_COLOR: Color32 = Color32::from_rgba_premultiplied(128, 0, 0, 128);

fn node_row_to_color(node_row: &NodeDataType) -> Color32 {
    match node_row {
        node_trait::NodeDataType::Candles => CANDLES_COLOR,
        node_trait::NodeDataType::Signal => SIGNAL_COLOR,
        node_trait::NodeDataType::DecimalSequence => DECIMAL_SEQUENCE_COLOR,
        node_trait::NodeDataType::Mask => MASK_COLOR,
    }
}

fn get_input_color(snarl: &Snarl<EFNodeFNSerialized<'_>>, pin: &InPin) -> Color32 {
    let node = &snarl[pin.id.node].get_node();
    let (_, input_type) = node
        .get_inputs()
        .get(pin.id.input)
        .expect("output pin not found");
    node_row_to_color(input_type)
}

fn get_output_color(snarl: &Snarl<EFNodeFNSerialized<'_>>, pin: &OutPin) -> Color32 {
    let node = &snarl[pin.id.node].get_node();
    let (_, output_type) = node
        .get_outputs()
        .get(pin.id.output)
        .expect("output pin not found");
    node_row_to_color(output_type)
}

#[derive(Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Display, EnumIter)]
pub enum Comparison {
    #[strum(serialize = "Bigger Than")]
    BiggerThan,
    #[strum(serialize = "Less Than")]
    LessThan,
    #[strum(serialize = "Equal")]
    Equal,
    #[strum(serialize = "Not Equal")]
    NotEqual,
}

#[derive(Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Display, EnumIter)]
pub enum OrderDirection {
    Buy,
    Sell,
}

#[derive(Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Display, EnumIter)]
pub enum OrderType {
    #[strum(serialize = "Limit Maker")]
    LimitMaker,
    Limit,
    Market,
}

pub struct EFViewer;

impl<'a> SnarlViewer<EFNodeFNSerialized<'a>> for EFViewer {
    #[inline]
    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<EFNodeFNSerialized<'_>>) {
        // Make sure this connection is not to the same node
        if from.id.node == to.id.node {
            debug!(
                "Not connecting #{:?} to #{:?} (Same)",
                from.id.node, to.id.node
            );

            return;
        }

        // Make sure this connection does not create a cyclic node graph
        {
            let mut node_ids = vec![];
            node_ids.push(to.id.node);

            while let Some(node_id) = node_ids.pop() {
                for node_id in snarl
                    .out_pin(OutPinId {
                        node: node_id,
                        output: 0,
                    })
                    .remotes
                    .iter()
                    .map(|remote| remote.node)
                {
                    if node_id == from.id.node {
                        debug!(
                            "Not connecting #{:?} to #{:?} (Cyclic)",
                            from.id.node, to.id.node
                        );

                        // We found a cycle
                        return;
                    }

                    node_ids.push(node_id);
                }
            }
        }

        // Enforce the same type by checking the color
        let color_from = get_output_color(snarl, from);
        let color_to = get_input_color(snarl, to);
        if color_from != color_to {
            return;
        }

        for &remote in &to.remotes {
            snarl.disconnect(remote, to.id);
        }

        snarl.connect(from.id, to.id);
        run_nodes(snarl);
    }

    fn title(&mut self, node: &EFNodeFNSerialized<'_>) -> String {
        node.get_node().get_name().to_owned()
    }

    fn inputs(&mut self, node: &EFNodeFNSerialized<'_>) -> usize {
        node.get_node().get_inputs().len()
    }

    fn show_input(
        &mut self,
        pin: &InPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<EFNodeFNSerialized<'a>>,
    ) -> PinInfo {
        let color = get_input_color(snarl, pin);
        let (label, _) = snarl[pin.id.node].get_node().get_inputs()[pin.id.input];
        ui.label(label);
        PinInfo::circle().with_fill(color)
    }

    fn outputs(&mut self, node: &EFNodeFNSerialized<'_>) -> usize {
        node.get_node().get_outputs().len()
    }

    fn show_output(
        &mut self,
        pin: &OutPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<EFNodeFNSerialized<'a>>,
    ) -> PinInfo {
        let color = get_output_color(snarl, pin);
        let (label, _) = snarl[pin.id.node].get_node().get_outputs()[pin.id.output];
        ui.label(label);
        PinInfo::circle().with_fill(color)
    }

    fn has_graph_menu(
        &mut self,
        _pos: egui::Pos2,
        _snarl: &mut Snarl<EFNodeFNSerialized<'_>>,
    ) -> bool {
        true
    }

    fn show_graph_menu(
        &mut self,
        pos: egui::Pos2,
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<EFNodeFNSerialized<'_>>,
    ) {
        ui.label("Add node");

        for node in NODE_DEFAULT_VALUES.keys() {
            if ui.button(*node).clicked() {
                let mut node = EFNodeFNSerialized {
                    loaded_node: None,
                    node_name: (*node).into(),
                    arguments: NODE_DEFAULT_VALUES.get(node).unwrap().clone(),
                };
                node.load_node().expect("Loading node failed");
                snarl.insert_node(pos, node);
                ui.close_menu();
            }
        }
    }

    fn show_header(
        &mut self,
        node_id: NodeId,
        inputs: &[InPin],
        outputs: &[OutPin],
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<EFNodeFNSerialized<'a>>,
    ) {
        ui.set_height(16.0 * scale);
        ui.set_width(128.0 * scale);
        ui.with_layout(
            Layout::top_down(Align::Min).with_cross_align(Align::Center),
            |ui| {
                let node = snarl.get_node(node_id).unwrap();

                #[cfg(debug_assertions)]
                ui.label(
                    RichText::new(format!("{} #{}", self.title(node), node_id.0)).color(
                        if is_node_realtime_executable(snarl, node_id, node) {
                            DEBUG_COLOR_EXECUTABLE
                        } else {
                            DEBUG_COLOR
                        },
                    ),
                );
                #[cfg(not(debug_assertions))]
                ui.label(RichText::new(self.title(node)).color(
                    if is_node_realtime_executable(snarl, node_id, node) {
                        DEBUG_COLOR_EXECUTABLE
                    } else {
                        DEBUG_COLOR
                    },
                ));

                let node = snarl.get_node_mut(node_id).unwrap();
                let changed = node
                    .get_node_mut()
                    .show_header(node_id, inputs, outputs, ui, scale);
                if changed {
                    clear_cache_from_node_onward(snarl, &node_id);
                }
            },
        );
    }
}
