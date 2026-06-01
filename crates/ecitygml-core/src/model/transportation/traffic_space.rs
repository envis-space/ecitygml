use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::core::{
    AbstractUnoccupiedSpace, AsAbstractFeatureMut, AsAbstractUnoccupiedSpace,
    AsAbstractUnoccupiedSpaceMut,
};
use crate::model::transportation::TrafficDirectionValue;
use crate::model::transportation::granularity_value::GranularityValue;
use egml::model::basic::Code;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct TrafficSpace {
    pub(crate) abstract_unoccupied_space: AbstractUnoccupiedSpace,
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
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
            granularity,
            traffic_direction: None,
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_unoccupied_space.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_unoccupied_space.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_unoccupied_space.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_unoccupied_space.apply_transform(m);
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

impl<'a> From<&'a TrafficSpace> for FeatureRef<'a> {
    fn from(item: &'a TrafficSpace) -> Self {
        FeatureRef::TrafficSpace(item)
    }
}

impl<'a> From<&'a mut TrafficSpace> for FeatureRefMut<'a> {
    fn from(item: &'a mut TrafficSpace) -> Self {
        FeatureRefMut::TrafficSpace(item)
    }
}
