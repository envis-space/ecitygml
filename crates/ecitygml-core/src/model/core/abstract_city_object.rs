use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::city_object_relation_property::CityObjectRelationProperty;
use crate::model::core::enums::{RelativeToTerrain, RelativeToWater};
use crate::model::core::refs::{AbstractFeatureKindRef, AbstractFeatureKindRefMut};
use crate::model::core::{
    AbstractAppearanceProperty, AbstractFeatureWithLifespan, AsAbstractFeatureWithLifespan,
    AsAbstractFeatureWithLifespanMut, ExternalReference,
};
use crate::model::generics::GenericAttributeKind;
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractCityObject {
    pub(crate) abstract_feature_with_lifespan: AbstractFeatureWithLifespan,
    relative_to_terrain: Option<RelativeToTerrain>,
    relative_to_water: Option<RelativeToWater>,
    external_references: Vec<ExternalReference>,
    generic_attributes: Vec<GenericAttributeKind>,
    appearances: Vec<AbstractAppearanceProperty>,
    related_to: Vec<CityObjectRelationProperty>,
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
            relative_to_terrain: None,
            relative_to_water: None,
            external_references: Vec::new(),
            generic_attributes: Vec::new(),
            appearances: Vec::new(),
            related_to: Vec::new(),
        }
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

    fn external_references(&self) -> &[ExternalReference] {
        &self.abstract_city_object().external_references
    }

    fn generic_attributes(&self) -> &[GenericAttributeKind] {
        &self.abstract_city_object().generic_attributes
    }

    fn appearances(&self) -> &[AbstractAppearanceProperty] {
        &self.abstract_city_object().appearances
    }

    fn related_to(&self) -> &[CityObjectRelationProperty] {
        &self.abstract_city_object().related_to
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

    fn external_references_mut(&mut self) -> &mut Vec<ExternalReference> {
        &mut self.abstract_city_object_mut().external_references
    }

    fn set_external_references(&mut self, external_references: Vec<ExternalReference>) {
        self.abstract_city_object_mut().external_references = external_references;
    }

    fn push_external_reference(&mut self, external_reference: ExternalReference) {
        self.abstract_city_object_mut()
            .external_references
            .push(external_reference);
    }

    fn extend_external_references(
        &mut self,
        external_references: impl IntoIterator<Item = ExternalReference>,
    ) {
        self.abstract_city_object_mut()
            .external_references
            .extend(external_references);
    }

    fn generic_attributes_mut(&mut self) -> &mut Vec<GenericAttributeKind> {
        &mut self.abstract_city_object_mut().generic_attributes
    }

    fn set_generic_attributes(&mut self, generic_attributes: Vec<GenericAttributeKind>) {
        self.abstract_city_object_mut().generic_attributes = generic_attributes;
    }

    fn push_generic_attribute(&mut self, generic_attribute: GenericAttributeKind) {
        self.abstract_city_object_mut()
            .generic_attributes
            .push(generic_attribute);
    }

    fn extend_generic_attributes(
        &mut self,
        generic_attributes: impl IntoIterator<Item = GenericAttributeKind>,
    ) {
        self.abstract_city_object_mut()
            .generic_attributes
            .extend(generic_attributes);
    }

    fn appearances_mut(&mut self) -> &mut Vec<AbstractAppearanceProperty> {
        &mut self.abstract_city_object_mut().appearances
    }

    fn set_appearances(&mut self, appearances: Vec<AbstractAppearanceProperty>) {
        self.abstract_city_object_mut().appearances = appearances;
    }

    fn push_appearance(&mut self, appearance: AbstractAppearanceProperty) {
        self.abstract_city_object_mut().appearances.push(appearance);
    }

    fn extend_appearances(
        &mut self,
        appearances: impl IntoIterator<Item = AbstractAppearanceProperty>,
    ) {
        self.abstract_city_object_mut()
            .appearances
            .extend(appearances);
    }

    fn related_to_mut(&mut self) -> &mut Vec<CityObjectRelationProperty> {
        &mut self.abstract_city_object_mut().related_to
    }

    fn set_related_to(&mut self, related_to: Vec<CityObjectRelationProperty>) {
        self.abstract_city_object_mut().related_to = related_to;
    }

    fn push_related_to(&mut self, related_to: CityObjectRelationProperty) {
        self.abstract_city_object_mut().related_to.push(related_to);
    }

    fn extend_related_to(
        &mut self,
        related_to: impl IntoIterator<Item = CityObjectRelationProperty>,
    ) {
        self.abstract_city_object_mut()
            .related_to
            .extend(related_to);
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
                &<$type as $crate::model::core::AsAbstractCityObject>::abstract_city_object(self)
                    .abstract_feature_with_lifespan
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
                &mut <$type as $crate::model::core::AsAbstractCityObjectMut>::abstract_city_object_mut(self).abstract_feature_with_lifespan
            }
        }
    };
}

impl_abstract_city_object_traits!(AbstractCityObject);
impl_abstract_city_object_mut_traits!(AbstractCityObject);

impl IterFeatures for AbstractCityObject {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            self.appearances
                .iter()
                .filter_map(|x| x.object())
                .flat_map(|x| x.iter_features()),
        )
    }
}

impl ForEachFeatureMut for AbstractCityObject {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        for prop in &mut self.appearances {
            if let Some(x) = prop.object_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }
}

impl ComputeEnvelope for AbstractCityObject {
    fn compute_envelope(&self) -> Option<Envelope> {
        None
    }
}

impl ApplyTransform for AbstractCityObject {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_feature_with_lifespan.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_feature_with_lifespan.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_feature_with_lifespan
            .apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_feature_with_lifespan.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_feature_with_lifespan.apply_scale(scale);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::core::AsAbstractFeature;

    #[test]
    fn trait_implementation_macro_test() {
        let abstract_city_object =
            AbstractCityObject::new(egml::model::base::Id::generate_uuid_v4());
        abstract_city_object.feature_id();
    }
}
