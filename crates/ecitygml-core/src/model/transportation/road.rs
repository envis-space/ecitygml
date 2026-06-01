use crate::model::common::{FeatureRef, FeatureRefMut, TopLevelFeatureRef};
use crate::model::core::AsAbstractFeatureMut;
use crate::model::transportation::{
    AbstractTransportationSpace, AsAbstractTransportationSpace, AsAbstractTransportationSpaceMut,
    IntersectionProperty, SectionProperty,
};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct Road {
    pub(crate) abstract_transportation_space: AbstractTransportationSpace,
    pub sections: Vec<SectionProperty>,
    pub intersections: Vec<IntersectionProperty>,
}

impl Road {
    pub fn new(abstract_transportation_space: AbstractTransportationSpace) -> Self {
        Self {
            abstract_transportation_space,
            sections: Default::default(),
            intersections: Default::default(),
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        std::iter::once(self.into())
            .chain(self.abstract_transportation_space.iter_features())
            .chain(
                self.sections
                    .iter()
                    .flat_map(|x| x.object.as_ref())
                    .flat_map(|x| x.iter_features()),
            )
            .chain(
                self.intersections
                    .iter()
                    .flat_map(|x| x.object.as_ref())
                    .flat_map(|x| x.iter_features()),
            )
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_transportation_space.for_each_feature_mut(f);
        for prop in &mut self.sections {
            if let Some(x) = prop.object.as_mut() {
                x.for_each_feature_mut(f);
            }
        }
        for prop in &mut self.intersections {
            if let Some(x) = prop.object.as_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_transportation_space.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_transportation_space.apply_transform(m);

        self.sections
            .iter_mut()
            .flat_map(|x| x.object.as_mut())
            .for_each(|x| x.apply_transform(m));
        self.intersections
            .iter_mut()
            .flat_map(|x| x.object.as_mut())
            .for_each(|x| x.apply_transform(m));
    }
}

impl AsAbstractTransportationSpace for Road {
    fn abstract_transportation_space(&self) -> &AbstractTransportationSpace {
        &self.abstract_transportation_space
    }
}

impl AsAbstractTransportationSpaceMut for Road {
    fn abstract_transportation_space_mut(&mut self) -> &mut AbstractTransportationSpace {
        &mut self.abstract_transportation_space
    }
}

crate::impl_abstract_transportation_space_traits!(Road);

impl<'a> From<&'a Road> for FeatureRef<'a> {
    fn from(item: &'a Road) -> Self {
        FeatureRef::Road(item)
    }
}

impl<'a> From<&'a mut Road> for FeatureRefMut<'a> {
    fn from(item: &'a mut Road) -> Self {
        FeatureRefMut::Road(item)
    }
}

impl<'a> From<&'a Road> for TopLevelFeatureRef<'a> {
    fn from(item: &'a Road) -> Self {
        TopLevelFeatureRef::Road(item)
    }
}
