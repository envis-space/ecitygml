use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::transportation::values::{
    SquareClassValue, SquareFunctionValue, SquareUsageValue,
};
use crate::model::transportation::{
    AbstractTransportationSpace, AsAbstractTransportationSpace, AsAbstractTransportationSpaceMut,
};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct Square {
    pub(crate) abstract_transportation_space: AbstractTransportationSpace,
    class: Option<SquareClassValue>,
    functions: Vec<SquareFunctionValue>,
    usages: Vec<SquareUsageValue>,
}

impl Square {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_transportation_space(AbstractTransportationSpace::new(id))
    }

    pub fn from_abstract_transportation_space(
        abstract_transportation_space: AbstractTransportationSpace,
    ) -> Self {
        Self {
            abstract_transportation_space,
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
        }
    }

    pub fn class(&self) -> Option<&SquareClassValue> {
        self.class.as_ref()
    }

    pub fn set_class(&mut self, class: SquareClassValue) {
        self.class = Some(class);
    }

    pub fn set_class_opt(&mut self, class: Option<SquareClassValue>) {
        self.class = class;
    }

    pub fn clear_class(&mut self) {
        self.class = None;
    }

    pub fn functions(&self) -> &[SquareFunctionValue] {
        &self.functions
    }

    pub fn functions_mut(&mut self) -> &mut [SquareFunctionValue] {
        &mut self.functions
    }

    pub fn set_functions(&mut self, functions: Vec<SquareFunctionValue>) {
        self.functions = functions;
    }

    pub fn push_function(&mut self, function: SquareFunctionValue) {
        self.functions.push(function);
    }

    pub fn extend_functions(&mut self, functions: impl IntoIterator<Item = SquareFunctionValue>) {
        self.functions.extend(functions);
    }

    pub fn usages(&self) -> &[SquareUsageValue] {
        &self.usages
    }

    pub fn usages_mut(&mut self) -> &mut [SquareUsageValue] {
        &mut self.usages
    }

    pub fn set_usages(&mut self, usages: Vec<SquareUsageValue>) {
        self.usages = usages;
    }

    pub fn push_usage(&mut self, usage: SquareUsageValue) {
        self.usages.push(usage);
    }

    pub fn extend_usages(&mut self, usages: impl IntoIterator<Item = SquareUsageValue>) {
        self.usages.extend(usages);
    }
}

impl AsAbstractTransportationSpace for Square {
    fn abstract_transportation_space(&self) -> &AbstractTransportationSpace {
        &self.abstract_transportation_space
    }
}

impl AsAbstractTransportationSpaceMut for Square {
    fn abstract_transportation_space_mut(&mut self) -> &mut AbstractTransportationSpace {
        &mut self.abstract_transportation_space
    }
}

crate::impl_abstract_transportation_space_traits!(Square);
crate::impl_abstract_transportation_space_mut_traits!(Square);
crate::impl_has_feature_type!(Square, Square);

impl IterFeatures for Square {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into()).chain(self.abstract_transportation_space.iter_features()),
        )
    }
}

impl ForEachFeatureMut for Square {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_transportation_space.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for Square {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_transportation_space.compute_envelope()
    }
}

impl ApplyTransform for Square {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_transportation_space.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_transportation_space.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_transportation_space.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_transportation_space.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_transportation_space.apply_scale(scale);
    }
}
