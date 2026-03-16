use crate::model::construction::{
    AbstractConstructiveElement, AsAbstractConstructiveElement, AsAbstractConstructiveElementMut,
};
use crate::model::core::{
    AsAbstractOccupiedSpace, AsAbstractOccupiedSpaceMut, CityObjectKind, CityObjectRef,
};
use crate::operations::{Visitable, Visitor};
use egml::model::basic::Code;

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

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::BuildingConstructiveElement(self))
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

impl From<BuildingConstructiveElement> for CityObjectKind {
    fn from(item: BuildingConstructiveElement) -> Self {
        CityObjectKind::BuildingConstructiveElement(item)
    }
}

impl Visitable for BuildingConstructiveElement {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_building_constructive_element(self);
    }
}
