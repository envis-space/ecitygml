use crate::model::core::{
    AbstractOccupiedSpace, AsAbstractOccupiedSpace, AsAbstractOccupiedSpaceMut, CityObjectKind,
    CityObjectRef,
};
use crate::operations::{Visitable, Visitor};

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingConstructiveElement {
    pub abstract_occupied_space: AbstractOccupiedSpace,
}

impl BuildingConstructiveElement {
    pub fn new(abstract_occupied_space: AbstractOccupiedSpace) -> Self {
        Self {
            abstract_occupied_space,
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::BuildingConstructiveElement(self))
    }
}

impl AsAbstractOccupiedSpace for BuildingConstructiveElement {
    fn abstract_occupied_space(&self) -> &AbstractOccupiedSpace {
        &self.abstract_occupied_space
    }
}

impl AsAbstractOccupiedSpaceMut for BuildingConstructiveElement {
    fn abstract_occupied_space_mut(&mut self) -> &mut AbstractOccupiedSpace {
        &mut self.abstract_occupied_space
    }
}

crate::impl_abstract_occupied_space_traits!(BuildingConstructiveElement);

impl From<BuildingConstructiveElement> for CityObjectKind {
    fn from(item: BuildingConstructiveElement) -> Self {
        CityObjectKind::BuildingConstructiveElement(item)
    }
}

impl Visitable for BuildingConstructiveElement {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_building_constructive_element(self);
    }
}
