use crate::model::core::{
    AbstractOccupiedSpace, AsAbstractOccupiedSpace, AsAbstractOccupiedSpaceMut, CityObjectKind,
    CityObjectRef,
};
use crate::operations::{Visitable, Visitor};

#[derive(Debug, Clone, PartialEq)]
pub struct CityFurniture {
    pub abstract_occupied_space: AbstractOccupiedSpace,
}

impl CityFurniture {
    pub fn new(abstract_occupied_space: AbstractOccupiedSpace) -> Self {
        Self {
            abstract_occupied_space,
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::CityFurniture(self))
    }
}

impl AsAbstractOccupiedSpace for CityFurniture {
    fn abstract_occupied_space(&self) -> &AbstractOccupiedSpace {
        &self.abstract_occupied_space
    }
}

impl AsAbstractOccupiedSpaceMut for CityFurniture {
    fn abstract_occupied_space_mut(&mut self) -> &mut AbstractOccupiedSpace {
        &mut self.abstract_occupied_space
    }
}

crate::impl_abstract_occupied_space_traits!(CityFurniture);

impl From<CityFurniture> for CityObjectKind {
    fn from(item: CityFurniture) -> Self {
        CityObjectKind::CityFurniture(item)
    }
}

impl Visitable for CityFurniture {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_city_furniture(self);
    }
}
