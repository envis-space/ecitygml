use crate::model::building::values::{
    BuildingConstructiveElementClassValue, BuildingConstructiveElementFunctionValue,
    BuildingConstructiveElementUsageValue,
};
use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::construction::{
    AbstractConstructiveElement, AsAbstractConstructiveElement, AsAbstractConstructiveElementMut,
};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingConstructiveElement {
    pub(crate) abstract_constructive_element: AbstractConstructiveElement,
    class: Option<BuildingConstructiveElementClassValue>,
    functions: Vec<BuildingConstructiveElementFunctionValue>,
    usages: Vec<BuildingConstructiveElementUsageValue>,
}

impl BuildingConstructiveElement {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_constructive_element(AbstractConstructiveElement::new(id))
    }

    pub fn from_abstract_constructive_element(
        abstract_constructive_element: AbstractConstructiveElement,
    ) -> Self {
        Self {
            abstract_constructive_element,
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
        }
    }

    pub fn class(&self) -> Option<&BuildingConstructiveElementClassValue> {
        self.class.as_ref()
    }

    pub fn set_class(&mut self, class: BuildingConstructiveElementClassValue) {
        self.class = Some(class);
    }

    pub fn set_class_opt(&mut self, class: Option<BuildingConstructiveElementClassValue>) {
        self.class = class;
    }

    pub fn clear_class(&mut self) {
        self.class = None;
    }

    pub fn functions(&self) -> &[BuildingConstructiveElementFunctionValue] {
        &self.functions
    }

    pub fn functions_mut(&mut self) -> &mut [BuildingConstructiveElementFunctionValue] {
        &mut self.functions
    }

    pub fn set_functions(&mut self, functions: Vec<BuildingConstructiveElementFunctionValue>) {
        self.functions = functions;
    }

    pub fn push_function(&mut self, function: BuildingConstructiveElementFunctionValue) {
        self.functions.push(function);
    }

    pub fn extend_functions(
        &mut self,
        functions: impl IntoIterator<Item = BuildingConstructiveElementFunctionValue>,
    ) {
        self.functions.extend(functions);
    }

    pub fn usages(&self) -> &[BuildingConstructiveElementUsageValue] {
        &self.usages
    }

    pub fn usages_mut(&mut self) -> &mut [BuildingConstructiveElementUsageValue] {
        &mut self.usages
    }

    pub fn set_usages(&mut self, usages: Vec<BuildingConstructiveElementUsageValue>) {
        self.usages = usages;
    }

    pub fn push_usage(&mut self, usage: BuildingConstructiveElementUsageValue) {
        self.usages.push(usage);
    }

    pub fn extend_usages(
        &mut self,
        usages: impl IntoIterator<Item = BuildingConstructiveElementUsageValue>,
    ) {
        self.usages.extend(usages);
    }
}

impl AsAbstractConstructiveElement for BuildingConstructiveElement {
    fn abstract_constructive_element(&self) -> &AbstractConstructiveElement {
        &self.abstract_constructive_element
    }
}

impl AsAbstractConstructiveElementMut for BuildingConstructiveElement {
    fn abstract_constructive_element_mut(&mut self) -> &mut AbstractConstructiveElement {
        &mut self.abstract_constructive_element
    }
}

crate::impl_abstract_constructive_element_traits!(BuildingConstructiveElement);
crate::impl_abstract_constructive_element_mut_traits!(BuildingConstructiveElement);
crate::impl_has_feature_type!(BuildingConstructiveElement, BuildingConstructiveElement);

impl IterFeatures for BuildingConstructiveElement {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into()).chain(self.abstract_constructive_element.iter_features()),
        )
    }
}

impl ForEachFeatureMut for BuildingConstructiveElement {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_constructive_element.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for BuildingConstructiveElement {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_constructive_element.compute_envelope()
    }
}

impl ApplyTransform for BuildingConstructiveElement {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_constructive_element.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_constructive_element.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_constructive_element.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_constructive_element.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_constructive_element.apply_scale(scale);
    }
}
