use crate::model::core::{
    AbstractOccupiedSpace, AsAbstractOccupiedSpace, AsAbstractOccupiedSpaceMut, CityObjectKind,
    CityObjectRef,
};
use crate::operations::{Visitable, Visitor};
use egml::model::basic::Measure;

#[derive(Debug, Clone, PartialEq)]
pub struct SolitaryVegetationObject {
    pub(crate) abstract_occupied_space: AbstractOccupiedSpace,
    pub(crate) height: Option<Measure>,
    pub(crate) trunk_diameter: Option<Measure>,
}

impl SolitaryVegetationObject {
    pub fn new(abstract_occupied_space: AbstractOccupiedSpace) -> Self {
        Self {
            abstract_occupied_space,
            height: None,
            trunk_diameter: None,
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::SolitaryVegetationObject(self))
    }

    pub fn height(&self) -> Option<&Measure> {
        self.height.as_ref()
    }

    pub fn trunk_diameter(&self) -> Option<&Measure> {
        self.trunk_diameter.as_ref()
    }

    pub fn set_height(&mut self, height: Option<Measure>) {
        self.height = height;
    }

    pub fn set_trunk_diameter(&mut self, trunk_diameter: Option<Measure>) {
        self.trunk_diameter = trunk_diameter;
    }
}

impl AsAbstractOccupiedSpace for SolitaryVegetationObject {
    fn abstract_occupied_space(&self) -> &AbstractOccupiedSpace {
        &self.abstract_occupied_space
    }
}

impl AsAbstractOccupiedSpaceMut for SolitaryVegetationObject {
    fn abstract_occupied_space_mut(&mut self) -> &mut AbstractOccupiedSpace {
        &mut self.abstract_occupied_space
    }
}

crate::impl_abstract_occupied_space_traits!(SolitaryVegetationObject);

impl From<SolitaryVegetationObject> for CityObjectKind {
    fn from(item: SolitaryVegetationObject) -> Self {
        CityObjectKind::SolitaryVegetationObject(item)
    }
}

impl Visitable for SolitaryVegetationObject {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_solitary_vegetation_object(self);
    }
}
