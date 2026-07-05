use crate::model::core::enums::{RelativeToTerrain, RelativeToWater};
use crate::model::core::refs::{FeatureKindRef, FeatureKindRefMut};
use crate::model::core::{
    AbstractFeatureWithLifespan, AppearanceProperty, AsAbstractFeatureWithLifespan,
    AsAbstractFeatureWithLifespanMut, ExternalReference,
};
use crate::model::generics::GenericAttributeKind;
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractCityObject {
    pub(crate) abstract_feature_with_lifespan: AbstractFeatureWithLifespan,
    external_references: Vec<ExternalReference>,
    relative_to_terrain: Option<RelativeToTerrain>,
    relative_to_water: Option<RelativeToWater>,
    generic_attributes: Vec<GenericAttributeKind>,
    appearances: Vec<AppearanceProperty>,
}

impl AbstractCityObject {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_feature_with_lifespan(AbstractFeatureWithLifespan::new(id))
    }

    pub fn from_abstract_feature_with_lifespan(
        abstract_feature_with_lifespan: AbstractFeatureWithLifespan,
    ) -> Self {
        Self {
            abstract_feature_with_lifespan,
            external_references: Vec::new(),
            relative_to_terrain: None,
            relative_to_water: None,
            generic_attributes: Vec::new(),
            appearances: Vec::new(),
        }
    }
}
impl AbstractCityObject {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        std::iter::empty()
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, _f: &mut F) {}

    pub fn compute_envelope(&self) -> Option<Envelope> {
        None
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_feature_with_lifespan.apply_transform(m);
    }
}

pub trait AsAbstractCityObject: AsAbstractFeatureWithLifespan {
    fn abstract_city_object(&self) -> &AbstractCityObject;

    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        self.abstract_city_object().relative_to_terrain
    }

    fn relative_to_water(&self) -> Option<RelativeToWater> {
        self.abstract_city_object().relative_to_water
    }

    fn generic_attributes(&self) -> &[GenericAttributeKind] {
        &self.abstract_city_object().generic_attributes
    }

    fn external_references(&self) -> &[ExternalReference] {
        &self.abstract_city_object().external_references
    }

    fn appearances(&self) -> &[AppearanceProperty] {
        &self.abstract_city_object().appearances
    }
}

pub trait AsAbstractCityObjectMut: AsAbstractFeatureWithLifespanMut + AsAbstractCityObject {
    fn abstract_city_object_mut(&mut self) -> &mut AbstractCityObject;

    fn set_relative_to_terrain(&mut self, value: Option<RelativeToTerrain>) {
        self.abstract_city_object_mut().relative_to_terrain = value;
    }

    fn set_relative_to_water(&mut self, value: Option<RelativeToWater>) {
        self.abstract_city_object_mut().relative_to_water = value;
    }

    fn set_generic_attributes(&mut self, generic_attributes: Vec<GenericAttributeKind>) {
        self.abstract_city_object_mut().generic_attributes = generic_attributes;
    }

    fn set_external_references(&mut self, external_references: Vec<ExternalReference>) {
        self.abstract_city_object_mut().external_references = external_references;
    }

    fn set_appearances(&mut self, appearances: Vec<AppearanceProperty>) {
        self.abstract_city_object_mut().appearances = appearances;
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
    };
}

#[macro_export]
macro_rules! impl_abstract_city_object_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_feature_with_lifespan_mut_traits!($type);

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
impl_abstract_city_object_mut_traits!(AbstractCityObject);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::core::AsAbstractFeature;

    #[test]
    fn trait_implementation_macro_test() {
        let abstract_city_object =
            AbstractCityObject::new(egml::model::base::Id::generate_uuid_v4());
        abstract_city_object.id();
    }
}
