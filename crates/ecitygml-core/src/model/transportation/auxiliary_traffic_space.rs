use crate::model::core::{
    AbstractSpace, AsAbstractFeature, AsAbstractFeatureMut, AsAbstractSpace, AsAbstractSpaceMut,
    AsAbstractThematicSurfaceMut, CityObjectKind, CityObjectRef,
};
use crate::model::transportation::{AuxiliaryTrafficArea, GranularityValue};
use crate::operations::{Visitable, Visitor};
use egml::model::basic::Code;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AuxiliaryTrafficSpace {
    pub(crate) abstract_space: AbstractSpace,
    pub auxiliary_traffic_area: Vec<AuxiliaryTrafficArea>, // this should be located in boundaries the space struct
    pub(crate) function: Vec<Code>,
    pub(crate) usage: Vec<Code>,
    pub(crate) granularity: GranularityValue,
}

impl AuxiliaryTrafficSpace {
    pub fn new(abstract_space: AbstractSpace, granularity: GranularityValue) -> Self {
        Self {
            abstract_space,
            auxiliary_traffic_area: Vec::new(),
            function: Vec::new(),
            usage: Vec::new(),
            granularity,
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::AuxiliaryTrafficSpace(self)).chain(
            self.auxiliary_traffic_area
                .iter()
                .flat_map(|x| x.iter_city_object()),
        )
    }

    pub fn refresh_bounded_by_recursive(&mut self) {
        self.auxiliary_traffic_area
            .iter_mut()
            .for_each(|x| x.refresh_bounded_by());

        let own_envelope = self.compute_envelope();
        let envelopes: Vec<&Envelope> = own_envelope
            .as_ref()
            .into_iter()
            .chain(
                self.auxiliary_traffic_area
                    .iter()
                    .filter_map(|x| x.bounded_by()),
            )
            .collect();

        self.set_bounded_by(Envelope::from_envelopes(&envelopes));
    }

    pub fn apply_transform_recursive(&mut self, m: &Isometry3<f64>) {
        self.abstract_space.apply_transform(m);

        self.auxiliary_traffic_area
            .iter_mut()
            .for_each(|x| x.apply_transform(m));
    }

    pub fn function(&self) -> &Vec<Code> {
        &self.function
    }

    pub fn set_function(&mut self, function: Vec<Code>) {
        self.function = function;
    }

    pub fn usage(&self) -> &Vec<Code> {
        &self.usage
    }

    pub fn set_usage(&mut self, usage: Vec<Code>) {
        self.usage = usage;
    }

    pub fn granularity(&self) -> &GranularityValue {
        &self.granularity
    }

    pub fn set_granularity(&mut self, granularity: GranularityValue) {
        self.granularity = granularity;
    }
}

impl AsAbstractSpace for AuxiliaryTrafficSpace {
    fn abstract_space(&self) -> &AbstractSpace {
        &self.abstract_space
    }
}

impl AsAbstractSpaceMut for AuxiliaryTrafficSpace {
    fn abstract_space_mut(&mut self) -> &mut AbstractSpace {
        &mut self.abstract_space
    }
}

crate::impl_abstract_space_traits!(AuxiliaryTrafficSpace);

impl From<AuxiliaryTrafficSpace> for CityObjectKind {
    fn from(item: AuxiliaryTrafficSpace) -> Self {
        CityObjectKind::AuxiliaryTrafficSpace(item)
    }
}

impl Visitable for AuxiliaryTrafficSpace {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_auxiliary_traffic_space(self);
        self.auxiliary_traffic_area
            .iter()
            .for_each(|x| x.accept(visitor));
    }
}
