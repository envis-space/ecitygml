use crate::model::common::IterFeatures;
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::{AsAbstractFeature, CityModel};
use egml::model::base::Id;
use std::collections::HashMap;

/// Resolves `gml:id`/`xlink:href` references to the feature they point at.
///
/// Built once from a [`CityModel`] snapshot; the resolver borrows from that model, so it
/// must be rebuilt after any mutation.
#[derive(Debug, Clone, Default)]
pub struct FeatureResolver<'a> {
    by_id: HashMap<Id, AbstractFeatureKindRef<'a>>,
}

impl<'a> FeatureResolver<'a> {
    pub fn from_city_model(city_model: &'a CityModel) -> Self {
        let by_id = city_model
            .iter_features()
            .map(|x| (x.feature_id().clone(), x))
            .collect();

        Self { by_id }
    }

    pub fn resolve(&self, id: &Id) -> Option<AbstractFeatureKindRef<'a>> {
        self.by_id.get(id).copied()
    }

    /// Resolves and narrows in one step, e.g. `resolve_as::<&TrafficSpace>(id)` or
    /// `resolve_as::<AbstractCityObjectKindRef>(id)`. Returns `None` if the id is unknown
    /// or the resolved feature isn't a `T`.
    pub fn resolve_as<T>(&self, id: &Id) -> Option<T>
    where
        T: TryFrom<AbstractFeatureKindRef<'a>>,
    {
        self.resolve(id).and_then(|f| T::try_from(f).ok())
    }

    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }
}
