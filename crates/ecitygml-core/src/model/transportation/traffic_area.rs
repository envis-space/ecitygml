use crate::model::core::{
    AbstractThematicSurface, AsAbstractThematicSurface, AsAbstractThematicSurfaceMut,
    CityObjectKind, CityObjectRef,
};
use crate::operations::{Visitable, Visitor};
use egml::model::basic::Code;

#[derive(Debug, Clone, PartialEq)]
pub struct TrafficArea {
    pub(crate) abstract_thematic_surface: AbstractThematicSurface,
    pub(crate) class: Option<Code>,
    pub(crate) functions: Vec<Code>,
    pub(crate) usages: Vec<Code>,
    pub(crate) surface_material: Option<Code>,
}

impl TrafficArea {
    pub fn new(abstract_thematic_surface: AbstractThematicSurface) -> Self {
        Self {
            abstract_thematic_surface,
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
            surface_material: None,
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::TrafficArea(self))
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

    pub fn surface_material(&self) -> Option<&Code> {
        self.surface_material.as_ref()
    }

    pub fn set_surface_material(&mut self, surface_material: Option<Code>) {
        self.surface_material = surface_material;
    }
}

impl AsAbstractThematicSurface for TrafficArea {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface {
        &self.abstract_thematic_surface
    }
}

impl AsAbstractThematicSurfaceMut for TrafficArea {
    fn abstract_thematic_surface_mut(&mut self) -> &mut AbstractThematicSurface {
        &mut self.abstract_thematic_surface
    }
}

crate::impl_abstract_thematic_surface_traits!(TrafficArea);

impl From<TrafficArea> for CityObjectKind {
    fn from(item: TrafficArea) -> Self {
        CityObjectKind::TrafficArea(item)
    }
}

impl Visitable for TrafficArea {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_traffic_area(self);
    }
}
