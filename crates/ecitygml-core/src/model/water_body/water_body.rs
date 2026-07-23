use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::{AbstractFeatureKindRef, AbstractFeatureKindRefMut};
use crate::model::core::{
    AbstractOccupiedSpace, AsAbstractOccupiedSpace, AsAbstractOccupiedSpaceMut,
};
use crate::model::water_body::values::{
    WaterBodyClassValue, WaterBodyFunctionValue, WaterBodyUsageValue,
};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct WaterBody {
    pub(crate) abstract_occupied_space: AbstractOccupiedSpace,
    class: Option<WaterBodyClassValue>,
    functions: Vec<WaterBodyFunctionValue>,
    usages: Vec<WaterBodyUsageValue>,
}

impl WaterBody {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_occupied_space(AbstractOccupiedSpace::new(id))
    }

    pub fn from_abstract_occupied_space(abstract_occupied_space: AbstractOccupiedSpace) -> Self {
        Self {
            abstract_occupied_space,
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
        }
    }

    pub fn class(&self) -> Option<&WaterBodyClassValue> {
        self.class.as_ref()
    }

    pub fn set_class(&mut self, class: WaterBodyClassValue) {
        self.class = Some(class);
    }

    pub fn set_class_opt(&mut self, class: Option<WaterBodyClassValue>) {
        self.class = class;
    }

    pub fn clear_class(&mut self) {
        self.class = None;
    }

    pub fn functions(&self) -> &[WaterBodyFunctionValue] {
        &self.functions
    }

    pub fn functions_mut(&mut self) -> &mut [WaterBodyFunctionValue] {
        &mut self.functions
    }

    pub fn set_functions(&mut self, functions: Vec<WaterBodyFunctionValue>) {
        self.functions = functions;
    }

    pub fn push_function(&mut self, function: WaterBodyFunctionValue) {
        self.functions.push(function);
    }

    pub fn extend_functions(
        &mut self,
        functions: impl IntoIterator<Item = WaterBodyFunctionValue>,
    ) {
        self.functions.extend(functions);
    }

    pub fn usages(&self) -> &[WaterBodyUsageValue] {
        &self.usages
    }

    pub fn usages_mut(&mut self) -> &mut [WaterBodyUsageValue] {
        &mut self.usages
    }

    pub fn set_usages(&mut self, usages: Vec<WaterBodyUsageValue>) {
        self.usages = usages;
    }

    pub fn push_usage(&mut self, usage: WaterBodyUsageValue) {
        self.usages.push(usage);
    }

    pub fn extend_usages(&mut self, usages: impl IntoIterator<Item = WaterBodyUsageValue>) {
        self.usages.extend(usages);
    }
}

impl AsAbstractOccupiedSpace for WaterBody {
    fn abstract_occupied_space(&self) -> &AbstractOccupiedSpace {
        &self.abstract_occupied_space
    }
}

impl AsAbstractOccupiedSpaceMut for WaterBody {
    fn abstract_occupied_space_mut(&mut self) -> &mut AbstractOccupiedSpace {
        &mut self.abstract_occupied_space
    }
}

crate::impl_abstract_occupied_space_traits!(WaterBody);
crate::impl_abstract_occupied_space_mut_traits!(WaterBody);
crate::impl_has_feature_type!(WaterBody, WaterBody);

impl IterFeatures for WaterBody {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(std::iter::once(self.into()).chain(self.abstract_occupied_space.iter_features()))
    }
}

impl ForEachFeatureMut for WaterBody {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_occupied_space.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for WaterBody {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_occupied_space.compute_envelope()
    }
}

impl ApplyTransform for WaterBody {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_occupied_space.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_occupied_space.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_occupied_space.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_occupied_space.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_occupied_space.apply_scale(scale);
    }
}
