use crate::model::core::{
    AbstractSpace, AsAbstractFeature, AsAbstractFeatureMut, AsAbstractSpace, AsAbstractSpaceMut,
    CityObjectKind, CityObjectRef,
};
use crate::model::transportation::{Intersection, Section};
use crate::operations::{Visitable, Visitor};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct Road {
    pub(crate) abstract_space: AbstractSpace,
    pub section: Vec<Section>,
    pub intersection: Vec<Intersection>,
}

impl Road {
    pub fn new(abstract_space: AbstractSpace) -> Self {
        Self {
            abstract_space,
            section: Default::default(),
            intersection: Default::default(),
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::Road(self))
            .chain(self.section.iter().flat_map(|x| x.iter_city_object()))
            .chain(self.intersection.iter().flat_map(|x| x.iter_city_object()))
    }

    pub fn refresh_bounded_by_recursive(&mut self) {
        self.section
            .iter_mut()
            .for_each(|x| x.refresh_bounded_by_recursive());
        self.intersection
            .iter_mut()
            .for_each(|x| x.refresh_bounded_by_recursive());

        let own_envelope = self.compute_envelope();
        let envelopes: Vec<&Envelope> = own_envelope
            .as_ref()
            .into_iter()
            .chain(self.section.iter().filter_map(|x| x.bounded_by()))
            .chain(self.intersection.iter().filter_map(|x| x.bounded_by()))
            .collect();

        self.set_bounded_by(Envelope::from_envelopes(&envelopes));
    }

    pub fn apply_transform_recursive(&mut self, m: &Isometry3<f64>) {
        self.abstract_space.apply_transform(m);

        self.section
            .iter_mut()
            .for_each(|x| x.apply_transform_recursive(m));
        self.intersection
            .iter_mut()
            .for_each(|x| x.apply_transform_recursive(m));
    }
}

impl AsAbstractSpace for Road {
    fn abstract_space(&self) -> &AbstractSpace {
        &self.abstract_space
    }
}

impl AsAbstractSpaceMut for Road {
    fn abstract_space_mut(&mut self) -> &mut AbstractSpace {
        &mut self.abstract_space
    }
}

crate::impl_abstract_space_traits!(Road);

impl From<Road> for CityObjectKind {
    fn from(item: Road) -> Self {
        CityObjectKind::Road(item)
    }
}

impl Visitable for Road {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_road(self);
        self.section.iter().for_each(|x| x.accept(visitor));
        self.intersection.iter().for_each(|x| x.accept(visitor));
    }
}
