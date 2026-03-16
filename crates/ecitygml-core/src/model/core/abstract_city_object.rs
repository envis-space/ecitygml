use crate::model::building::{
    Building, BuildingConstructiveElement, BuildingInstallation, BuildingRoom, Storey,
};
use crate::model::city_furniture::CityFurniture;
use crate::model::common::CityObjectClass;
use crate::model::construction::{
    DoorSurface, GroundSurface, RoofSurface, WallSurface, WindowSurface,
};
use crate::model::core::{
    AbstractFeature, AbstractFeatureWithLifespan, AsAbstractFeature, AsAbstractFeatureMut,
    AsAbstractFeatureWithLifespan, AsAbstractFeatureWithLifespanMut, AsAbstractOccupiedSpaceMut,
    AsAbstractThematicSurfaceMut, ExternalReference,
};
use crate::model::generics::GenericAttributeKind;
use crate::model::relief::{ReliefFeature, TinRelief};
use crate::model::transportation::{
    AuxiliaryTrafficArea, AuxiliaryTrafficSpace, Intersection, Road, Section, TrafficArea,
    TrafficSpace,
};
use crate::model::vegetation::SolitaryVegetationObject;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractCityObject {
    pub(crate) abstract_feature_with_lifespan: AbstractFeatureWithLifespan,
    pub external_references: Vec<ExternalReference>,
    pub generic_attributes: Vec<GenericAttributeKind>,
}

impl AbstractCityObject {
    pub fn new(abstract_feature_with_lifespan: AbstractFeatureWithLifespan) -> Self {
        Self {
            abstract_feature_with_lifespan,
            external_references: vec![],
            generic_attributes: vec![],
        }
    }
}

pub trait AsAbstractCityObject: AsAbstractFeatureWithLifespan {
    fn abstract_city_object(&self) -> &AbstractCityObject;

    fn generic_attributes(&self) -> &[GenericAttributeKind] {
        &self.abstract_city_object().generic_attributes
    }

    fn external_references(&self) -> &[ExternalReference] {
        &self.abstract_city_object().external_references
    }
}

pub trait AsAbstractCityObjectMut: AsAbstractFeatureWithLifespanMut + AsAbstractCityObject {
    fn abstract_city_object_mut(&mut self) -> &mut AbstractCityObject;

    fn set_generic_attributes(&mut self, generic_attributes: Vec<GenericAttributeKind>) {
        self.abstract_city_object_mut().generic_attributes = generic_attributes;
    }

    fn set_external_references(&mut self, external_references: Vec<ExternalReference>) {
        self.abstract_city_object_mut().external_references = external_references;
    }
}

impl AsAbstractCityObject for AbstractCityObject {
    fn abstract_city_object(&self) -> &AbstractCityObject {
        self
    }
}

impl AsAbstractCityObjectMut for AbstractCityObject {
    fn abstract_city_object_mut(&mut self) -> &mut AbstractCityObject {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_city_object_traits {
    ($type:ty) => {
        $crate::impl_abstract_feature_with_lifespan_traits!($type);

        impl $crate::model::core::AsAbstractFeatureWithLifespan for $type {
            fn abstract_feature_with_lifespan(
                &self,
            ) -> &$crate::model::core::AbstractFeatureWithLifespan {
                use $crate::model::core::AsAbstractCityObject;
                &self.abstract_city_object().abstract_feature_with_lifespan
            }
        }

        impl $crate::model::core::AsAbstractFeatureWithLifespanMut for $type {
            fn abstract_feature_with_lifespan_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractFeatureWithLifespan {
                use $crate::model::core::AsAbstractCityObjectMut;
                &mut self
                    .abstract_city_object_mut()
                    .abstract_feature_with_lifespan
            }
        }
    };
}

impl_abstract_city_object_traits!(AbstractCityObject);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::core::AbstractFeature;
    use egml::model::base::Id;

    #[test]
    fn trait_implementation_macro_test() {
        let abstract_feature = AbstractFeature::new(egml::model::base::Id::generate_uuid_v4());
        let abstract_feature_with_lifespan = AbstractFeatureWithLifespan::new(abstract_feature);
        let abstract_city_object = AbstractCityObject::new(abstract_feature_with_lifespan);
        abstract_city_object.id();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CityObjectKind {
    AuxiliaryTrafficArea(AuxiliaryTrafficArea),
    AuxiliaryTrafficSpace(AuxiliaryTrafficSpace),
    Building(Building),
    BuildingConstructiveElement(BuildingConstructiveElement),
    BuildingRoom(BuildingRoom),
    BuildingInstallation(BuildingInstallation),
    CityFurniture(CityFurniture),
    DoorSurface(DoorSurface),
    GroundSurface(GroundSurface),
    Intersection(Intersection),
    ReliefFeature(ReliefFeature),
    Road(Road),
    RoofSurface(RoofSurface),
    Section(Section),
    SolitaryVegetationObject(SolitaryVegetationObject),
    Storey(Storey),
    TinRelief(TinRelief),
    TrafficArea(TrafficArea),
    TrafficSpace(TrafficSpace),
    WallSurface(WallSurface),
    WindowSurface(WindowSurface),
}

impl AsAbstractCityObject for CityObjectKind {
    fn abstract_city_object(&self) -> &AbstractCityObject {
        match self {
            Self::AuxiliaryTrafficArea(x) => x.abstract_city_object(),
            Self::AuxiliaryTrafficSpace(x) => x.abstract_city_object(),
            Self::Building(x) => x.abstract_city_object(),
            Self::BuildingConstructiveElement(x) => x.abstract_city_object(),
            Self::BuildingInstallation(x) => x.abstract_city_object(),
            Self::BuildingRoom(x) => x.abstract_city_object(),
            Self::CityFurniture(x) => x.abstract_city_object(),
            Self::DoorSurface(x) => x.abstract_city_object(),
            Self::GroundSurface(x) => x.abstract_city_object(),
            Self::Intersection(x) => x.abstract_city_object(),
            Self::ReliefFeature(x) => x.abstract_city_object(),
            Self::Road(x) => x.abstract_city_object(),
            Self::RoofSurface(x) => x.abstract_city_object(),
            Self::Section(x) => x.abstract_city_object(),
            Self::SolitaryVegetationObject(x) => x.abstract_city_object(),
            Self::Storey(x) => x.abstract_city_object(),
            Self::TinRelief(x) => x.abstract_city_object(),
            Self::TrafficArea(x) => x.abstract_city_object(),
            Self::TrafficSpace(x) => x.abstract_city_object(),
            Self::WallSurface(x) => x.abstract_city_object(),
            Self::WindowSurface(x) => x.abstract_city_object(),
        }
    }
}

impl AsAbstractCityObjectMut for CityObjectKind {
    fn abstract_city_object_mut(&mut self) -> &mut AbstractCityObject {
        match self {
            Self::AuxiliaryTrafficArea(x) => x.abstract_city_object_mut(),
            Self::AuxiliaryTrafficSpace(x) => x.abstract_city_object_mut(),
            Self::Building(x) => x.abstract_city_object_mut(),
            Self::BuildingConstructiveElement(x) => x.abstract_city_object_mut(),
            Self::BuildingInstallation(x) => x.abstract_city_object_mut(),
            Self::BuildingRoom(x) => x.abstract_city_object_mut(),
            Self::CityFurniture(x) => x.abstract_city_object_mut(),
            Self::DoorSurface(x) => x.abstract_city_object_mut(),
            Self::GroundSurface(x) => x.abstract_city_object_mut(),
            Self::Intersection(x) => x.abstract_city_object_mut(),
            Self::ReliefFeature(x) => x.abstract_city_object_mut(),
            Self::Road(x) => x.abstract_city_object_mut(),
            Self::RoofSurface(x) => x.abstract_city_object_mut(),
            Self::Section(x) => x.abstract_city_object_mut(),
            Self::SolitaryVegetationObject(x) => x.abstract_city_object_mut(),
            Self::Storey(x) => x.abstract_city_object_mut(),
            Self::TinRelief(x) => x.abstract_city_object_mut(),
            Self::TrafficArea(x) => x.abstract_city_object_mut(),
            Self::TrafficSpace(x) => x.abstract_city_object_mut(),
            Self::WallSurface(x) => x.abstract_city_object_mut(),
            Self::WindowSurface(x) => x.abstract_city_object_mut(),
        }
    }
}

impl_abstract_city_object_traits!(CityObjectKind);

impl CityObjectKind {
    pub fn iter_city_object(&self) -> Box<dyn Iterator<Item = CityObjectRef<'_>> + '_> {
        match self {
            Self::AuxiliaryTrafficArea(x) => Box::new(x.iter_city_object()),
            Self::AuxiliaryTrafficSpace(x) => Box::new(x.iter_city_object()),
            Self::Building(x) => Box::new(x.iter_city_object()),
            Self::BuildingConstructiveElement(x) => Box::new(x.iter_city_object()),
            Self::BuildingInstallation(x) => Box::new(x.iter_city_object()),
            Self::BuildingRoom(x) => Box::new(x.iter_city_object()),
            Self::CityFurniture(x) => Box::new(x.iter_city_object()),
            Self::DoorSurface(x) => Box::new(x.iter_city_object()),
            Self::GroundSurface(x) => Box::new(x.iter_city_object()),
            Self::Intersection(x) => Box::new(x.iter_city_object()),
            Self::ReliefFeature(x) => Box::new(x.iter_city_object()),
            Self::Road(x) => Box::new(x.iter_city_object()),
            Self::RoofSurface(x) => Box::new(x.iter_city_object()),
            Self::Section(x) => Box::new(x.iter_city_object()),
            Self::SolitaryVegetationObject(x) => Box::new(x.iter_city_object()),
            Self::Storey(x) => Box::new(x.iter_city_object()),
            Self::TinRelief(x) => Box::new(x.iter_city_object()),
            Self::TrafficArea(x) => Box::new(x.iter_city_object()),
            Self::TrafficSpace(x) => Box::new(x.iter_city_object()),
            Self::WallSurface(x) => Box::new(x.iter_city_object()),
            Self::WindowSurface(x) => Box::new(x.iter_city_object()),
        }
    }

    pub fn city_object_class(&self) -> CityObjectClass {
        match self {
            Self::AuxiliaryTrafficArea(_) => CityObjectClass::AuxiliaryTrafficArea,
            Self::AuxiliaryTrafficSpace(_) => CityObjectClass::AuxiliaryTrafficSpace,
            Self::Building(_) => CityObjectClass::Building,
            Self::BuildingConstructiveElement(_) => CityObjectClass::BuildingConstructiveElement,
            Self::BuildingInstallation(_) => CityObjectClass::BuildingInstallation,
            Self::BuildingRoom(_) => CityObjectClass::BuildingRoom,
            Self::CityFurniture(_) => CityObjectClass::CityFurniture,
            Self::DoorSurface(_) => CityObjectClass::DoorSurface,
            Self::GroundSurface(_) => CityObjectClass::GroundSurface,
            Self::Intersection(_) => CityObjectClass::Intersection,
            Self::ReliefFeature(_) => CityObjectClass::ReliefFeature,
            Self::Road(_) => CityObjectClass::Road,
            Self::RoofSurface(_) => CityObjectClass::RoofSurface,
            Self::Section(_) => CityObjectClass::Section,
            Self::SolitaryVegetationObject(_) => CityObjectClass::SolitaryVegetationObject,
            Self::Storey(_) => CityObjectClass::Storey,
            Self::TinRelief(_) => CityObjectClass::TinRelief,
            Self::TrafficArea(_) => CityObjectClass::TrafficArea,
            Self::TrafficSpace(_) => CityObjectClass::TrafficSpace,
            Self::WallSurface(_) => CityObjectClass::WallSurface,
            Self::WindowSurface(_) => CityObjectClass::WindowSurface,
        }
    }

    pub fn refresh_bounded_by_recursive(&mut self) {
        match self {
            Self::AuxiliaryTrafficArea(x) => x.refresh_bounded_by(),
            Self::AuxiliaryTrafficSpace(x) => x.refresh_bounded_by_recursive(),
            Self::Building(x) => x.refresh_bounded_by_recursive(),
            Self::BuildingConstructiveElement(x) => {
                AsAbstractOccupiedSpaceMut::refresh_bounded_by(x)
            }
            Self::BuildingInstallation(x) => AsAbstractOccupiedSpaceMut::refresh_bounded_by(x),
            Self::BuildingRoom(x) => x.refresh_bounded_by_recursive(),
            Self::CityFurniture(x) => AsAbstractOccupiedSpaceMut::refresh_bounded_by(x),
            Self::DoorSurface(x) => x.refresh_bounded_by(),
            Self::GroundSurface(x) => x.refresh_bounded_by(),
            Self::Intersection(x) => x.refresh_bounded_by_recursive(),
            Self::ReliefFeature(x) => x.refresh_bounded_by_recursive(),
            Self::Road(x) => x.refresh_bounded_by_recursive(),
            Self::RoofSurface(x) => x.refresh_bounded_by(),
            Self::Section(x) => x.refresh_bounded_by_recursive(),
            Self::SolitaryVegetationObject(x) => x.refresh_bounded_by(),
            Self::Storey(x) => x.refresh_bounded_by(),
            Self::TinRelief(x) => x.refresh_bounded_by_recursive(),
            Self::TrafficArea(x) => x.refresh_bounded_by(),
            Self::TrafficSpace(x) => x.refresh_bounded_by_recursive(),
            Self::WallSurface(x) => x.refresh_bounded_by_recursive(),
            Self::WindowSurface(x) => x.refresh_bounded_by(),
        }
    }

    pub fn apply_transform_recursive(&mut self, m: &Isometry3<f64>) {
        match self {
            Self::AuxiliaryTrafficArea(x) => x.apply_transform(m),
            Self::AuxiliaryTrafficSpace(x) => x.apply_transform_recursive(m),
            Self::Building(x) => x.apply_transform_recursive(m),
            Self::BuildingConstructiveElement(x) => {
                AsAbstractOccupiedSpaceMut::apply_transform(x, m)
            }
            Self::BuildingInstallation(x) => AsAbstractOccupiedSpaceMut::apply_transform(x, m),
            Self::BuildingRoom(x) => x.apply_transform_recursive(m),
            Self::CityFurniture(x) => AsAbstractOccupiedSpaceMut::apply_transform(x, m),
            Self::DoorSurface(x) => x.apply_transform(m),
            Self::GroundSurface(x) => x.apply_transform(m),
            Self::Intersection(x) => x.apply_transform_recursive(m),
            Self::ReliefFeature(x) => x.apply_transform_recursive(m),
            Self::Road(x) => x.apply_transform_recursive(m),
            Self::RoofSurface(x) => x.apply_transform(m),
            Self::Section(x) => x.apply_transform_recursive(m),
            Self::SolitaryVegetationObject(x) => AsAbstractOccupiedSpaceMut::apply_transform(x, m),
            Self::Storey(x) => x.apply_transform_recursive(m),
            Self::TinRelief(x) => x.apply_transform(m),
            Self::TrafficArea(x) => x.apply_transform(m),
            Self::TrafficSpace(x) => x.apply_transform_recursive(m),
            Self::WallSurface(x) => x.apply_transform_recursive(m),
            Self::WindowSurface(x) => x.apply_transform(m),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CityObjectRef<'a> {
    AuxiliaryTrafficArea(&'a AuxiliaryTrafficArea),
    AuxiliaryTrafficSpace(&'a AuxiliaryTrafficSpace),
    Building(&'a Building),
    BuildingConstructiveElement(&'a BuildingConstructiveElement),
    BuildingInstallation(&'a BuildingInstallation),
    BuildingRoom(&'a BuildingRoom),
    CityFurniture(&'a CityFurniture),
    DoorSurface(&'a DoorSurface),
    GroundSurface(&'a GroundSurface),
    Intersection(&'a Intersection),
    ReliefFeature(&'a ReliefFeature),
    Road(&'a Road),
    RoofSurface(&'a RoofSurface),
    Section(&'a Section),
    SolitaryVegetationObject(&'a SolitaryVegetationObject),
    Storey(&'a Storey),
    TinRelief(&'a TinRelief),
    TrafficArea(&'a TrafficArea),
    TrafficSpace(&'a TrafficSpace),
    WallSurface(&'a WallSurface),
    WindowSurface(&'a WindowSurface),
}

impl<'a> AsAbstractCityObject for CityObjectRef<'a> {
    fn abstract_city_object(&self) -> &AbstractCityObject {
        match self {
            Self::AuxiliaryTrafficArea(x) => x.abstract_city_object(),
            Self::AuxiliaryTrafficSpace(x) => x.abstract_city_object(),
            Self::Building(x) => x.abstract_city_object(),
            Self::BuildingConstructiveElement(x) => x.abstract_city_object(),
            Self::BuildingInstallation(x) => x.abstract_city_object(),
            Self::BuildingRoom(x) => x.abstract_city_object(),
            Self::CityFurniture(x) => x.abstract_city_object(),
            Self::DoorSurface(x) => x.abstract_city_object(),
            Self::GroundSurface(x) => x.abstract_city_object(),
            Self::Intersection(x) => x.abstract_city_object(),
            Self::ReliefFeature(x) => x.abstract_city_object(),
            Self::Road(x) => x.abstract_city_object(),
            Self::RoofSurface(x) => x.abstract_city_object(),
            Self::Section(x) => x.abstract_city_object(),
            Self::SolitaryVegetationObject(x) => x.abstract_city_object(),
            Self::Storey(x) => x.abstract_city_object(),
            Self::TinRelief(x) => x.abstract_city_object(),
            Self::TrafficArea(x) => x.abstract_city_object(),
            Self::TrafficSpace(x) => x.abstract_city_object(),
            Self::WallSurface(x) => x.abstract_city_object(),
            Self::WindowSurface(x) => x.abstract_city_object(),
        }
    }
}

impl<'a> AsAbstractFeatureWithLifespan for CityObjectRef<'a> {
    fn abstract_feature_with_lifespan(&self) -> &AbstractFeatureWithLifespan {
        match self {
            Self::AuxiliaryTrafficArea(x) => x.abstract_feature_with_lifespan(),
            Self::AuxiliaryTrafficSpace(x) => x.abstract_feature_with_lifespan(),
            Self::Building(x) => x.abstract_feature_with_lifespan(),
            Self::BuildingConstructiveElement(x) => x.abstract_feature_with_lifespan(),
            Self::BuildingInstallation(x) => x.abstract_feature_with_lifespan(),
            Self::BuildingRoom(x) => x.abstract_feature_with_lifespan(),
            Self::CityFurniture(x) => x.abstract_feature_with_lifespan(),
            Self::DoorSurface(x) => x.abstract_feature_with_lifespan(),
            Self::GroundSurface(x) => x.abstract_feature_with_lifespan(),
            Self::Intersection(x) => x.abstract_feature_with_lifespan(),
            Self::ReliefFeature(x) => x.abstract_feature_with_lifespan(),
            Self::Road(x) => x.abstract_feature_with_lifespan(),
            Self::RoofSurface(x) => x.abstract_feature_with_lifespan(),
            Self::Section(x) => x.abstract_feature_with_lifespan(),
            Self::SolitaryVegetationObject(x) => x.abstract_feature_with_lifespan(),
            Self::Storey(x) => x.abstract_feature_with_lifespan(),
            Self::TinRelief(x) => x.abstract_feature_with_lifespan(),
            Self::TrafficArea(x) => x.abstract_feature_with_lifespan(),
            Self::TrafficSpace(x) => x.abstract_feature_with_lifespan(),
            Self::WallSurface(x) => x.abstract_feature_with_lifespan(),
            Self::WindowSurface(x) => x.abstract_feature_with_lifespan(),
        }
    }
}

impl<'a> AsAbstractFeature for CityObjectRef<'a> {
    fn abstract_feature(&self) -> &AbstractFeature {
        match self {
            Self::AuxiliaryTrafficArea(x) => x.abstract_feature(),
            Self::AuxiliaryTrafficSpace(x) => x.abstract_feature(),
            Self::Building(x) => x.abstract_feature(),
            Self::BuildingConstructiveElement(x) => x.abstract_feature(),
            Self::BuildingInstallation(x) => x.abstract_feature(),
            Self::BuildingRoom(x) => x.abstract_feature(),
            Self::CityFurniture(x) => x.abstract_feature(),
            Self::DoorSurface(x) => x.abstract_feature(),
            Self::GroundSurface(x) => x.abstract_feature(),
            Self::Intersection(x) => x.abstract_feature(),
            Self::ReliefFeature(x) => x.abstract_feature(),
            Self::Road(x) => x.abstract_feature(),
            Self::RoofSurface(x) => x.abstract_feature(),
            Self::Section(x) => x.abstract_feature(),
            Self::SolitaryVegetationObject(x) => x.abstract_feature(),
            Self::Storey(x) => x.abstract_feature(),
            Self::TinRelief(x) => x.abstract_feature(),
            Self::TrafficArea(x) => x.abstract_feature(),
            Self::TrafficSpace(x) => x.abstract_feature(),
            Self::WallSurface(x) => x.abstract_feature(),
            Self::WindowSurface(x) => x.abstract_feature(),
        }
    }
}

impl<'a> CityObjectRef<'a> {
    pub fn city_object_class(&self) -> CityObjectClass {
        match self {
            Self::AuxiliaryTrafficArea(_) => CityObjectClass::AuxiliaryTrafficArea,
            Self::AuxiliaryTrafficSpace(_) => CityObjectClass::AuxiliaryTrafficSpace,
            Self::Building(_) => CityObjectClass::Building,
            Self::BuildingConstructiveElement(_) => CityObjectClass::BuildingConstructiveElement,
            Self::BuildingInstallation(_) => CityObjectClass::BuildingInstallation,
            Self::BuildingRoom(_) => CityObjectClass::BuildingRoom,
            Self::CityFurniture(_) => CityObjectClass::CityFurniture,
            Self::DoorSurface(_) => CityObjectClass::DoorSurface,
            Self::GroundSurface(_) => CityObjectClass::GroundSurface,
            Self::Intersection(_) => CityObjectClass::Intersection,
            Self::ReliefFeature(_) => CityObjectClass::ReliefFeature,
            Self::Road(_) => CityObjectClass::Road,
            Self::RoofSurface(_) => CityObjectClass::RoofSurface,
            Self::Section(_) => CityObjectClass::Section,
            Self::SolitaryVegetationObject(_) => CityObjectClass::SolitaryVegetationObject,
            Self::Storey(_) => CityObjectClass::Storey,
            Self::TinRelief(_) => CityObjectClass::TinRelief,
            Self::TrafficArea(_) => CityObjectClass::TrafficArea,
            Self::TrafficSpace(_) => CityObjectClass::TrafficSpace,
            Self::WallSurface(_) => CityObjectClass::WallSurface,
            Self::WindowSurface(_) => CityObjectClass::WindowSurface,
        }
    }
}

#[derive(Debug)]
pub enum CityObjectRefMut<'a> {
    AuxiliaryTrafficArea(&'a mut AuxiliaryTrafficArea),
    AuxiliaryTrafficSpace(&'a mut AuxiliaryTrafficSpace),
    Building(&'a mut Building),
    BuildingConstructiveElement(&'a mut BuildingConstructiveElement),
    CityFurniture(&'a mut CityFurniture),
    DoorSurface(&'a mut DoorSurface),
    GroundSurface(&'a mut GroundSurface),
    Intersection(&'a mut Intersection),
    ReliefFeature(&'a mut ReliefFeature),
    Road(&'a mut Road),
    RoofSurface(&'a mut RoofSurface),
    Section(&'a mut Section),
    SolitaryVegetationObject(&'a mut SolitaryVegetationObject),
    Storey(&'a mut Storey),
    TinRelief(&'a mut TinRelief),
    TrafficArea(&'a mut TrafficArea),
    TrafficSpace(&'a mut TrafficSpace),
    WallSurface(&'a mut WallSurface),
    WindowSurface(&'a mut WindowSurface),
}

impl<'a> AsAbstractFeature for CityObjectRefMut<'a> {
    fn abstract_feature(&self) -> &AbstractFeature {
        match self {
            Self::AuxiliaryTrafficArea(x) => x.abstract_feature(),
            Self::AuxiliaryTrafficSpace(x) => x.abstract_feature(),
            Self::Building(x) => x.abstract_feature(),
            Self::BuildingConstructiveElement(x) => x.abstract_feature(),
            Self::CityFurniture(x) => x.abstract_feature(),
            Self::DoorSurface(x) => x.abstract_feature(),
            Self::GroundSurface(x) => x.abstract_feature(),
            Self::Intersection(x) => x.abstract_feature(),
            Self::ReliefFeature(x) => x.abstract_feature(),
            Self::Road(x) => x.abstract_feature(),
            Self::RoofSurface(x) => x.abstract_feature(),
            Self::Section(x) => x.abstract_feature(),
            Self::SolitaryVegetationObject(x) => x.abstract_feature(),
            Self::Storey(x) => x.abstract_feature(),
            Self::TinRelief(x) => x.abstract_feature(),
            Self::TrafficArea(x) => x.abstract_feature(),
            Self::TrafficSpace(x) => x.abstract_feature(),
            Self::WallSurface(x) => x.abstract_feature(),
            Self::WindowSurface(x) => x.abstract_feature(),
        }
    }
}

impl<'a> AsAbstractFeatureMut for CityObjectRefMut<'a> {
    fn abstract_feature_mut(&mut self) -> &mut AbstractFeature {
        match self {
            Self::AuxiliaryTrafficArea(x) => x.abstract_feature_mut(),
            Self::AuxiliaryTrafficSpace(x) => x.abstract_feature_mut(),
            Self::Building(x) => x.abstract_feature_mut(),
            Self::BuildingConstructiveElement(x) => x.abstract_feature_mut(),
            Self::CityFurniture(x) => x.abstract_feature_mut(),
            Self::DoorSurface(x) => x.abstract_feature_mut(),
            Self::GroundSurface(x) => x.abstract_feature_mut(),
            Self::Intersection(x) => x.abstract_feature_mut(),
            Self::ReliefFeature(x) => x.abstract_feature_mut(),
            Self::Road(x) => x.abstract_feature_mut(),
            Self::RoofSurface(x) => x.abstract_feature_mut(),
            Self::Section(x) => x.abstract_feature_mut(),
            Self::SolitaryVegetationObject(x) => x.abstract_feature_mut(),
            Self::Storey(x) => x.abstract_feature_mut(),
            Self::TinRelief(x) => x.abstract_feature_mut(),
            Self::TrafficArea(x) => x.abstract_feature_mut(),
            Self::TrafficSpace(x) => x.abstract_feature_mut(),
            Self::WallSurface(x) => x.abstract_feature_mut(),
            Self::WindowSurface(x) => x.abstract_feature_mut(),
        }
    }
}
