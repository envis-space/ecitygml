use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::core::refs::{
    AbstractCityObjectKindRef, AbstractFeatureKindRef, AbstractFeatureKindRefMut,
    AbstractFeatureWithLifespanKindRef,
};
use crate::model::core::{
    AbstractAppearanceProperty, AbstractCityObjectProperty, AbstractFeatureWithLifespan,
    AsAbstractFeature, AsAbstractFeatureWithLifespan, AsAbstractFeatureWithLifespanMut,
};
use egml::model::base::Id;
use egml::model::common::ApplyTransform;
use egml::model::common::RecomputeBoundingShape;
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct CityModel {
    pub(crate) abstract_feature_with_lifespan: AbstractFeatureWithLifespan,
    city_object_members: Vec<AbstractCityObjectProperty>,
    appearance_members: Vec<AbstractAppearanceProperty>,
}

impl CityModel {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_feature_with_lifespan(AbstractFeatureWithLifespan::new(id))
    }

    pub fn from_abstract_feature_with_lifespan(
        abstract_feature_with_lifespan: AbstractFeatureWithLifespan,
    ) -> Self {
        Self {
            abstract_feature_with_lifespan,
            city_object_members: Vec::new(),
            appearance_members: Vec::new(),
        }
    }

    pub fn city_object_members(&self) -> &[AbstractCityObjectProperty] {
        &self.city_object_members
    }

    pub fn city_object_members_mut(&mut self) -> &mut Vec<AbstractCityObjectProperty> {
        &mut self.city_object_members
    }

    pub fn set_city_object_members(
        &mut self,
        city_object_members: Vec<AbstractCityObjectProperty>,
    ) {
        self.city_object_members = city_object_members;
    }

    pub fn push_city_object_member(&mut self, member: AbstractCityObjectProperty) {
        self.city_object_members.push(member);
    }

    pub fn extend_city_object_members(
        &mut self,
        members: impl IntoIterator<Item = AbstractCityObjectProperty>,
    ) {
        self.city_object_members.extend(members);
    }

    pub fn appearance_members(&self) -> &[AbstractAppearanceProperty] {
        &self.appearance_members
    }

    pub fn appearance_members_mut(&mut self) -> &mut [AbstractAppearanceProperty] {
        &mut self.appearance_members
    }

    pub fn set_appearance_members(&mut self, appearance_members: Vec<AbstractAppearanceProperty>) {
        self.appearance_members = appearance_members;
    }

    pub fn push_appearance_member(&mut self, member: AbstractAppearanceProperty) {
        self.appearance_members.push(member);
    }

    pub fn extend_appearance_members(
        &mut self,
        members: impl IntoIterator<Item = AbstractAppearanceProperty>,
    ) {
        self.appearance_members.extend(members);
    }
}

impl CityModel {
    pub fn recompute_child_bounding_shapes(&mut self) {
        self.city_object_members
            .par_iter_mut()
            .filter_map(|x| x.object_mut())
            .for_each(|current_city_object| {
                current_city_object.for_each_feature_mut(&mut |mut x: AbstractFeatureKindRefMut<
                    '_,
                >| {
                    x.recompute_bounding_shape()
                })
            });
    }

    pub fn from_city_models(city_models: Vec<Self>) -> Self {
        let abstract_feature_with_lifespan =
            AbstractFeatureWithLifespan::new(Id::generate_uuid_v4());
        let mut combined_city_model =
            CityModel::from_abstract_feature_with_lifespan(abstract_feature_with_lifespan);

        let combined_city_object_members: Vec<AbstractCityObjectProperty> = city_models
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

    pub fn iter_city_objects<'a>(
        &'a self,
    ) -> impl Iterator<Item = AbstractCityObjectKindRef<'a>> + 'a {
        self.city_object_members
            .iter()
            .flat_map(|p| p.object())
            .flat_map(|x| {
                x.iter_features().filter_map(|f| match f {
                    AbstractFeatureKindRef::AbstractFeatureWithLifespanKind(
                        AbstractFeatureWithLifespanKindRef::AbstractCityObjectKind(co),
                    ) => Some(co),
                    _ => None,
                })
            })
    }

    /// Computes the smallest envelope containing all city object members, or `None` if there
    /// are none or none of them have a resolved envelope.
    pub fn compute_city_object_members_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = self
            .iter_city_objects()
            .filter_map(|x| {
                egml::model::feature::AsAbstractFeature::bounded_by(&x)
                    .and_then(|b| b.envelope())
                    .cloned()
            })
            .collect();

        Envelope::from_envelopes(&envelopes)
    }

    pub fn filter_city_objects_by_envelope(&mut self, filter_envelope: &Envelope) {
        self.city_object_members.retain(|obj| {
            if let Some(x) = obj.object() {
                egml::model::feature::AsAbstractFeature::bounded_by(x)
                    .and_then(|x| x.envelope())
                    .is_some_and(|x| filter_envelope.contains_envelope_partially(x))
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

impl HasFeatureType for CityModel {
    fn feature_type(&self) -> FeatureType {
        FeatureType::CityModel
    }
}

impl IterFeatures for CityModel {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into())
                .chain(
                    self.city_object_members
                        .iter()
                        .flat_map(|p| p.object())
                        .flat_map(|x| x.iter_features()),
                )
                .chain(
                    self.appearance_members
                        .iter()
                        .filter_map(|p| p.object())
                        .flat_map(|x| x.iter_features()),
                ),
        )
    }
}

impl ForEachFeatureMut for CityModel {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        for prop in &mut self.city_object_members {
            if let Some(x) = prop.object_mut() {
                x.for_each_feature_mut(f);
            }
        }
        for prop in &mut self.appearance_members {
            if let Some(x) = prop.object_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }
}

impl ApplyTransform for CityModel {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.city_object_members
            .par_iter_mut()
            .filter_map(|x| x.object_mut())
            .for_each(|x| x.apply_transform(m));
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.city_object_members
            .par_iter_mut()
            .filter_map(|x| x.object_mut())
            .for_each(|x| x.apply_isometry(isometry));
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.city_object_members
            .par_iter_mut()
            .filter_map(|x| x.object_mut())
            .for_each(|x| x.apply_translation(vector));
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.city_object_members
            .par_iter_mut()
            .filter_map(|x| x.object_mut())
            .for_each(|x| x.apply_rotation(rotation));
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.city_object_members
            .par_iter_mut()
            .filter_map(|x| x.object_mut())
            .for_each(|x| x.apply_scale(scale));
    }
}
