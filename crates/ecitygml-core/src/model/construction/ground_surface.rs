use crate::model::core::{
    AbstractThematicSurface, AsAbstractThematicSurface, AsAbstractThematicSurfaceMut,
    CityObjectKind, CityObjectRef,
};
use crate::operations::{Visitable, Visitor};

#[derive(Debug, Clone, PartialEq)]
pub struct GroundSurface {
    pub abstract_thematic_surface: AbstractThematicSurface,
}

impl GroundSurface {
    pub fn new(abstract_thematic_surface: AbstractThematicSurface) -> Self {
        Self {
            abstract_thematic_surface,
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::GroundSurface(self))
    }
}

impl AsAbstractThematicSurface for GroundSurface {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface {
        &self.abstract_thematic_surface
    }
}

impl AsAbstractThematicSurfaceMut for GroundSurface {
    fn abstract_thematic_surface_mut(&mut self) -> &mut AbstractThematicSurface {
        &mut self.abstract_thematic_surface
    }
}

crate::impl_abstract_thematic_surface_traits!(GroundSurface);

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
