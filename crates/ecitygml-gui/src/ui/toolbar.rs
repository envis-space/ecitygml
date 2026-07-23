#![allow(deprecated)]
use crate::app::AppState;
use std::path::PathBuf;
use three_d::egui::{self, Context};

pub struct ToolbarAction {
    pub open_file: Option<PathBuf>,
    pub recenter: bool,
}

pub fn show(ctx: &Context, state: &AppState) -> ToolbarAction {
    let mut action = ToolbarAction {
        open_file: None,
        recenter: false,
    };

    egui::Panel::top("toolbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Open File…").clicked()
                && let Some(path) = rfd::FileDialog::new()
                    .add_filter("CityGML", &["gml", "xml", "zst", "gz"])
                    .pick_file()
            {
                action.open_file = Some(path);
            }

            ui.separator();

            if state.is_loading() {
                ui.spinner();
                ui.label("Loading…");
            } else if state.graphics.is_some() {
                let count = state.city_object_count;
                ui.label(format!("{count} city objects"));
            }

            if let crate::app::LoadState::Error(msg) = &state.load_state {
                ui.colored_label(egui::Color32::RED, format!("Error: {msg}"));
            }

            // Placed last so it only claims the leftover space to the right of
            // the widgets above, rather than the whole row's width.
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add_enabled(state.graphics.is_some(), egui::Button::new("Recenter View"))
                    .on_hover_text("Reset the camera to frame the whole model")
                    .clicked()
                {
                    action.recenter = true;
                }
            });
        });
    });

    action
}
