use crate::model::core::{
    AbstractThematicSurface, AsAbstractThematicSurface, AsAbstractThematicSurfaceMut,
    CityObjectKind, CityObjectRef,
};
use crate::operations::{Visitable, Visitor};
use egml::model::basic::Code;

#[derive(Debug, Clone, PartialEq)]
pub struct TrafficArea {
    pub(crate) abstract_thematic_surface: AbstractThematicSurface,
    pub(crate) function: Vec<Code>,
    pub(crate) usage: Vec<Code>,
    pub(crate) surface_material: Option<Code>,
}

impl TrafficArea {
    pub fn new(abstract_thematic_surface: AbstractThematicSurface) -> Self {
        Self {
            abstract_thematic_surface,
            function: Vec::new(),
            usage: Vec::new(),
            surface_material: None,
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::TrafficArea(self))
    }

    pub fn function(&self) -> &Vec<Code> {
        &self.function
    }

    pub fn set_function(&mut self, function: Vec<Code>) {
        self.function = function;
    }

    pub fn usage(&self) -> &Vec<Code> {
        &self.usage
    }

    pub fn set_usage(&mut self, usage: Vec<Code>) {
        self.usage = usage;
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
