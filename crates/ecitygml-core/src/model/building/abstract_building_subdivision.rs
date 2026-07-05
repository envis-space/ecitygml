use crate::model::building::BuildingConstructiveElementProperty;
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use crate::model::core::{AbstractLogicalSpace, AsAbstractLogicalSpace, AsAbstractLogicalSpaceMut};
use egml::model::base::Id;
use egml::model::basic::Code;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractBuildingSubdivision {
    pub(crate) abstract_logical_space: AbstractLogicalSpace,
    class: Option<Code>,
    functions: Vec<Code>,
    usages: Vec<Code>,
    building_constructive_elements: Vec<BuildingConstructiveElementProperty>,
}

impl AbstractBuildingSubdivision {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_logical_space(AbstractLogicalSpace::new(id))
    }

    pub fn from_abstract_logical_space(abstract_logical_space: AbstractLogicalSpace) -> Self {
        Self {
            abstract_logical_space,
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
            building_constructive_elements: Vec::new(),
        }
    }
}
impl AbstractBuildingSubdivision {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        self.abstract_logical_space.iter_features().chain(
            self.building_constructive_elements
                .iter()
                .filter_map(|x| x.object.as_ref())
                .flat_map(|x| x.iter_features()),
        )
    }
    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
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

    fn class(&self) -> Option<&Code> {
        self.abstract_building_subdivision().class.as_ref()
    }

    fn functions(&self) -> &[Code] {
        &self.abstract_building_subdivision().functions
    }

    fn usages(&self) -> &[Code] {
        &self.abstract_building_subdivision().usages
    }

    fn building_constructive_elements(&self) -> &[BuildingConstructiveElementProperty] {
        &self
            .abstract_building_subdivision()
            .building_constructive_elements
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

    fn push_function(&mut self, function: Code) {
        self.abstract_building_subdivision_mut()
            .functions
            .push(function);
    }

    fn extend_functions(&mut self, functions: impl IntoIterator<Item = Code>) {
        self.abstract_building_subdivision_mut()
            .functions
            .extend(functions);
    }

    fn set_usages(&mut self, usages: Vec<Code>) {
        self.abstract_building_subdivision_mut().usages = usages;
    }

    fn push_usage(&mut self, usage: Code) {
        self.abstract_building_subdivision_mut().usages.push(usage);
    }

    fn extend_usages(&mut self, usages: impl IntoIterator<Item = Code>) {
        self.abstract_building_subdivision_mut()
            .usages
            .extend(usages);
    }

    fn set_building_constructive_elements(
        &mut self,
        building_constructive_elements: Vec<BuildingConstructiveElementProperty>,
    ) {
        self.abstract_building_subdivision_mut()
            .building_constructive_elements = building_constructive_elements;
    }

    fn push_building_constructive_element(&mut self, element: BuildingConstructiveElementProperty) {
        self.abstract_building_subdivision_mut()
            .building_constructive_elements
            .push(element);
    }

    fn extend_building_constructive_elements(
        &mut self,
        elements: impl IntoIterator<Item = BuildingConstructiveElementProperty>,
    ) {
        self.abstract_building_subdivision_mut()
            .building_constructive_elements
            .extend(elements);
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
    };
}

#[macro_export]
macro_rules! impl_abstract_building_subdivision_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_logical_space_mut_traits!($type);

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
impl_abstract_building_subdivision_mut_traits!(AbstractBuildingSubdivision);
