use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::construction::values::{DoorClassValue, DoorFunctionValue, DoorUsageValue};
use crate::model::construction::{
    AbstractFillingElement, AsAbstractFillingElement, AsAbstractFillingElementMut,
};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct Door {
    pub(crate) abstract_filling_element: AbstractFillingElement,
    class: Option<DoorClassValue>,
    functions: Vec<DoorFunctionValue>,
    usages: Vec<DoorUsageValue>,
}

impl Door {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_filling_element(AbstractFillingElement::new(id))
    }

    pub fn from_abstract_filling_element(abstract_filling_element: AbstractFillingElement) -> Self {
        Self {
            abstract_filling_element,
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
        }
    }
}

impl AsAbstractFillingElement for Door {
    fn abstract_filling_element(&self) -> &AbstractFillingElement {
        &self.abstract_filling_element
    }
}

impl AsAbstractFillingElementMut for Door {
    fn abstract_filling_element_mut(&mut self) -> &mut AbstractFillingElement {
        &mut self.abstract_filling_element
    }
}

crate::impl_abstract_filling_element_traits!(Door);
crate::impl_abstract_filling_element_mut_traits!(Door);
crate::impl_has_feature_type!(Door, Door);

impl IterFeatures for Door {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(std::iter::once(self.into()).chain(self.abstract_filling_element.iter_features()))
    }
}

impl ForEachFeatureMut for Door {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_filling_element.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for Door {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_filling_element.compute_envelope()
    }
}

impl ApplyTransform for Door {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_filling_element.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_filling_element.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_filling_element.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_filling_element.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_filling_element.apply_scale(scale);
    }
}
