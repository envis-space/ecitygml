use crate::model::core::{
    AbstractThematicSurface, AsAbstractThematicSurface, AsAbstractThematicSurfaceMut,
    CityObjectKind, CityObjectRef,
};
use crate::operations::{Visitable, Visitor};

#[derive(Debug, Clone, PartialEq)]
pub struct DoorSurface {
    pub abstract_thematic_surface: AbstractThematicSurface,
}

impl DoorSurface {
    pub fn new(abstract_thematic_surface: AbstractThematicSurface) -> Self {
        Self {
            abstract_thematic_surface,
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::DoorSurface(self))
    }
}

impl AsAbstractThematicSurface for DoorSurface {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface {
        &self.abstract_thematic_surface
    }
}

impl AsAbstractThematicSurfaceMut for DoorSurface {
    fn abstract_thematic_surface_mut(&mut self) -> &mut AbstractThematicSurface {
        &mut self.abstract_thematic_surface
    }
}

crate::impl_abstract_thematic_surface_traits!(DoorSurface);

impl From<DoorSurface> for CityObjectKind {
    fn from(item: DoorSurface) -> Self {
        CityObjectKind::DoorSurface(item)
    }
}

impl Visitable for DoorSurface {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_door_surface(self);
    }
}
