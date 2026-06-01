use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::construction::{
    AbstractFillingElement, AsAbstractFillingElement, AsAbstractFillingElementMut,
};
use crate::model::core::AsAbstractFeatureMut;
use egml::model::basic::Code;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct Window {
    pub abstract_filling_element: AbstractFillingElement,
    pub(crate) class: Option<Code>,
    pub(crate) functions: Vec<Code>,
    pub(crate) usages: Vec<Code>,
}

impl Window {
    pub fn new(abstract_filling_element: AbstractFillingElement) -> Self {
        Self {
            abstract_filling_element,
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_filling_element.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_filling_element.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_filling_element.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_filling_element.apply_transform(m);
    }
}

impl AsAbstractFillingElement for Window {
    fn abstract_filling_element(&self) -> &AbstractFillingElement {
        &self.abstract_filling_element
    }
}

impl AsAbstractFillingElementMut for Window {
    fn abstract_filling_element_mut(&mut self) -> &mut AbstractFillingElement {
        &mut self.abstract_filling_element
    }
}

crate::impl_abstract_filling_element_traits!(Window);

impl<'a> From<&'a Window> for FeatureRef<'a> {
    fn from(item: &'a Window) -> Self {
        FeatureRef::Window(item)
    }
}

impl<'a> From<&'a mut Window> for FeatureRefMut<'a> {
    fn from(item: &'a mut Window) -> Self {
        FeatureRefMut::Window(item)
    }
}
