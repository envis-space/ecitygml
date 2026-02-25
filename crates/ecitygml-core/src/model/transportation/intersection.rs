use crate::model::core::{
    AbstractSpace, AsAbstractFeature, AsAbstractFeatureMut, AsAbstractSpace, AsAbstractSpaceMut,
    CityObjectKind, CityObjectRef,
};
use crate::model::transportation::{AuxiliaryTrafficSpace, TrafficSpace};
use crate::operations::{Visitable, Visitor};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection {
    pub(crate) abstract_space: AbstractSpace,
    pub traffic_space: Vec<TrafficSpace>,
    pub auxiliary_traffic_space: Vec<AuxiliaryTrafficSpace>,
}

impl Intersection {
    pub fn new(abstract_space: AbstractSpace) -> Self {
        Self {
            abstract_space,
            traffic_space: Vec::new(),
            auxiliary_traffic_space: Vec::new(),
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::Intersection(self))
            .chain(self.traffic_space.iter().flat_map(|x| x.iter_city_object()))
            .chain(
                self.auxiliary_traffic_space
                    .iter()
                    .flat_map(|x| x.iter_city_object()),
            )
    }

    pub fn refresh_bounded_by_recursive(&mut self) {
        self.traffic_space
            .iter_mut()
            .for_each(|x| x.refresh_bounded_by_recursive());
        self.auxiliary_traffic_space
            .iter_mut()
            .for_each(|x| x.refresh_bounded_by_recursive());

        let own_envelope = self.compute_envelope();
        let envelopes: Vec<&Envelope> = own_envelope
            .as_ref()
            .into_iter()
            .chain(self.traffic_space.iter().filter_map(|x| x.bounded_by()))
            .chain(
                self.auxiliary_traffic_space
                    .iter()
                    .filter_map(|x| x.bounded_by()),
            )
            .collect();

        self.set_bounded_by(Envelope::from_envelopes(&envelopes));
    }

    pub fn apply_transform_recursive(&mut self, m: &Isometry3<f64>) {
        self.abstract_space.apply_transform(m);

        self.traffic_space
            .iter_mut()
            .for_each(|x| x.apply_transform_recursive(m));
        self.auxiliary_traffic_space
            .iter_mut()
            .for_each(|x| x.apply_transform_recursive(m));
    }
}

impl AsAbstractSpace for Intersection {
    fn abstract_space(&self) -> &AbstractSpace {
        &self.abstract_space
    }
}

impl AsAbstractSpaceMut for Intersection {
    fn abstract_space_mut(&mut self) -> &mut AbstractSpace {
        &mut self.abstract_space
    }
}

crate::impl_abstract_space_traits!(Intersection);

impl From<Intersection> for CityObjectKind {
    fn from(item: Intersection) -> Self {
        CityObjectKind::Intersection(item)
    }
}

impl Visitable for Intersection {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_intersection(self);
        self.traffic_space.iter().for_each(|x| x.accept(visitor));
        self.auxiliary_traffic_space
            .iter()
            .for_each(|x| x.accept(visitor));
    }
}
