use crate::model::construction::{
    AbstractFillingElement, AsAbstractFillingElement, AsAbstractFillingElementMut,
};
use crate::model::core::AsAbstractFeatureMut;
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use egml::model::base::Id;
use egml::model::basic::Code;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct Window {
    pub(crate) abstract_filling_element: AbstractFillingElement,
    class: Option<Code>,
    functions: Vec<Code>,
    usages: Vec<Code>,
}

impl Window {
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
impl Window {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_filling_element.iter_features())
    }
    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
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
crate::impl_abstract_filling_element_mut_traits!(Window);
crate::impl_has_feature_type!(Window, Window);
