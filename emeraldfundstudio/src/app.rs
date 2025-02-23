use eframe::egui::{self};
use egui_snarl::Snarl;

use crate::{
    candles::chart::candlestick_chart,
    node_editor::{node_trait::EFNodeFNSerialized, style::default_style, EFViewer},
    node_runners::realtime::run_nodes,
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct EmeraldFundStudioApp<'a> {
    snarl: Snarl<EFNodeFNSerialized<'a>>,
}

impl Default for EmeraldFundStudioApp<'_> {
    fn default() -> Self {
        Self {
            snarl: Snarl::new(),
        }
    }
}

impl EmeraldFundStudioApp<'_> {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            let mut result: Self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            for node in result.snarl.nodes_mut() {
                node.load_node().unwrap();
            }
            run_nodes(&result.snarl);
            return result;
        }

        Default::default()
    }
}

impl eframe::App for EmeraldFundStudioApp<'_> {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        for node in self.snarl.nodes_mut() {
            node.save_node();
        }

        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        if ui.button("Reset").clicked() {
                            self.snarl = Default::default();
                            ctx.memory_mut(|mem| *mem = Default::default());
                        }
                    });
                    ui.add_space(16.0);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Emerald Fund Studio");
            egui::TopBottomPanel::top("top")
                .resizable(true)
                .min_height(256.0)
                .show(ctx, |ui| {
                    candlestick_chart(ui, &self.snarl);
                });
            egui::CentralPanel::default().show(ctx, |ui| {
                self.snarl
                    .show(&mut EFViewer, &default_style(), "snarl", ui);
            });
        });
    }
}
