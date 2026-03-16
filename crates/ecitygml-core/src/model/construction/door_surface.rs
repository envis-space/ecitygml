use crate::model::construction::{
    AbstractFillingSurface, AsAbstractFillingSurface, AsAbstractFillingSurfaceMut,
};
use crate::model::core::{CityObjectKind, CityObjectRef};
use crate::operations::{Visitable, Visitor};

#[derive(Debug, Clone, PartialEq)]
pub struct DoorSurface {
    pub abstract_filling_surface: AbstractFillingSurface,
}

impl DoorSurface {
    pub fn new(abstract_filling_surface: AbstractFillingSurface) -> Self {
        Self {
            abstract_filling_surface,
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::DoorSurface(self))
    }
}

impl AsAbstractFillingSurface for DoorSurface {
    fn abstract_filling_surface(&self) -> &AbstractFillingSurface {
        &self.abstract_filling_surface
    }
}

impl AsAbstractFillingSurfaceMut for DoorSurface {
    fn abstract_filling_surface_mut(&mut self) -> &mut AbstractFillingSurface {
        &mut self.abstract_filling_surface
    }
}

crate::impl_abstract_filling_surface_traits!(DoorSurface);

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
