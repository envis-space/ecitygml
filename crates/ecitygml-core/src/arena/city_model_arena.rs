use crate::Error;
use crate::arena::convert::core::{flatten_city_model, unflatten_city_model_arena};
use crate::model::core::refs::{
    AbstractCityObjectKindRef, AbstractFeatureKindRef, AbstractFeatureKindRefMut,
};
use crate::model::core::{
    AbstractFeatureKind, AbstractFeatureWithLifespanKind, AsAbstractFeature, CityModel,
};
use egml::model::base::Id;
use egml::model::common::RecomputeBoundingShape;
use rayon::prelude::*;
use slotmap::{DefaultKey, SlotMap};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct CityModelArena {
    features: SlotMap<DefaultKey, AbstractFeatureKind>,
    by_id: HashMap<Id, DefaultKey>,
}

impl CityModelArena {
    pub fn from_city_model(city_model: CityModel) -> Result<Self, Error> {
        let mut city_model_arena = CityModelArena::default();
        flatten_city_model(city_model, &mut city_model_arena);

        Ok(city_model_arena)
    }

    pub fn to_city_model(self) -> Result<CityModel, Error> {
        unflatten_city_model_arena(self)
    }

    pub fn features(&self) -> &SlotMap<DefaultKey, AbstractFeatureKind> {
        &self.features
    }

    pub fn features_len(&self) -> usize {
        self.features.len()
    }

    pub fn get_by_id(&self, id: &Id) -> Option<AbstractFeatureKindRef<'_>> {
        let key = *self.by_id.get(id)?;
        self.features.get(key).map(|feature| feature.into())
    }

    pub fn insert_feature(&mut self, feature: AbstractFeatureKind) -> DefaultKey {
        let feature_id = feature.feature_id().clone();
        let key = self.features.insert(feature);
        self.by_id.insert(feature_id, key);

        key
    }

    pub fn iter_features(&self) -> impl Iterator<Item = AbstractFeatureKindRef<'_>> {
        self.features.values().map(|feature| feature.into())
    }

    pub fn iter_city_objects(&self) -> impl Iterator<Item = AbstractCityObjectKindRef<'_>> {
        self.features.values().filter_map(|feature| match feature {
            AbstractFeatureKind::AbstractFeatureWithLifespanKind(
                AbstractFeatureWithLifespanKind::AbstractCityObjectKind(x),
            ) => Some(x.into()),
            _ => None,
        })
    }

    pub fn get_city_object_by_id(&self, id: &Id) -> Option<AbstractCityObjectKindRef<'_>> {
        let key = *self.by_id.get(id)?;
        match self.features.get(key)? {
            AbstractFeatureKind::AbstractFeatureWithLifespanKind(
                AbstractFeatureWithLifespanKind::AbstractCityObjectKind(x),
            ) => Some(x.into()),
            _ => None,
        }
    }

    pub fn recompute_bounding_shapes(&mut self) {
        self.features
            .values_mut()
            .collect::<Vec<_>>()
            .into_par_iter()
            .for_each(|feature| {
                let mut feature_ref: AbstractFeatureKindRefMut<'_> = feature.into();
                feature_ref.recompute_bounding_shape();
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::building::AsAbstractBuilding;
    use crate::model::construction::AsAbstractConstruction;
    use crate::model::core::AsAbstractFeatureWithLifespan;
    use egml::io::util::extract_xml_element_spans;

    #[test]
    fn test_arena_construction() {
        let mut sm = SlotMap::new();
        let foo = sm.insert("foo"); // Key generated on insert.
        let bar = sm.insert("bar");

        let _a = sm.get(foo);

        assert_eq!(sm[foo], "foo");
        assert_eq!(sm[bar], "bar");
    }
}
