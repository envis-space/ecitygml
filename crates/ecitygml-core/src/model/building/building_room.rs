use crate::impl_abstract_unoccupied_space_mut_traits;
use crate::impl_abstract_unoccupied_space_traits;
use crate::model::building::values::{
    BuildingRoomClassValue, BuildingRoomFunctionValue, BuildingRoomUsageValue,
};
use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractUnoccupiedSpace, AsAbstractUnoccupiedSpace, AsAbstractUnoccupiedSpaceMut,
};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingRoom {
    pub(crate) abstract_unoccupied_space: AbstractUnoccupiedSpace,
    class: Option<BuildingRoomClassValue>,
    functions: Vec<BuildingRoomFunctionValue>,
    usages: Vec<BuildingRoomUsageValue>,
}

impl BuildingRoom {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_unoccupied_space(AbstractUnoccupiedSpace::new(id))
    }

    pub fn from_abstract_unoccupied_space(
        abstract_unoccupied_space: AbstractUnoccupiedSpace,
    ) -> Self {
        BuildingRoom {
            abstract_unoccupied_space,
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
        }
    }

    pub fn class(&self) -> Option<&BuildingRoomClassValue> {
        self.class.as_ref()
    }

    pub fn set_class(&mut self, class: BuildingRoomClassValue) {
        self.class = Some(class);
    }

    pub fn set_class_opt(&mut self, class: Option<BuildingRoomClassValue>) {
        self.class = class;
    }

    pub fn clear_class(&mut self) {
        self.class = None;
    }

    pub fn functions(&self) -> &[BuildingRoomFunctionValue] {
        &self.functions
    }

    pub fn functions_mut(&mut self) -> &mut [BuildingRoomFunctionValue] {
        &mut self.functions
    }

    pub fn set_functions(&mut self, functions: Vec<BuildingRoomFunctionValue>) {
        self.functions = functions;
    }

    pub fn push_function(&mut self, function: BuildingRoomFunctionValue) {
        self.functions.push(function);
    }

    pub fn extend_functions(
        &mut self,
        functions: impl IntoIterator<Item = BuildingRoomFunctionValue>,
    ) {
        self.functions.extend(functions);
    }

    pub fn usages(&self) -> &[BuildingRoomUsageValue] {
        &self.usages
    }

    pub fn usages_mut(&mut self) -> &mut [BuildingRoomUsageValue] {
        &mut self.usages
    }

    pub fn set_usages(&mut self, usages: Vec<BuildingRoomUsageValue>) {
        self.usages = usages;
    }

    pub fn push_usage(&mut self, usage: BuildingRoomUsageValue) {
        self.usages.push(usage);
    }

    pub fn extend_usages(&mut self, usages: impl IntoIterator<Item = BuildingRoomUsageValue>) {
        self.usages.extend(usages);
    }
}

impl AsAbstractUnoccupiedSpace for BuildingRoom {
    fn abstract_unoccupied_space(&self) -> &AbstractUnoccupiedSpace {
        &self.abstract_unoccupied_space
    }
}

impl AsAbstractUnoccupiedSpaceMut for BuildingRoom {
    fn abstract_unoccupied_space_mut(&mut self) -> &mut AbstractUnoccupiedSpace {
        &mut self.abstract_unoccupied_space
    }
}

impl_abstract_unoccupied_space_traits!(BuildingRoom);
impl_abstract_unoccupied_space_mut_traits!(BuildingRoom);
crate::impl_has_feature_type!(BuildingRoom, BuildingRoom);

impl IterFeatures for BuildingRoom {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(std::iter::once(self.into()).chain(self.abstract_unoccupied_space.iter_features()))
    }
}

impl ForEachFeatureMut for BuildingRoom {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_unoccupied_space.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for BuildingRoom {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_unoccupied_space.compute_envelope()
    }
}

impl ApplyTransform for BuildingRoom {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_unoccupied_space.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_unoccupied_space.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_unoccupied_space.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_unoccupied_space.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_unoccupied_space.apply_scale(scale);
    }
}
