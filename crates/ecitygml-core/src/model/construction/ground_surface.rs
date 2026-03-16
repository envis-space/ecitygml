use crate::model::construction::{
    AbstractConstructionSurface, AsAbstractConstructionSurface, AsAbstractConstructionSurfaceMut,
};
use crate::model::core::{CityObjectKind, CityObjectRef};
use crate::operations::{Visitable, Visitor};

#[derive(Debug, Clone, PartialEq)]
pub struct GroundSurface {
    pub abstract_construction_surface: AbstractConstructionSurface,
}

impl GroundSurface {
    pub fn new(abstract_construction_surface: AbstractConstructionSurface) -> Self {
        Self {
            abstract_construction_surface,
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::GroundSurface(self))
    }
}

impl AsAbstractConstructionSurface for GroundSurface {
    fn abstract_construction_surface(&self) -> &AbstractConstructionSurface {
        &self.abstract_construction_surface
    }
}

impl AsAbstractConstructionSurfaceMut for GroundSurface {
    fn abstract_construction_surface_mut(&mut self) -> &mut AbstractConstructionSurface {
        &mut self.abstract_construction_surface
    }
}

crate::impl_abstract_construction_surface_traits!(GroundSurface);

impl From<GroundSurface> for CityObjectKind {
    fn from(item: GroundSurface) -> Self {
        CityObjectKind::GroundSurface(item)
    }
}

impl Visitable for GroundSurface {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_ground_surface(self);
    }
}
