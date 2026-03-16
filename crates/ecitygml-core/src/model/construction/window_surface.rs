use crate::model::construction::{
    AbstractFillingSurface, AsAbstractFillingSurface, AsAbstractFillingSurfaceMut,
};
use crate::model::core::CityObjectKind;
use crate::model::core::CityObjectRef;
use crate::operations::{Visitable, Visitor};

#[derive(Debug, Clone, PartialEq)]
pub struct WindowSurface {
    pub abstract_filling_surface: AbstractFillingSurface,
}

impl WindowSurface {
    pub fn new(abstract_filling_surface: AbstractFillingSurface) -> Self {
        Self {
            abstract_filling_surface,
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::WindowSurface(self))
    }
}

impl AsAbstractFillingSurface for WindowSurface {
    fn abstract_filling_surface(&self) -> &AbstractFillingSurface {
        &self.abstract_filling_surface
    }
}

impl AsAbstractFillingSurfaceMut for WindowSurface {
    fn abstract_filling_surface_mut(&mut self) -> &mut AbstractFillingSurface {
        &mut self.abstract_filling_surface
    }
}

crate::impl_abstract_filling_surface_traits!(WindowSurface);

impl From<WindowSurface> for CityObjectKind {
    fn from(item: WindowSurface) -> Self {
        CityObjectKind::WindowSurface(item)
    }
}

impl Visitable for WindowSurface {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_window_surface(self);
    }
}
