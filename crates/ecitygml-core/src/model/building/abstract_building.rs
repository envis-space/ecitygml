use crate::model::building::building_constructive_element_property::BuildingConstructiveElementProperty;
use crate::model::building::{
    BuildingInstallationProperty, BuildingRoomProperty, BuildingSubdivisionProperty,
};
use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::construction::{
    AbstractConstruction, AsAbstractConstruction, AsAbstractConstructionMut,
};

use egml::model::basic::Code;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractBuilding {
    pub abstract_construction: AbstractConstruction,
    pub(crate) class: Option<Code>,
    pub(crate) functions: Vec<Code>,
    pub(crate) usages: Vec<Code>,
    pub(crate) roof_type: Option<Code>,
    pub(crate) storeys_above_ground: Option<i64>,
    pub(crate) storeys_below_ground: Option<i64>,
    pub building_constructive_elements: Vec<BuildingConstructiveElementProperty>,
    pub building_installations: Vec<BuildingInstallationProperty>,
    pub building_rooms: Vec<BuildingRoomProperty>,
    pub building_subdivisions: Vec<BuildingSubdivisionProperty>,
}

impl AbstractBuilding {
    pub fn new(abstract_construction: AbstractConstruction) -> Self {
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

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
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

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
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

    fn class(&self) -> &Option<Code> {
        &self.abstract_building().class
    }

    fn functions(&self) -> &Vec<Code> {
        &self.abstract_building().functions
    }

    fn usages(&self) -> &Vec<Code> {
        &self.abstract_building().usages
    }

    fn roof_type(&self) -> &Option<Code> {
        &self.abstract_building().roof_type
    }

    fn storeys_above_ground(&self) -> Option<i64> {
        self.abstract_building().storeys_above_ground
    }

    fn storeys_below_ground(&self) -> Option<i64> {
        self.abstract_building().storeys_below_ground
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

    fn set_usages(&mut self, usages: Vec<Code>) {
        self.abstract_building_mut().usages = usages;
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
