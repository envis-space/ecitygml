#![allow(deprecated)]

use crate::app::AppState;
use ecitygml_core::model::common::{HasFeatureType, LevelOfDetail};
use ecitygml_core::model::core::refs::{
    AbstractCityObjectKindRef, AbstractPhysicalSpaceKindRef, AbstractSpaceBoundaryKindRef,
    AbstractSpaceKindRef,
};
use ecitygml_core::model::core::{
    AsAbstractCityObject, AsAbstractOccupiedSpace, AsAbstractSpace, AsAbstractThematicSurface,
};
use ecitygml_core::model::generics::GenericAttributeKind;
use egml::model::feature::AsAbstractFeature;
use std::collections::HashSet;
use three_d::egui::{self, Context};

pub fn show(ctx: &Context, state: &AppState) {
    egui::Panel::right("inspector")
        .default_size(280.0)
        .show(ctx, |ui| {
            ui.heading("Inspector");
            ui.separator();

            let (Some(graphics), Some(id)) = (&state.graphics, &state.selected_id) else {
                ui.label("Nothing selected.");
                return;
            };

            let Some(city_object) = graphics.city_model_arena.get_city_object_by_id(id) else {
                ui.label("Object not found.");
                return;
            };

            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("inspector_grid")
                    .num_columns(2)
                    .striped(true)
                    .spacing([8.0, 4.0])
                    .show(ui, |ui| {
                        row(ui, "ID", &id.to_string());
                        row(ui, "Type", &city_object.feature_type().to_string());

                        if let Some(env) = city_object.bounded_by().and_then(|x| x.envelope()) {
                            let lo = env.lower_corner();
                            let hi = env.upper_corner();
                            row(ui, "Min X", &format!("{:.2}", lo.x()));
                            row(ui, "Min Y", &format!("{:.2}", lo.y()));
                            row(ui, "Min Z", &format!("{:.2}", lo.z()));
                            row(ui, "Max X", &format!("{:.2}", hi.x()));
                            row(ui, "Max Y", &format!("{:.2}", hi.y()));
                            row(ui, "Max Z", &format!("{:.2}", hi.z()));
                        }

                        let lods_solid: Vec<String> = solid_lods(city_object)
                            .into_iter()
                            .map(|l| format!("{l:?}"))
                            .collect();
                        let lods_ms: Vec<String> = multi_surface_lods(city_object)
                            .into_iter()
                            .map(|l| format!("{l:?}"))
                            .collect();

                        if !lods_solid.is_empty() {
                            row(ui, "Solid LODs", &lods_solid.join(", "));
                        }
                        if !lods_ms.is_empty() {
                            row(ui, "Surface LODs", &lods_ms.join(", "));
                        }
                    });

                let attrs = city_object.generic_attributes();
                if !attrs.is_empty() {
                    ui.separator();
                    ui.strong("Generic Attributes");
                    ui.add_space(2.0);
                    for attr in attrs {
                        show_attribute(ui, attr);
                    }
                }
            });
        });
}

/// LODs at which a city object has a solid representation.
fn solid_lods(city_object: AbstractCityObjectKindRef<'_>) -> HashSet<LevelOfDetail> {
    match city_object {
        AbstractCityObjectKindRef::AbstractSpaceKind(
            AbstractSpaceKindRef::AbstractPhysicalSpaceKind(
                AbstractPhysicalSpaceKindRef::AbstractOccupiedSpaceKind(x),
            ),
        ) => x
            .abstract_occupied_space()
            .solids_by_lod()
            .into_keys()
            .collect(),
        AbstractCityObjectKindRef::AbstractSpaceKind(x) => {
            x.abstract_space().solids_by_lod().into_keys().collect()
        }
        _ => HashSet::new(),
    }
}

/// LODs at which a city object has a multi-surface representation.
fn multi_surface_lods(city_object: AbstractCityObjectKindRef<'_>) -> HashSet<LevelOfDetail> {
    match city_object {
        AbstractCityObjectKindRef::AbstractSpaceKind(
            AbstractSpaceKindRef::AbstractPhysicalSpaceKind(
                AbstractPhysicalSpaceKindRef::AbstractOccupiedSpaceKind(x),
            ),
        ) => x
            .abstract_occupied_space()
            .multi_surfaces_by_lod()
            .into_keys()
            .collect(),
        AbstractCityObjectKindRef::AbstractSpaceKind(x) => x
            .abstract_space()
            .multi_surfaces_by_lod()
            .into_keys()
            .collect(),
        AbstractCityObjectKindRef::AbstractSpaceBoundaryKind(
            AbstractSpaceBoundaryKindRef::AbstractThematicSurfaceKind(x),
        ) => x
            .abstract_thematic_surface()
            .multi_surfaces_by_lod()
            .into_keys()
            .collect(),
        _ => HashSet::new(),
    }
}

fn show_attribute(ui: &mut egui::Ui, attr: &GenericAttributeKind) {
    match attr {
        GenericAttributeKind::StringAttribute(a) => {
            attribute_row(ui, &a.name, &a.value);
        }
        GenericAttributeKind::IntAttribute(a) => {
            attribute_row(ui, &a.name, &a.value.to_string());
        }
        GenericAttributeKind::DoubleAttribute(a) => {
            attribute_row(ui, &a.name, &format!("{:.6}", a.value));
        }
        GenericAttributeKind::MeasureAttribute(a) => {
            let display = if a.value.uom.is_empty() {
                format!("{:.6}", a.value.value)
            } else {
                format!("{:.6} {}", a.value.value, a.value.uom)
            };
            attribute_row(ui, &a.name, &display);
        }
        GenericAttributeKind::GenericAttributeSet(set) => {
            egui::CollapsingHeader::new(&set.name)
                .default_open(true)
                .show(ui, |ui| {
                    for child in &set.generic_attributes {
                        show_attribute(ui, child);
                    }
                });
        }
    }
}

fn attribute_row(ui: &mut egui::Ui, name: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(name).strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(value);
        });
    });
}

fn row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.label(egui::RichText::new(label).strong());
    ui.label(value);
    ui.end_row();
}
