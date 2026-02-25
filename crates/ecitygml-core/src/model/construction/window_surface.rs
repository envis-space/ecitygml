use crate::model::core::{
    AbstractThematicSurface, AsAbstractThematicSurface, AsAbstractThematicSurfaceMut, CityObjectRef,
};
use crate::model::core::{AsAbstractFeature, CityObjectKind};
use crate::operations::{Visitable, Visitor};

#[derive(Debug, Clone, PartialEq)]
pub struct WindowSurface {
    pub abstract_thematic_surface: AbstractThematicSurface,
}

impl WindowSurface {
    pub fn new(abstract_thematic_surface: AbstractThematicSurface) -> Self {
        Self {
            abstract_thematic_surface,
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::WindowSurface(self))
    }
}

impl AsAbstractThematicSurface for WindowSurface {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface {
        &self.abstract_thematic_surface
    }
}

impl AsAbstractThematicSurfaceMut for WindowSurface {
    fn abstract_thematic_surface_mut(&mut self) -> &mut AbstractThematicSurface {
        &mut self.abstract_thematic_surface
    }
}

crate::impl_abstract_thematic_surface_traits!(WindowSurface);

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
