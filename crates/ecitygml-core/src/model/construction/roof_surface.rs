use crate::model::core::{
    AbstractThematicSurface, AsAbstractThematicSurface, AsAbstractThematicSurfaceMut,
    CityObjectKind, CityObjectRef,
};
use crate::operations::{Visitable, Visitor};

#[derive(Debug, Clone, PartialEq)]
pub struct RoofSurface {
    pub abstract_thematic_surface: AbstractThematicSurface,
}

impl RoofSurface {
    pub fn new(abstract_thematic_surface: AbstractThematicSurface) -> Self {
        Self {
            abstract_thematic_surface,
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::RoofSurface(self))
    }
}

impl AsAbstractThematicSurface for RoofSurface {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface {
        &self.abstract_thematic_surface
    }
}

impl AsAbstractThematicSurfaceMut for RoofSurface {
    fn abstract_thematic_surface_mut(&mut self) -> &mut AbstractThematicSurface {
        &mut self.abstract_thematic_surface
    }
}

crate::impl_abstract_thematic_surface_traits!(RoofSurface);

impl From<RoofSurface> for CityObjectKind {
    fn from(item: RoofSurface) -> Self {
        CityObjectKind::RoofSurface(item)
    }
}

impl Visitable for RoofSurface {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_roof_surface(self);
    }
}
