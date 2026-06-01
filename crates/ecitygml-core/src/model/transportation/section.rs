use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::core::AsAbstractFeatureMut;
use crate::model::transportation::{
    AbstractTransportationSpace, AsAbstractTransportationSpace, AsAbstractTransportationSpaceMut,
};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct Section {
    pub(crate) abstract_transportation_space: AbstractTransportationSpace,
}

impl Section {
    pub fn new(abstract_transportation_space: AbstractTransportationSpace) -> Self {
        Self {
            abstract_transportation_space,
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_transportation_space.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_transportation_space.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_transportation_space.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_transportation_space.apply_transform(m);
    }
}

impl AsAbstractTransportationSpace for Section {
    fn abstract_transportation_space(&self) -> &AbstractTransportationSpace {
        &self.abstract_transportation_space
    }
}

impl AsAbstractTransportationSpaceMut for Section {
    fn abstract_transportation_space_mut(&mut self) -> &mut AbstractTransportationSpace {
        &mut self.abstract_transportation_space
    }
}

crate::impl_abstract_transportation_space_traits!(Section);

impl<'a> From<&'a Section> for FeatureRef<'a> {
    fn from(item: &'a Section) -> Self {
        FeatureRef::Section(item)
    }
}

impl<'a> From<&'a mut Section> for FeatureRefMut<'a> {
    fn from(item: &'a mut Section) -> Self {
        FeatureRefMut::Section(item)
    }
}
