use crate::model::building::building_constructive_element_property::BuildingConstructiveElementProperty;
use crate::model::building::{
    BuildingInstallationProperty, BuildingRoomProperty, BuildingSubdivisionProperty,
};
use crate::model::construction::{
    AbstractConstruction, AsAbstractConstruction, AsAbstractConstructionMut,
};

use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use egml::model::base::Id;
use egml::model::basic::Code;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractBuilding {
    pub(crate) abstract_construction: AbstractConstruction,
    class: Option<Code>,
    functions: Vec<Code>,
    usages: Vec<Code>,
    roof_type: Option<Code>,
    storeys_above_ground: Option<i64>,
    storeys_below_ground: Option<i64>,
    building_constructive_elements: Vec<BuildingConstructiveElementProperty>,
    building_installations: Vec<BuildingInstallationProperty>,
    building_rooms: Vec<BuildingRoomProperty>,
    building_subdivisions: Vec<BuildingSubdivisionProperty>,
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
impl AbstractBuilding {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        self.abstract_construction
            .iter_features()
            .chain(
                self.building_constructive_elements
                    .iter()
                    .filter_map(|x| x.object.as_ref())
                    .flat_map(|x| x.iter_features()),
            )
            .chain(
                self.building_installations
                    .iter()
                    .filter_map(|x| x.object.as_ref())
                    .flat_map(|x| x.iter_features()),
            )
            .chain(
                self.building_rooms
                    .iter()
                    .filter_map(|x| x.object.as_ref())
                    .flat_map(|x| x.iter_features()),
            )
            .chain(
                self.building_subdivisions
                    .iter()
                    .filter_map(|x| x.object.as_ref())
                    .flat_map(|x| x.iter_features()),
            )
    }
    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_construction.for_each_feature_mut(f);

        for prop in &mut self.building_constructive_elements {
            if let Some(x) = prop.object.as_mut() {
                x.for_each_feature_mut(f);
            }
        }
        for prop in &mut self.building_installations {
            if let Some(x) = prop.object.as_mut() {
                x.for_each_feature_mut(f);
            }
        }
        for prop in &mut self.building_rooms {
            if let Some(x) = prop.object.as_mut() {
                x.for_each_feature_mut(f);
            }
        }
        for prop in &mut self.building_subdivisions {
            if let Some(x) = prop.object.as_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }
    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_construction.compute_envelope()
    }
    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_construction.apply_transform(m);

        for prop in &mut self.building_constructive_elements {
            if let Some(x) = prop.object.as_mut() {
                x.apply_transform(m);
            }
        }
        for prop in &mut self.building_installations {
            if let Some(x) = prop.object.as_mut() {
                x.apply_transform(m);
            }
        }
        for prop in &mut self.building_rooms {
            if let Some(x) = prop.object.as_mut() {
                x.apply_transform(m);
            }
        }
        for prop in &mut self.building_subdivisions {
            if let Some(x) = prop.object.as_mut() {
                x.apply_transform(m);
            }
        }
    }
}

pub trait AsAbstractBuilding: AsAbstractConstruction {
    fn abstract_building(&self) -> &AbstractBuilding;

    fn class(&self) -> Option<&Code> {
        self.abstract_building().class.as_ref()
    }

    fn functions(&self) -> &[Code] {
        &self.abstract_building().functions
    }

    fn usages(&self) -> &[Code] {
        &self.abstract_building().usages
    }

    fn roof_type(&self) -> Option<&Code> {
        self.abstract_building().roof_type.as_ref()
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

    fn building_subdivisions(&self) -> &[BuildingSubdivisionProperty] {
        &self.abstract_building().building_subdivisions
    }
}

pub trait AsAbstractBuildingMut: AsAbstractConstructionMut + AsAbstractBuilding {
    fn abstract_building_mut(&mut self) -> &mut AbstractBuilding;

    fn set_class(&mut self, class: Option<Code>) {
        self.abstract_building_mut().class = class;
    }

    fn set_functions(&mut self, functions: Vec<Code>) {
        self.abstract_building_mut().functions = functions;
    }

    fn push_function(&mut self, function: Code) {
        self.abstract_building_mut().functions.push(function);
    }

    fn extend_functions(&mut self, functions: impl IntoIterator<Item = Code>) {
        self.abstract_building_mut().functions.extend(functions);
    }

    fn set_usages(&mut self, usages: Vec<Code>) {
        self.abstract_building_mut().usages = usages;
    }

    fn push_usage(&mut self, usage: Code) {
        self.abstract_building_mut().usages.push(usage);
    }

    fn extend_usages(&mut self, usages: impl IntoIterator<Item = Code>) {
        self.abstract_building_mut().usages.extend(usages);
    }

    fn set_roof_type(&mut self, roof_type: Option<Code>) {
        self.abstract_building_mut().roof_type = roof_type;
    }

    fn set_storeys_above_ground(&mut self, storeys_above_ground: Option<i64>) {
        self.abstract_building_mut().storeys_above_ground = storeys_above_ground;
    }

    fn set_storeys_below_ground(&mut self, storeys_below_ground: Option<i64>) {
        self.abstract_building_mut().storeys_below_ground = storeys_below_ground;
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

    fn set_building_rooms(&mut self, building_rooms: Vec<BuildingRoomProperty>) {
        self.abstract_building_mut().building_rooms = building_rooms;
    }

    fn push_building_room(&mut self, room: BuildingRoomProperty) {
        self.abstract_building_mut().building_rooms.push(room);
    }

    fn extend_building_rooms(&mut self, rooms: impl IntoIterator<Item = BuildingRoomProperty>) {
        self.abstract_building_mut().building_rooms.extend(rooms);
    }

    fn set_building_subdivisions(
        &mut self,
        building_subdivisions: Vec<BuildingSubdivisionProperty>,
    ) {
        self.abstract_building_mut().building_subdivisions = building_subdivisions;
    }

    fn push_building_subdivision(&mut self, subdivision: BuildingSubdivisionProperty) {
        self.abstract_building_mut()
            .building_subdivisions
            .push(subdivision);
    }

    fn extend_building_subdivisions(
        &mut self,
        subdivisions: impl IntoIterator<Item = BuildingSubdivisionProperty>,
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
                use $crate::model::building::AsAbstractBuilding;
                &self.abstract_building().abstract_construction
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
                use $crate::model::building::AsAbstractBuildingMut;
                &mut self.abstract_building_mut().abstract_construction
            }
        }
    };
}

impl_abstract_building_traits!(AbstractBuilding);
impl_abstract_building_mut_traits!(AbstractBuilding);
