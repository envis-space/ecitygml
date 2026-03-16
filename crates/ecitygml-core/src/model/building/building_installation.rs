use crate::model::construction::{
    AbstractInstallation, AsAbstractInstallation, AsAbstractInstallationMut,
};
use crate::model::core::{CityObjectKind, CityObjectRef};
use crate::operations::{Visitable, Visitor};
use egml::model::basic::Code;

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingInstallation {
    pub abstract_installation: AbstractInstallation,
    pub(crate) class: Option<Code>,
    pub(crate) functions: Vec<Code>,
    pub(crate) usages: Vec<Code>,
}

impl BuildingInstallation {
    pub fn new(abstract_installation: AbstractInstallation) -> Self {
        Self {
            abstract_installation,
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
        std::iter::once(CityObjectRef::BuildingInstallation(self))
    }
}

impl AsAbstractInstallation for BuildingInstallation {
    fn abstract_installation(&self) -> &AbstractInstallation {
        &self.abstract_installation
    }
}

impl AsAbstractInstallationMut for BuildingInstallation {
    fn abstract_installation_mut(&mut self) -> &mut AbstractInstallation {
        &mut self.abstract_installation
    }
}

crate::impl_abstract_installation_traits!(BuildingInstallation);

impl From<BuildingInstallation> for CityObjectKind {
    fn from(item: BuildingInstallation) -> Self {
        CityObjectKind::BuildingInstallation(item)
    }
}

impl Visitable for BuildingInstallation {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_building_installation(self);
    }
}
