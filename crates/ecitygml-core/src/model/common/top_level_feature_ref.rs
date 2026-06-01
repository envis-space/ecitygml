use crate::model::building::Building;
use crate::model::city_furniture::CityFurniture;
use crate::model::common::{FeatureRef, FeatureType};
use crate::model::core::{
    AbstractCityObject, AbstractFeature, AbstractFeatureWithLifespan, AsAbstractCityObject,
    AsAbstractFeature, AsAbstractFeatureWithLifespan,
};
use crate::model::relief::ReliefFeature;
use crate::model::transportation::Road;
use crate::model::vegetation::SolitaryVegetationObject;

/// A reference to a city object that appears directly in a [`crate::model::core::CityModel`],
/// i.e. a top-level member rather than a child of another object.
#[derive(Debug, Clone, Copy)]
pub enum TopLevelFeatureRef<'a> {
    Building(&'a Building),
    CityFurniture(&'a CityFurniture),
    ReliefFeature(&'a ReliefFeature),
    Road(&'a Road),
    SolitaryVegetationObject(&'a SolitaryVegetationObject),
}

impl<'a> TopLevelFeatureRef<'a> {
    pub fn feature_type(&self) -> FeatureType {
        match self {
            TopLevelFeatureRef::Building(_) => FeatureType::Building,
            TopLevelFeatureRef::CityFurniture(_) => FeatureType::CityFurniture,
            TopLevelFeatureRef::ReliefFeature(_) => FeatureType::ReliefFeature,
            TopLevelFeatureRef::Road(_) => FeatureType::Road,
            TopLevelFeatureRef::SolitaryVegetationObject(_) => {
                FeatureType::SolitaryVegetationObject
            }
        }
    }
}

impl<'a> AsAbstractCityObject for TopLevelFeatureRef<'a> {
    fn abstract_city_object(&self) -> &AbstractCityObject {
        match self {
            TopLevelFeatureRef::Building(x) => x.abstract_city_object(),
            TopLevelFeatureRef::CityFurniture(x) => x.abstract_city_object(),
            TopLevelFeatureRef::ReliefFeature(x) => x.abstract_city_object(),
            TopLevelFeatureRef::Road(x) => x.abstract_city_object(),
            TopLevelFeatureRef::SolitaryVegetationObject(x) => x.abstract_city_object(),
        }
    }
}

impl<'a> AsAbstractFeatureWithLifespan for TopLevelFeatureRef<'a> {
    fn abstract_feature_with_lifespan(&self) -> &AbstractFeatureWithLifespan {
        match self {
            TopLevelFeatureRef::Building(x) => x.abstract_feature_with_lifespan(),
            TopLevelFeatureRef::CityFurniture(x) => x.abstract_feature_with_lifespan(),
            TopLevelFeatureRef::ReliefFeature(x) => x.abstract_feature_with_lifespan(),
            TopLevelFeatureRef::Road(x) => x.abstract_feature_with_lifespan(),
            TopLevelFeatureRef::SolitaryVegetationObject(x) => x.abstract_feature_with_lifespan(),
        }
    }
}

impl<'a> AsAbstractFeature for TopLevelFeatureRef<'a> {
    fn abstract_feature(&self) -> &AbstractFeature {
        match self {
            TopLevelFeatureRef::Building(x) => x.abstract_feature(),
            TopLevelFeatureRef::CityFurniture(x) => x.abstract_feature(),
            TopLevelFeatureRef::ReliefFeature(x) => x.abstract_feature(),
            TopLevelFeatureRef::Road(x) => x.abstract_feature(),
            TopLevelFeatureRef::SolitaryVegetationObject(x) => x.abstract_feature(),
        }
    }
}

impl<'a> From<TopLevelFeatureRef<'a>> for FeatureRef<'a> {
    fn from(value: TopLevelFeatureRef<'a>) -> Self {
        match value {
            TopLevelFeatureRef::Building(x) => Self::Building(x),
            TopLevelFeatureRef::CityFurniture(x) => Self::CityFurniture(x),
            TopLevelFeatureRef::ReliefFeature(x) => Self::ReliefFeature(x),
            TopLevelFeatureRef::Road(x) => Self::Road(x),
            TopLevelFeatureRef::SolitaryVegetationObject(x) => Self::SolitaryVegetationObject(x),
        }
    }
}
