use crate::model::construction::{
    AbstractConstructionSurface, AsAbstractConstructionSurface, AsAbstractConstructionSurfaceMut,
};
use crate::model::core::{
    AsAbstractThematicSurface, AsAbstractThematicSurfaceMut, CityObjectKind, CityObjectRef,
};
use crate::operations::{Visitable, Visitor};

#[derive(Debug, Clone, PartialEq)]
pub struct RoofSurface {
    pub abstract_construction_surface: AbstractConstructionSurface,
}

impl RoofSurface {
    pub fn new(abstract_construction_surface: AbstractConstructionSurface) -> Self {
        Self {
            abstract_construction_surface,
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::RoofSurface(self))
    }
}

impl AsAbstractConstructionSurface for RoofSurface {
    fn abstract_construction_surface(&self) -> &AbstractConstructionSurface {
        &self.abstract_construction_surface
    }
}

impl AsAbstractConstructionSurfaceMut for RoofSurface {
    fn abstract_construction_surface_mut(&mut self) -> &mut AbstractConstructionSurface {
        &mut self.abstract_construction_surface
    }
}

crate::impl_abstract_construction_surface_traits!(RoofSurface);

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
