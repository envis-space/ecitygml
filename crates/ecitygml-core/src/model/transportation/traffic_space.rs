use crate::model::core::{
    AbstractUnoccupiedSpace, AsAbstractFeature, AsAbstractFeatureMut, AsAbstractSpace,
    AsAbstractSpaceMut, AsAbstractThematicSurfaceMut, AsAbstractUnoccupiedSpace,
    AsAbstractUnoccupiedSpaceMut, CityObjectKind, CityObjectRef,
};
use crate::model::transportation::granularity_value::GranularityValue;
use crate::model::transportation::{TrafficArea, TrafficDirectionValue};
use crate::operations::{Visitable, Visitor};
use egml::model::basic::Code;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct TrafficSpace {
    pub(crate) abstract_unoccupied_space: AbstractUnoccupiedSpace,
    pub traffic_area: Vec<TrafficArea>, // this should be located in boundaries the space struct
    pub(crate) class: Option<Code>,
    pub(crate) functions: Vec<Code>,
    pub(crate) usages: Vec<Code>,
    pub(crate) granularity: GranularityValue,
    pub(crate) traffic_direction: Option<TrafficDirectionValue>,
}

impl TrafficSpace {
    pub fn new(
        abstract_unoccupied_space: AbstractUnoccupiedSpace,
        granularity: GranularityValue,
    ) -> Self {
        Self {
            abstract_unoccupied_space,
            traffic_area: Vec::new(),
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
            granularity,
            traffic_direction: None,
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::TrafficSpace(self))
            .chain(self.traffic_area.iter().flat_map(|x| x.iter_city_object()))
    }

    pub fn refresh_bounded_by_recursive(&mut self) {
        self.traffic_area
            .iter_mut()
            .for_each(|x| x.refresh_bounded_by());

        let own_envelope = self.compute_envelope();
        let envelopes: Vec<Envelope> = own_envelope
            .as_ref()
            .into_iter()
            .chain(self.traffic_area.iter().filter_map(|x| x.bounded_by()))
            .cloned()
            .collect();

        self.set_bounded_by(Envelope::from_envelopes(&envelopes));
    }

    pub fn apply_transform_recursive(&mut self, m: &Isometry3<f64>) {
        self.abstract_unoccupied_space.apply_transform(m);

        self.traffic_area
            .iter_mut()
            .for_each(|x| x.apply_transform(m));
    }

    pub fn class(&self) -> &Option<Code> {
        &self.class
    }

    pub fn set_class(&mut self, class: Option<Code>) {
        self.class = class;
    }

    pub fn functions(&self) -> &Vec<Code> {
        &self.functions
    }

    pub fn set_functions(&mut self, functions: Vec<Code>) {
        self.functions = functions;
    }

    pub fn usages(&self) -> &Vec<Code> {
        &self.usages
    }

    pub fn set_usages(&mut self, usages: Vec<Code>) {
        self.usages = usages;
    }

    pub fn granularity(&self) -> &GranularityValue {
        &self.granularity
    }

    pub fn set_granularity(&mut self, granularity: GranularityValue) {
        self.granularity = granularity;
    }

    pub fn traffic_direction(&self) -> &Option<TrafficDirectionValue> {
        &self.traffic_direction
    }

    pub fn set_traffic_direction(&mut self, traffic_direction: Option<TrafficDirectionValue>) {
        self.traffic_direction = traffic_direction;
    }
}

impl AsAbstractUnoccupiedSpace for TrafficSpace {
    fn abstract_unoccupied_space(&self) -> &AbstractUnoccupiedSpace {
        &self.abstract_unoccupied_space
    }
}

impl AsAbstractUnoccupiedSpaceMut for TrafficSpace {
    fn abstract_unoccupied_space_mut(&mut self) -> &mut AbstractUnoccupiedSpace {
        &mut self.abstract_unoccupied_space
    }
}

crate::impl_abstract_unoccupied_space_traits!(TrafficSpace);

impl From<TrafficSpace> for CityObjectKind {
    fn from(item: TrafficSpace) -> Self {
        CityObjectKind::TrafficSpace(item)
    }
}

impl Visitable for TrafficSpace {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_traffic_space(self);
        self.traffic_area.iter().for_each(|x| x.accept(visitor));
    }
}
