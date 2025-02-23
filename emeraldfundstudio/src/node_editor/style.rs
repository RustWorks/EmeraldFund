use egui::CornerRadius;
use egui_snarl::ui::{NodeLayout, PinPlacement, SnarlStyle};

pub const fn default_style() -> SnarlStyle {
    SnarlStyle {
        node_layout: Some(NodeLayout::FlippedSandwich),
        pin_placement: Some(PinPlacement::Edge),
        pin_size: Some(7.0),
        node_frame: Some(egui::Frame {
            inner_margin: egui::Margin::same(8),
            outer_margin: egui::Margin {
                left: 0,
                right: 0,
                top: 0,
                bottom: 4,
            },
            fill: egui::Color32::from_gray(30),
            stroke: egui::Stroke::NONE,
            shadow: egui::Shadow::NONE,
            corner_radius: CornerRadius::same(8),
        }),
        bg_frame: Some(egui::Frame {
            inner_margin: egui::Margin::same(2),
            outer_margin: egui::Margin::ZERO,
            fill: egui::Color32::from_gray(40),
            stroke: egui::Stroke::NONE,
            shadow: egui::Shadow::NONE,
            corner_radius: CornerRadius::same(8),
        }),
        ..SnarlStyle::new()
    }
}
