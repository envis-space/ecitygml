use crate::model::building::BuildingConstructiveElementProperty;
use crate::model::building::values::{
    BuildingSubdivisionClassValue, BuildingSubdivisionFunctionValue, BuildingSubdivisionUsageValue,
};
use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{AbstractLogicalSpace, AsAbstractLogicalSpace, AsAbstractLogicalSpaceMut};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractBuildingSubdivision {
    pub(crate) abstract_logical_space: AbstractLogicalSpace,
    class: Option<BuildingSubdivisionClassValue>,
    functions: Vec<BuildingSubdivisionFunctionValue>,
    usages: Vec<BuildingSubdivisionUsageValue>,
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

pub trait AsAbstractBuildingSubdivision: AsAbstractLogicalSpace {
    fn abstract_building_subdivision(&self) -> &AbstractBuildingSubdivision;

    fn class(&self) -> Option<&BuildingSubdivisionClassValue> {
        self.abstract_building_subdivision().class.as_ref()
    }

    fn functions(&self) -> &[BuildingSubdivisionFunctionValue] {
        &self.abstract_building_subdivision().functions
    }

    fn usages(&self) -> &[BuildingSubdivisionUsageValue] {
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

    fn set_class(&mut self, class: BuildingSubdivisionClassValue) {
        self.abstract_building_subdivision_mut().class = Some(class);
    }

    fn set_class_opt(&mut self, class: Option<BuildingSubdivisionClassValue>) {
        self.abstract_building_subdivision_mut().class = class;
    }

    fn clear_class(&mut self) {
        self.abstract_building_subdivision_mut().class = None;
    }

    fn functions_mut(&mut self) -> &mut [BuildingSubdivisionFunctionValue] {
        &mut self.abstract_building_subdivision_mut().functions
    }

    fn set_functions(&mut self, functions: Vec<BuildingSubdivisionFunctionValue>) {
        self.abstract_building_subdivision_mut().functions = functions;
    }

    fn push_function(&mut self, function: BuildingSubdivisionFunctionValue) {
        self.abstract_building_subdivision_mut()
            .functions
            .push(function);
    }

    fn extend_functions(
        &mut self,
        functions: impl IntoIterator<Item = BuildingSubdivisionFunctionValue>,
    ) {
        self.abstract_building_subdivision_mut()
            .functions
            .extend(functions);
    }

    fn usages_mut(&mut self) -> &mut [BuildingSubdivisionUsageValue] {
        &mut self.abstract_building_subdivision_mut().usages
    }

    fn set_usages(&mut self, usages: Vec<BuildingSubdivisionUsageValue>) {
        self.abstract_building_subdivision_mut().usages = usages;
    }

    fn push_usage(&mut self, usage: BuildingSubdivisionUsageValue) {
        self.abstract_building_subdivision_mut().usages.push(usage);
    }

    fn extend_usages(&mut self, usages: impl IntoIterator<Item = BuildingSubdivisionUsageValue>) {
        self.abstract_building_subdivision_mut()
            .usages
            .extend(usages);
    }

    fn building_constructive_elements_mut(&mut self) -> &mut [BuildingConstructiveElementProperty] {
        &mut self
            .abstract_building_subdivision_mut()
            .building_constructive_elements
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
                &<$type as $crate::model::building::AsAbstractBuildingSubdivision>::abstract_building_subdivision(self).abstract_logical_space
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
                &mut <$type as $crate::model::building::AsAbstractBuildingSubdivisionMut>::abstract_building_subdivision_mut(self).abstract_logical_space
            }
        }
    };
}

impl_abstract_building_subdivision_traits!(AbstractBuildingSubdivision);
impl_abstract_building_subdivision_mut_traits!(AbstractBuildingSubdivision);

impl IterFeatures for AbstractBuildingSubdivision {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            self.abstract_logical_space.iter_features().chain(
                self.building_constructive_elements
                    .iter()
                    .filter_map(|x| x.object())
                    .flat_map(|x| x.iter_features()),
            ),
        )
    }
}

impl ForEachFeatureMut for AbstractBuildingSubdivision {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_logical_space.for_each_feature_mut(f);
        for prop in &mut self.building_constructive_elements {
            if let Some(x) = prop.object_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }
}

impl ComputeEnvelope for AbstractBuildingSubdivision {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_logical_space.compute_envelope()
    }
}

impl ApplyTransform for AbstractBuildingSubdivision {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_logical_space.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_logical_space.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_logical_space.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_logical_space.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_logical_space.apply_scale(scale);
    }
}
