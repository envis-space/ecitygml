use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractUnoccupiedSpace, AsAbstractUnoccupiedSpace, AsAbstractUnoccupiedSpaceMut,
};
use crate::model::transportation::GranularityValue;
use crate::model::transportation::values::{
    AuxiliaryTrafficSpaceClassValue, AuxiliaryTrafficSpaceFunctionValue,
    AuxiliaryTrafficSpaceUsageValue,
};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct AuxiliaryTrafficSpace {
    pub(crate) abstract_unoccupied_space: AbstractUnoccupiedSpace,
    class: Option<AuxiliaryTrafficSpaceClassValue>,
    functions: Vec<AuxiliaryTrafficSpaceFunctionValue>,
    usages: Vec<AuxiliaryTrafficSpaceUsageValue>,
    granularity: GranularityValue,
}

impl AuxiliaryTrafficSpace {
    pub fn new(id: Id, granularity: GranularityValue) -> Self {
        Self::from_abstract_unoccupied_space(AbstractUnoccupiedSpace::new(id), granularity)
    }

    pub fn from_abstract_unoccupied_space(
        abstract_unoccupied_space: AbstractUnoccupiedSpace,
        granularity: GranularityValue,
    ) -> Self {
        Self {
            abstract_unoccupied_space,
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
            granularity,
        }
    }

    pub fn class(&self) -> Option<&AuxiliaryTrafficSpaceClassValue> {
        self.class.as_ref()
    }

    pub fn set_class(&mut self, class: AuxiliaryTrafficSpaceClassValue) {
        self.class = Some(class);
    }

    pub fn set_class_opt(&mut self, class: Option<AuxiliaryTrafficSpaceClassValue>) {
        self.class = class;
    }

    pub fn clear_class(&mut self) {
        self.class = None;
    }

    pub fn functions(&self) -> &[AuxiliaryTrafficSpaceFunctionValue] {
        &self.functions
    }

    pub fn functions_mut(&mut self) -> &mut [AuxiliaryTrafficSpaceFunctionValue] {
        &mut self.functions
    }

    pub fn set_functions(&mut self, functions: Vec<AuxiliaryTrafficSpaceFunctionValue>) {
        self.functions = functions;
    }

    pub fn push_function(&mut self, function: AuxiliaryTrafficSpaceFunctionValue) {
        self.functions.push(function);
    }

    pub fn extend_functions(
        &mut self,
        functions: impl IntoIterator<Item = AuxiliaryTrafficSpaceFunctionValue>,
    ) {
        self.functions.extend(functions);
    }

    pub fn usages(&self) -> &[AuxiliaryTrafficSpaceUsageValue] {
        &self.usages
    }

    pub fn usages_mut(&mut self) -> &mut [AuxiliaryTrafficSpaceUsageValue] {
        &mut self.usages
    }

    pub fn set_usages(&mut self, usages: Vec<AuxiliaryTrafficSpaceUsageValue>) {
        self.usages = usages;
    }

    pub fn push_usage(&mut self, usage: AuxiliaryTrafficSpaceUsageValue) {
        self.usages.push(usage);
    }

    pub fn extend_usages(
        &mut self,
        usages: impl IntoIterator<Item = AuxiliaryTrafficSpaceUsageValue>,
    ) {
        self.usages.extend(usages);
    }

    pub fn granularity(&self) -> &GranularityValue {
        &self.granularity
    }

    pub fn set_granularity(&mut self, granularity: GranularityValue) {
        self.granularity = granularity;
    }
}

impl AsAbstractUnoccupiedSpace for AuxiliaryTrafficSpace {
    fn abstract_unoccupied_space(&self) -> &AbstractUnoccupiedSpace {
        &self.abstract_unoccupied_space
    }
}

impl AsAbstractUnoccupiedSpaceMut for AuxiliaryTrafficSpace {
    fn abstract_unoccupied_space_mut(&mut self) -> &mut AbstractUnoccupiedSpace {
        &mut self.abstract_unoccupied_space
    }
}

crate::impl_abstract_unoccupied_space_traits!(AuxiliaryTrafficSpace);
crate::impl_abstract_unoccupied_space_mut_traits!(AuxiliaryTrafficSpace);
crate::impl_has_feature_type!(AuxiliaryTrafficSpace, AuxiliaryTrafficSpace);

impl IterFeatures for AuxiliaryTrafficSpace {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(std::iter::once(self.into()).chain(self.abstract_unoccupied_space.iter_features()))
    }
}

impl ForEachFeatureMut for AuxiliaryTrafficSpace {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_unoccupied_space.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for AuxiliaryTrafficSpace {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_unoccupied_space.compute_envelope()
    }
}

impl ApplyTransform for AuxiliaryTrafficSpace {
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
