use crate::model::building::BuildingConstructiveElementProperty;
use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::core::{AbstractLogicalSpace, AsAbstractLogicalSpace, AsAbstractLogicalSpaceMut};
use egml::model::basic::Code;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractBuildingSubdivision {
    pub abstract_logical_space: AbstractLogicalSpace,
    pub(crate) class: Option<Code>,
    pub(crate) functions: Vec<Code>,
    pub(crate) usages: Vec<Code>,
    pub(crate) building_constructive_elements: Vec<BuildingConstructiveElementProperty>,
}

impl AbstractBuildingSubdivision {
    pub fn new(abstract_logical_space: AbstractLogicalSpace) -> Self {
        Self {
            abstract_logical_space,
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
            building_constructive_elements: Vec::new(),
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        self.abstract_logical_space.iter_features().chain(
            self.building_constructive_elements
                .iter()
                .filter_map(|x| x.object.as_ref())
                .flat_map(|x| x.iter_features()),
        )
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_logical_space.for_each_feature_mut(f);
        for prop in &mut self.building_constructive_elements {
            if let Some(x) = prop.object.as_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_logical_space.compute_envelope()
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_logical_space.apply_transform(m);
    }
}

pub trait AsAbstractBuildingSubdivision: AsAbstractLogicalSpace {
    fn abstract_building_subdivision(&self) -> &AbstractBuildingSubdivision;

    fn class(&self) -> &Option<Code> {
        &self.abstract_building_subdivision().class
    }

    fn functions(&self) -> &Vec<Code> {
        &self.abstract_building_subdivision().functions
    }

    fn usages(&self) -> &Vec<Code> {
        &self.abstract_building_subdivision().usages
    }
}

pub trait AsAbstractBuildingSubdivisionMut:
    AsAbstractLogicalSpaceMut + AsAbstractBuildingSubdivision
{
    fn abstract_building_subdivision_mut(&mut self) -> &mut AbstractBuildingSubdivision;

    fn set_class(&mut self, class: Option<Code>) {
        self.abstract_building_subdivision_mut().class = class;
    }

    fn set_functions(&mut self, functions: Vec<Code>) {
        self.abstract_building_subdivision_mut().functions = functions;
    }

    fn set_usages(&mut self, usages: Vec<Code>) {
        self.abstract_building_subdivision_mut().usages = usages;
    }
}

impl AsAbstractBuildingSubdivision for AbstractBuildingSubdivision {
    fn abstract_building_subdivision(&self) -> &AbstractBuildingSubdivision {
        self
    }
}

impl AsAbstractBuildingSubdivisionMut for AbstractBuildingSubdivision {
    fn abstract_building_subdivision_mut(&mut self) -> &mut AbstractBuildingSubdivision {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_building_subdivision_traits {
    ($type:ty) => {
        $crate::impl_abstract_logical_space_traits!($type);

        impl $crate::model::core::AsAbstractLogicalSpace for $type {
            fn abstract_logical_space(&self) -> &$crate::model::core::AbstractLogicalSpace {
                use $crate::model::building::AsAbstractBuildingSubdivision;
                &self.abstract_building_subdivision().abstract_logical_space
            }
        }

        impl $crate::model::core::AsAbstractLogicalSpaceMut for $type {
            fn abstract_logical_space_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractLogicalSpace {
                use $crate::model::building::AsAbstractBuildingSubdivisionMut;
                &mut self
                    .abstract_building_subdivision_mut()
                    .abstract_logical_space
            }
        }
    };
}

impl_abstract_building_subdivision_traits!(AbstractBuildingSubdivision);
