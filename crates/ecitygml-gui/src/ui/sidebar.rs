#![allow(deprecated)]
use crate::app::AppState;
use ecitygml_core::model::common::{FeatureType, HasFeatureType};
use ecitygml_core::model::core::AsAbstractFeature;
use egml::model::base::Id;
use std::collections::BTreeMap;
use three_d::egui::{self, Context};

pub struct SidebarAction {
    pub selected: Option<Id>,
    pub toggled_type: Option<FeatureType>,
    /// `Some(true)` to show all feature types, `Some(false)` to hide all.
    pub select_all: Option<bool>,
}

pub fn show(ctx: &Context, state: &AppState) -> SidebarAction {
    let mut action = SidebarAction {
        selected: None,
        toggled_type: None,
        select_all: None,
    };

    egui::Panel::left("sidebar")
        .default_size(240.0)
        .show(ctx, |ui| {
            ui.heading("Features");
            ui.separator();

            let Some(graphics) = &state.graphics else {
                ui.label("No file loaded.");
                return;
            };

            ui.horizontal(|ui| {
                if ui.button("Show All").clicked() {
                    action.select_all = Some(true);
                }
                if ui.button("Hide All").clicked() {
                    action.select_all = Some(false);
                }
            });
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                let grouped = group_by_type(&graphics.city_model_arena);
                for (ft, ids) in &grouped {
                    let visible = !state.hidden_feature_types.contains(ft);
                    let header = format!("{ft} ({})", ids.len());

                    let collapsing_id = ui.id().with(ft.to_string());
                    let collapsing =
                        egui::collapsing_header::CollapsingState::load_with_default_open(
                            ui.ctx(),
                            collapsing_id,
                            false,
                        );

                    let header_response = collapsing.show_header(ui, |ui| {
                        let mut checked = visible;
                        if ui.checkbox(&mut checked, "").changed() {
                            action.toggled_type = Some(*ft);
                        }
                        let label = if visible {
                            egui::RichText::new(&header)
                        } else {
                            egui::RichText::new(&header).weak()
                        };
                        ui.label(label);
                    });

                    header_response.body(|ui| {
                        if !visible {
                            ui.weak("(hidden)");
                            return;
                        }
                        for id in ids {
                            let selected = state.selected_id.as_ref() == Some(id);
                            let resp = ui
                                .add(egui::Label::new(id.to_string()).sense(egui::Sense::click()));
                            if selected {
                                ui.painter().rect_filled(
                                    resp.rect,
                                    2.0,
                                    egui::Color32::from_rgba_premultiplied(80, 130, 200, 60),
                                );
                            }
                            if resp.clicked() {
                                action.selected = Some(id.clone());
                            }
                        }
                    });
                }
            });
        });

    action
}

fn group_by_type(store: &ecitygml_core::arena::CityModelArena) -> BTreeMap<FeatureType, Vec<Id>> {
    let mut map: BTreeMap<FeatureType, Vec<Id>> = BTreeMap::new();
    for city_object in store.iter_city_objects() {
        map.entry(city_object.feature_type())
            .or_default()
            .push(city_object.feature_id().clone());
    }
    map
}
