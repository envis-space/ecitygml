use crate::model::construction::{
    AbstractConstructiveElement, AsAbstractConstructiveElement, AsAbstractConstructiveElementMut,
};
use crate::model::core::AsAbstractFeatureMut;
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use egml::model::base::Id;
use egml::model::basic::Code;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingConstructiveElement {
    pub(crate) abstract_constructive_element: AbstractConstructiveElement,
    class: Option<Code>,
    functions: Vec<Code>,
    usages: Vec<Code>,
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

    pub fn class(&self) -> Option<&Code> {
        self.class.as_ref()
    }

    pub fn set_class(&mut self, class: Option<Code>) {
        self.class = class;
    }

    pub fn functions(&self) -> &[Code] {
        &self.functions
    }

    pub fn set_functions(&mut self, functions: Vec<Code>) {
        self.functions = functions;
    }

    pub fn push_function(&mut self, function: Code) {
        self.functions.push(function);
    }

    pub fn extend_functions(&mut self, functions: impl IntoIterator<Item = Code>) {
        self.functions.extend(functions);
    }

    pub fn usages(&self) -> &[Code] {
        &self.usages
    }

    pub fn set_usages(&mut self, usages: Vec<Code>) {
        self.usages = usages;
    }

    pub fn push_usage(&mut self, usage: Code) {
        self.usages.push(usage);
    }

    pub fn extend_usages(&mut self, usages: impl IntoIterator<Item = Code>) {
        self.usages.extend(usages);
    }
}

impl BuildingConstructiveElement {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_constructive_element.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
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
