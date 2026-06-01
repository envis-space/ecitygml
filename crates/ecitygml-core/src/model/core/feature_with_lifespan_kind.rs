use crate::impl_abstract_feature_with_lifespan_traits;
use crate::model::core::city_object_kind::CityObjectKind;
use crate::model::core::{
    AbstractFeatureWithLifespan, AsAbstractFeatureWithLifespan, AsAbstractFeatureWithLifespanMut,
    CityModel,
};

#[derive(Debug, Clone, PartialEq)]
pub enum FeatureWithLifespanKind {
    CityModel(CityModel),
    CityObjectKind(CityObjectKind),
}

impl AsAbstractFeatureWithLifespan for FeatureWithLifespanKind {
    fn abstract_feature_with_lifespan(&self) -> &AbstractFeatureWithLifespan {
        match self {
            FeatureWithLifespanKind::CityModel(x) => x.abstract_feature_with_lifespan(),
            FeatureWithLifespanKind::CityObjectKind(x) => x.abstract_feature_with_lifespan(),
        }
    }
}

impl AsAbstractFeatureWithLifespanMut for FeatureWithLifespanKind {
    fn abstract_feature_with_lifespan_mut(&mut self) -> &mut AbstractFeatureWithLifespan {
        match self {
            FeatureWithLifespanKind::CityModel(x) => x.abstract_feature_with_lifespan_mut(),
            FeatureWithLifespanKind::CityObjectKind(x) => x.abstract_feature_with_lifespan_mut(),
        }
    }
}

impl_abstract_feature_with_lifespan_traits!(FeatureWithLifespanKind);
