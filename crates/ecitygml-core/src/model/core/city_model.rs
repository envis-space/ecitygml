use crate::model::common::{FeatureRef, FeatureRefMut, TopLevelFeatureRef};
use crate::model::core::{
    AbstractFeature, AbstractFeatureWithLifespan, AsAbstractFeature, AsAbstractFeatureWithLifespan,
    AsAbstractFeatureWithLifespanMut, CityObjectProperty,
};
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CityModel {
    pub(crate) abstract_feature_with_lifespan: AbstractFeatureWithLifespan,
    pub city_object_members: Vec<CityObjectProperty>,
}

impl CityModel {
    pub fn new(abstract_feature_with_lifespan: AbstractFeatureWithLifespan) -> Self {
        Self {
            abstract_feature_with_lifespan,
            city_object_members: Default::default(),
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        self.city_object_members
            .iter()
            .flat_map(|p| p.object.as_ref())
            .flat_map(|x| x.iter_features())
    }

    pub fn recompute_child_bounding_shapes(&mut self) {
        self.city_object_members
            .par_iter_mut()
            .filter_map(|x| x.object.as_mut())
            .for_each(|current_city_object| {
                current_city_object.for_each_feature_mut(&mut |mut x: FeatureRefMut<'_>| {
                    x.recompute_bounding_shape()
                })
            });
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.city_object_members
            .par_iter_mut()
            .filter_map(|x| x.object.as_mut())
            .for_each(|x| x.apply_transform(m));
    }

    pub fn from_city_models(city_models: Vec<Self>) -> Self {
        let abstract_feature = AbstractFeature::new(Id::generate_uuid_v4());
        let abstract_feature_with_lifespan = AbstractFeatureWithLifespan::new(abstract_feature);
        let mut combined_city_model = CityModel::new(abstract_feature_with_lifespan);

        let combined_city_object_members: Vec<CityObjectProperty> = city_models
            .into_iter()
            .flat_map(|x| x.city_object_members)
            .collect();
        combined_city_model.set_city_object_members(combined_city_object_members);

        combined_city_model
    }

    pub fn is_empty(&self) -> bool {
        self.city_object_members.is_empty()
    }

    pub fn city_object_members(&self) -> &Vec<CityObjectProperty> {
        &self.city_object_members
    }

    pub fn city_object_members_len(&self) -> usize {
        self.city_object_members.len()
    }

    pub fn set_city_object_members(&mut self, values: Vec<CityObjectProperty>) {
        self.city_object_members = values;
    }

    pub fn iter_top_level_features(&self) -> impl Iterator<Item = TopLevelFeatureRef<'_>> + '_ {
        self.city_object_members
            .iter()
            .flat_map(|p| p.object.as_ref())
            .filter_map(|x| x.try_into().ok())
    }

    pub fn filter_city_objects_by_envelope(&mut self, filter_envelope: &Envelope) {
        self.city_object_members.retain(|obj| {
            if let Some(x) = obj.object.as_ref() {
                x.bounded_by()
                    .is_some_and(|env| filter_envelope.contains_envelope_partially(env))
            } else {
                false
            }
        });
    }
}

impl AsAbstractFeatureWithLifespan for CityModel {
    fn abstract_feature_with_lifespan(&self) -> &AbstractFeatureWithLifespan {
        &self.abstract_feature_with_lifespan
    }
}

impl AsAbstractFeatureWithLifespanMut for CityModel {
    fn abstract_feature_with_lifespan_mut(&mut self) -> &mut AbstractFeatureWithLifespan {
        &mut self.abstract_feature_with_lifespan
    }
}

crate::impl_abstract_feature_with_lifespan_traits!(CityModel);
