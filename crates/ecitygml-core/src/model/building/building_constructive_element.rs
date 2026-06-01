use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::construction::{
    AbstractConstructiveElement, AsAbstractConstructiveElement, AsAbstractConstructiveElementMut,
};
use crate::model::core::AsAbstractFeatureMut;
use egml::model::basic::Code;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingConstructiveElement {
    pub abstract_constructive_element: AbstractConstructiveElement,
    pub(crate) class: Option<Code>,
    pub(crate) functions: Vec<Code>,
    pub(crate) usages: Vec<Code>,
}

impl BuildingConstructiveElement {
    pub fn new(abstract_constructive_element: AbstractConstructiveElement) -> Self {
        Self {
            abstract_constructive_element,
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_constructive_element.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_constructive_element.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_constructive_element.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_constructive_element.apply_transform(m);
    }

    pub fn class(&self) -> &Option<Code> {
        &self.class
    }

    pub fn set_class(&mut self, class: Option<Code>) {
        self.class = class;
    }

    pub fn functions(&self) -> &Vec<Code> {
        &self.functions
    }

    pub fn set_functions(&mut self, functions: Vec<Code>) {
        self.functions = functions;
    }

    pub fn usages(&self) -> &Vec<Code> {
        &self.usages
    }

    pub fn set_usages(&mut self, usages: Vec<Code>) {
        self.usages = usages;
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

impl<'a> From<&'a BuildingConstructiveElement> for FeatureRef<'a> {
    fn from(item: &'a BuildingConstructiveElement) -> Self {
        FeatureRef::BuildingConstructiveElement(item)
    }
}

impl<'a> From<&'a mut BuildingConstructiveElement> for FeatureRefMut<'a> {
    fn from(item: &'a mut BuildingConstructiveElement) -> Self {
        FeatureRefMut::BuildingConstructiveElement(item)
    }
}
