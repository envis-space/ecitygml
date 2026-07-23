use crate::model::building::building_constructive_element_property::BuildingConstructiveElementProperty;
use crate::model::building::values::{
    BuildingClassValue, BuildingFunctionValue, BuildingUsageValue, RoofTypeValue,
};
use crate::model::building::{
    AbstractBuildingSubdivisionProperty, BuildingInstallationProperty, BuildingRoomProperty,
};
use crate::model::common::{CodeListValue, ForEachFeatureMut, IterFeatures};
use crate::model::construction::{
    AbstractConstruction, AsAbstractConstruction, AsAbstractConstructionMut,
};

use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractBuilding {
    pub(crate) abstract_construction: AbstractConstruction,
    class: Option<BuildingClassValue>,
    functions: Vec<BuildingFunctionValue>,
    usages: Vec<BuildingUsageValue>,
    roof_type: Option<RoofTypeValue>,
    storeys_above_ground: Option<i64>,
    storeys_below_ground: Option<i64>,
    building_constructive_elements: Vec<BuildingConstructiveElementProperty>,
    building_installations: Vec<BuildingInstallationProperty>,
    building_rooms: Vec<BuildingRoomProperty>,
    building_subdivisions: Vec<AbstractBuildingSubdivisionProperty>,
}

impl AbstractBuilding {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_construction(AbstractConstruction::new(id))
    }

    pub fn from_abstract_construction(abstract_construction: AbstractConstruction) -> Self {
        Self {
            abstract_construction,
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
            roof_type: None,
            storeys_above_ground: None,
            storeys_below_ground: None,
            building_constructive_elements: Vec::new(),
            building_installations: Vec::new(),
            building_rooms: Vec::new(),
            building_subdivisions: Vec::new(),
        }
    }
}

pub trait AsAbstractBuilding: AsAbstractConstruction {
    fn abstract_building(&self) -> &AbstractBuilding;

    fn class(&self) -> Option<&BuildingClassValue> {
        self.abstract_building().class.as_ref()
    }

    /// Interprets `class` against a specific code list, if its codeSpace
    /// matches (or `class` has no codeSpace at all).
    fn class_as<T: CodeListValue>(&self) -> Option<T> {
        self.class()?.interpret::<T>()
    }

    fn functions(&self) -> &[BuildingFunctionValue] {
        &self.abstract_building().functions
    }

    /// Interprets `functions` against a specific code list, yielding only
    /// the entries whose codeSpace matches (or have no codeSpace at all).
    fn functions_as<T: CodeListValue + 'static>(&self) -> impl Iterator<Item = T> + '_ {
        self.functions()
            .iter()
            .filter_map(BuildingFunctionValue::interpret::<T>)
    }

    fn usages(&self) -> &[BuildingUsageValue] {
        &self.abstract_building().usages
    }

    /// Interprets `usages` against a specific code list, yielding only
    /// the entries whose codeSpace matches (or have no codeSpace at all).
    fn usages_as<T: CodeListValue + 'static>(&self) -> impl Iterator<Item = T> + '_ {
        self.usages()
            .iter()
            .filter_map(BuildingUsageValue::interpret::<T>)
    }

    fn roof_type(&self) -> Option<&RoofTypeValue> {
        self.abstract_building().roof_type.as_ref()
    }

    /// Interprets `roofType` against a specific code list, if its
    /// codeSpace matches (or `roofType` has no codeSpace at all).
    fn roof_type_as<T: CodeListValue>(&self) -> Option<T> {
        self.roof_type()?.interpret::<T>()
    }

    fn storeys_above_ground(&self) -> Option<i64> {
        self.abstract_building().storeys_above_ground
    }

    fn storeys_below_ground(&self) -> Option<i64> {
        self.abstract_building().storeys_below_ground
    }

    fn building_constructive_elements(&self) -> &[BuildingConstructiveElementProperty] {
        &self.abstract_building().building_constructive_elements
    }

    fn building_installations(&self) -> &[BuildingInstallationProperty] {
        &self.abstract_building().building_installations
    }

    fn building_rooms(&self) -> &[BuildingRoomProperty] {
        &self.abstract_building().building_rooms
    }

    fn building_subdivisions(&self) -> &[AbstractBuildingSubdivisionProperty] {
        &self.abstract_building().building_subdivisions
    }
}

pub trait AsAbstractBuildingMut: AsAbstractConstructionMut + AsAbstractBuilding {
    fn abstract_building_mut(&mut self) -> &mut AbstractBuilding;

    fn set_class(&mut self, class: BuildingClassValue) {
        self.abstract_building_mut().class = Some(class);
    }

    fn set_class_opt(&mut self, class: Option<BuildingClassValue>) {
        self.abstract_building_mut().class = class;
    }

    fn clear_class(&mut self) {
        self.abstract_building_mut().class = None;
    }

    /// Sets `class`, encoding it with `T`'s code list codeSpace.
    fn set_class_as<T: CodeListValue>(&mut self, value: &T) {
        self.abstract_building_mut().class = Some(BuildingClassValue::from_value(value));
    }

    fn functions_mut(&mut self) -> &mut [BuildingFunctionValue] {
        &mut self.abstract_building_mut().functions
    }

    fn set_functions(&mut self, functions: Vec<BuildingFunctionValue>) {
        self.abstract_building_mut().functions = functions;
    }

    fn push_function(&mut self, function: BuildingFunctionValue) {
        self.abstract_building_mut().functions.push(function);
    }

    fn extend_functions(&mut self, functions: impl IntoIterator<Item = BuildingFunctionValue>) {
        self.abstract_building_mut().functions.extend(functions);
    }

    /// Appends a function, encoding it with `T`'s code list codeSpace.
    fn push_function_as<T: CodeListValue>(&mut self, value: &T) {
        self.abstract_building_mut()
            .functions
            .push(BuildingFunctionValue::from_value(value));
    }

    fn usages_mut(&mut self) -> &mut [BuildingUsageValue] {
        &mut self.abstract_building_mut().usages
    }

    fn set_usages(&mut self, usages: Vec<BuildingUsageValue>) {
        self.abstract_building_mut().usages = usages;
    }

    fn push_usage(&mut self, usage: BuildingUsageValue) {
        self.abstract_building_mut().usages.push(usage);
    }

    fn extend_usages(&mut self, usages: impl IntoIterator<Item = BuildingUsageValue>) {
        self.abstract_building_mut().usages.extend(usages);
    }

    /// Appends a usage, encoding it with `T`'s code list codeSpace.
    fn push_usage_as<T: CodeListValue>(&mut self, value: &T) {
        self.abstract_building_mut()
            .usages
            .push(BuildingUsageValue::from_value(value));
    }

    fn set_roof_type(&mut self, roof_type: RoofTypeValue) {
        self.abstract_building_mut().roof_type = Some(roof_type);
    }

    fn set_roof_type_opt(&mut self, roof_type: Option<RoofTypeValue>) {
        self.abstract_building_mut().roof_type = roof_type;
    }

    fn clear_roof_type(&mut self) {
        self.abstract_building_mut().roof_type = None;
    }

    /// Sets `roofType`, encoding it with `T`'s code list codeSpace.
    fn set_roof_type_as<T: CodeListValue>(&mut self, value: &T) {
        self.abstract_building_mut().roof_type = Some(RoofTypeValue::from_value(value));
    }

    fn set_storeys_above_ground(&mut self, storeys_above_ground: i64) {
        self.abstract_building_mut().storeys_above_ground = Some(storeys_above_ground);
    }

    fn set_storeys_above_ground_opt(&mut self, storeys_above_ground: Option<i64>) {
        self.abstract_building_mut().storeys_above_ground = storeys_above_ground;
    }

    fn clear_storeys_above_ground(&mut self) {
        self.abstract_building_mut().storeys_above_ground = None;
    }

    fn set_storeys_below_ground(&mut self, storeys_below_ground: i64) {
        self.abstract_building_mut().storeys_below_ground = Some(storeys_below_ground);
    }

    fn set_storeys_below_ground_opt(&mut self, storeys_below_ground: Option<i64>) {
        self.abstract_building_mut().storeys_below_ground = storeys_below_ground;
    }

    fn clear_storeys_below_ground(&mut self) {
        self.abstract_building_mut().storeys_below_ground = None;
    }

    fn building_constructive_elements_mut(&mut self) -> &mut [BuildingConstructiveElementProperty] {
        &mut self.abstract_building_mut().building_constructive_elements
    }

    fn set_building_constructive_elements(
        &mut self,
        building_constructive_elements: Vec<BuildingConstructiveElementProperty>,
    ) {
        self.abstract_building_mut().building_constructive_elements =
            building_constructive_elements;
    }

    fn push_building_constructive_element(&mut self, element: BuildingConstructiveElementProperty) {
        self.abstract_building_mut()
            .building_constructive_elements
            .push(element);
    }

    fn extend_building_constructive_elements(
        &mut self,
        elements: impl IntoIterator<Item = BuildingConstructiveElementProperty>,
    ) {
        self.abstract_building_mut()
            .building_constructive_elements
            .extend(elements);
    }

    fn building_installations_mut(&mut self) -> &mut [BuildingInstallationProperty] {
        &mut self.abstract_building_mut().building_installations
    }

    fn set_building_installations(
        &mut self,
        building_installations: Vec<BuildingInstallationProperty>,
    ) {
        self.abstract_building_mut().building_installations = building_installations;
    }

    fn push_building_installation(&mut self, installation: BuildingInstallationProperty) {
        self.abstract_building_mut()
            .building_installations
            .push(installation);
    }

    fn extend_building_installations(
        &mut self,
        installations: impl IntoIterator<Item = BuildingInstallationProperty>,
    ) {
        self.abstract_building_mut()
            .building_installations
            .extend(installations);
    }

    fn building_rooms_mut(&mut self) -> &mut [BuildingRoomProperty] {
        &mut self.abstract_building_mut().building_rooms
    }

    fn set_building_rooms(&mut self, building_rooms: Vec<BuildingRoomProperty>) {
        self.abstract_building_mut().building_rooms = building_rooms;
    }

    fn push_building_room(&mut self, room: BuildingRoomProperty) {
        self.abstract_building_mut().building_rooms.push(room);
    }

    fn extend_building_rooms(&mut self, rooms: impl IntoIterator<Item = BuildingRoomProperty>) {
        self.abstract_building_mut().building_rooms.extend(rooms);
    }

    fn building_subdivisions_mut(&mut self) -> &mut [AbstractBuildingSubdivisionProperty] {
        &mut self.abstract_building_mut().building_subdivisions
    }

    fn set_building_subdivisions(
        &mut self,
        building_subdivisions: Vec<AbstractBuildingSubdivisionProperty>,
    ) {
        self.abstract_building_mut().building_subdivisions = building_subdivisions;
    }

    fn push_building_subdivision(&mut self, subdivision: AbstractBuildingSubdivisionProperty) {
        self.abstract_building_mut()
            .building_subdivisions
            .push(subdivision);
    }

    fn extend_building_subdivisions(
        &mut self,
        subdivisions: impl IntoIterator<Item = AbstractBuildingSubdivisionProperty>,
    ) {
        self.abstract_building_mut()
            .building_subdivisions
            .extend(subdivisions);
    }
}

impl AsAbstractBuilding for AbstractBuilding {
    fn abstract_building(&self) -> &AbstractBuilding {
        self
    }
}

impl AsAbstractBuildingMut for AbstractBuilding {
    fn abstract_building_mut(&mut self) -> &mut AbstractBuilding {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_building_traits {
    ($type:ty) => {
        $crate::impl_abstract_construction_traits!($type);

        impl $crate::model::construction::AsAbstractConstruction for $type {
            fn abstract_construction(&self) -> &$crate::model::construction::AbstractConstruction {
                &<$type as $crate::model::building::AsAbstractBuilding>::abstract_building(self)
                    .abstract_construction
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_building_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_construction_mut_traits!($type);

        impl $crate::model::construction::AsAbstractConstructionMut for $type {
            fn abstract_construction_mut(
                &mut self,
            ) -> &mut $crate::model::construction::AbstractConstruction {
                &mut <$type as $crate::model::building::AsAbstractBuildingMut>::abstract_building_mut(self).abstract_construction
            }
        }
    };
}

impl_abstract_building_traits!(AbstractBuilding);
impl_abstract_building_mut_traits!(AbstractBuilding);

impl IterFeatures for AbstractBuilding {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            self.abstract_construction
                .iter_features()
                .chain(
                    self.building_constructive_elements
                        .iter()
                        .filter_map(|x| x.object())
                        .flat_map(|x| x.iter_features()),
                )
                .chain(
                    self.building_installations
                        .iter()
                        .filter_map(|x| x.object())
                        .flat_map(|x| x.iter_features()),
                )
                .chain(
                    self.building_rooms
                        .iter()
                        .filter_map(|x| x.object())
                        .flat_map(|x| x.iter_features()),
                )
                .chain(
                    self.building_subdivisions
                        .iter()
                        .filter_map(|x| x.object())
                        .flat_map(|x| x.iter_features()),
                ),
        )
    }
}

impl ForEachFeatureMut for AbstractBuilding {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_construction.for_each_feature_mut(f);

        for prop in &mut self.building_constructive_elements {
            if let Some(x) = prop.object_mut() {
                x.for_each_feature_mut(f);
            }
        }
        for prop in &mut self.building_installations {
            if let Some(x) = prop.object_mut() {
                x.for_each_feature_mut(f);
            }
        }
        for prop in &mut self.building_rooms {
            if let Some(x) = prop.object_mut() {
                x.for_each_feature_mut(f);
            }
        }
        for prop in &mut self.building_subdivisions {
            if let Some(x) = prop.object_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }
}

impl ComputeEnvelope for AbstractBuilding {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_construction.compute_envelope()
    }
}

impl ApplyTransform for AbstractBuilding {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_construction.apply_transform(m);

        for prop in &mut self.building_constructive_elements {
            if let Some(x) = prop.object_mut() {
                x.apply_transform(m);
            }
        }
        for prop in &mut self.building_installations {
            if let Some(x) = prop.object_mut() {
                x.apply_transform(m);
            }
        }
        for prop in &mut self.building_rooms {
            if let Some(x) = prop.object_mut() {
                x.apply_transform(m);
            }
        }
        for prop in &mut self.building_subdivisions {
            if let Some(x) = prop.object_mut() {
                x.apply_transform(m);
            }
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_construction.apply_isometry(isometry);

        for prop in &mut self.building_constructive_elements {
            if let Some(x) = prop.object_mut() {
                x.apply_isometry(isometry);
            }
        }
        for prop in &mut self.building_installations {
            if let Some(x) = prop.object_mut() {
                x.apply_isometry(isometry);
            }
        }
        for prop in &mut self.building_rooms {
            if let Some(x) = prop.object_mut() {
                x.apply_isometry(isometry);
            }
        }
        for prop in &mut self.building_subdivisions {
            if let Some(x) = prop.object_mut() {
                x.apply_isometry(isometry);
            }
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_construction.apply_translation(vector);

        for prop in &mut self.building_constructive_elements {
            if let Some(x) = prop.object_mut() {
                x.apply_translation(vector);
            }
        }
        for prop in &mut self.building_installations {
            if let Some(x) = prop.object_mut() {
                x.apply_translation(vector);
            }
        }
        for prop in &mut self.building_rooms {
            if let Some(x) = prop.object_mut() {
                x.apply_translation(vector);
            }
        }
        for prop in &mut self.building_subdivisions {
            if let Some(x) = prop.object_mut() {
                x.apply_translation(vector);
            }
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_construction.apply_rotation(rotation);

        for prop in &mut self.building_constructive_elements {
            if let Some(x) = prop.object_mut() {
                x.apply_rotation(rotation);
            }
        }
        for prop in &mut self.building_installations {
            if let Some(x) = prop.object_mut() {
                x.apply_rotation(rotation);
            }
        }
        for prop in &mut self.building_rooms {
            if let Some(x) = prop.object_mut() {
                x.apply_rotation(rotation);
            }
        }
        for prop in &mut self.building_subdivisions {
            if let Some(x) = prop.object_mut() {
                x.apply_rotation(rotation);
            }
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_construction.apply_scale(scale);

        for prop in &mut self.building_constructive_elements {
            if let Some(x) = prop.object_mut() {
                x.apply_scale(scale);
            }
        }
        for prop in &mut self.building_installations {
            if let Some(x) = prop.object_mut() {
                x.apply_scale(scale);
            }
        }
        for prop in &mut self.building_rooms {
            if let Some(x) = prop.object_mut() {
                x.apply_scale(scale);
            }
        }
        for prop in &mut self.building_subdivisions {
            if let Some(x) = prop.object_mut() {
                x.apply_scale(scale);
            }
        }
    }
}
