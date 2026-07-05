use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::core::refs::{
    CityObjectKindRef, FeatureKindRef, FeatureKindRefMut, FeatureWithLifespanKindRef,
};
use crate::model::core::{
    AbstractFeatureWithLifespan, AppearanceProperty, AsAbstractFeature,
    AsAbstractFeatureWithLifespan, AsAbstractFeatureWithLifespanMut, CityObjectProperty,
};
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct CityModel {
    pub(crate) abstract_feature_with_lifespan: AbstractFeatureWithLifespan,
    city_object_members: Vec<CityObjectProperty>,
    appearance_members: Vec<AppearanceProperty>,
}

impl CityModel {
    pub fn new(abstract_feature_with_lifespan: AbstractFeatureWithLifespan) -> Self {
        Self {
            abstract_feature_with_lifespan,
            city_object_members: Vec::new(),
            appearance_members: Vec::new(),
        }
    }

    pub fn city_object_members(&self) -> &[CityObjectProperty] {
        &self.city_object_members
    }

    pub fn set_city_object_members(&mut self, city_object_members: Vec<CityObjectProperty>) {
        self.city_object_members = city_object_members;
    }

    pub fn push_city_object_member(&mut self, member: CityObjectProperty) {
        self.city_object_members.push(member);
    }

    pub fn extend_city_object_members(
        &mut self,
        members: impl IntoIterator<Item = CityObjectProperty>,
    ) {
        self.city_object_members.extend(members);
    }

    pub fn appearance_members(&self) -> &[AppearanceProperty] {
        &self.appearance_members
    }

    pub fn set_appearance_members(&mut self, appearance_members: Vec<AppearanceProperty>) {
        self.appearance_members = appearance_members;
    }

    pub fn push_appearance_member(&mut self, member: AppearanceProperty) {
        self.appearance_members.push(member);
    }

    pub fn extend_appearance_members(
        &mut self,
        members: impl IntoIterator<Item = AppearanceProperty>,
    ) {
        self.appearance_members.extend(members);
    }
}

impl CityModel {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        self.city_object_members
            .iter()
            .flat_map(|p| p.object.as_ref())
            .flat_map(|x| x.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        for prop in &mut self.city_object_members {
            if let Some(x) = prop.object.as_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }

    pub fn recompute_child_bounding_shapes(&mut self) {
        self.city_object_members
            .par_iter_mut()
            .filter_map(|x| x.object.as_mut())
            .for_each(|current_city_object| {
                current_city_object.for_each_feature_mut(&mut |mut x: FeatureKindRefMut<'_>| {
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
        let abstract_feature_with_lifespan =
            AbstractFeatureWithLifespan::new(Id::generate_uuid_v4());
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

    pub fn city_object_members_len(&self) -> usize {
        self.city_object_members.len()
    }

    pub fn iter_city_objects<'a>(&'a self) -> impl Iterator<Item = CityObjectKindRef<'a>> + 'a {
        self.city_object_members
            .iter()
            .flat_map(|p| p.object.as_ref())
            .flat_map(|x| {
                std::iter::once(CityObjectKindRef::from(x)).chain(x.iter_features().filter_map(
                    |f| match f {
                        FeatureKindRef::FeatureWithLifespanKind(
                            FeatureWithLifespanKindRef::CityObjectKind(co),
                        ) => Some(co),
                        _ => None,
                    },
                ))
            })
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

    pub fn feature_counts(&self) -> HashMap<FeatureType, usize> {
        let mut counts = HashMap::new();
        for feature in self.iter_features() {
            *counts.entry(feature.feature_type()).or_insert(0) += 1;
        }
        counts
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
crate::impl_abstract_feature_with_lifespan_mut_traits!(CityModel);
