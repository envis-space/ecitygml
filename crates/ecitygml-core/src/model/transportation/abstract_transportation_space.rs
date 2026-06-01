use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::core::{
    AbstractUnoccupiedSpace, AsAbstractUnoccupiedSpace, AsAbstractUnoccupiedSpaceMut,
};
use crate::model::transportation::marking_property::MarkingProperty;
use crate::model::transportation::{
    AuxiliaryTrafficSpaceProperty, TrafficDirectionValue, TrafficSpaceProperty,
};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractTransportationSpace {
    pub(crate) abstract_unoccupied_space: AbstractUnoccupiedSpace,
    pub traffic_direction: Option<TrafficDirectionValue>,
    pub traffic_spaces: Vec<TrafficSpaceProperty>,
    pub auxiliary_traffic_spaces: Vec<AuxiliaryTrafficSpaceProperty>,
    pub markings: Vec<MarkingProperty>,
}

impl AbstractTransportationSpace {
    pub fn new(abstract_unoccupied_space: AbstractUnoccupiedSpace) -> Self {
        Self {
            abstract_unoccupied_space,
            traffic_direction: None,
            traffic_spaces: Vec::new(),
            auxiliary_traffic_spaces: Vec::new(),
            markings: Vec::new(),
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        self.abstract_unoccupied_space
            .iter_features()
            .chain(
                self.traffic_spaces
                    .iter()
                    .flat_map(|x| x.object.as_ref())
                    .flat_map(|x| x.iter_features()),
            )
            .chain(
                self.auxiliary_traffic_spaces
                    .iter()
                    .flat_map(|x| x.object.as_ref())
                    .flat_map(|x| x.iter_features()),
            )
            .chain(
                self.markings
                    .iter()
                    .flat_map(|x| x.object.as_ref())
                    .flat_map(|x| x.iter_features()),
            )
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_unoccupied_space.for_each_feature_mut(f);
        for prop in &mut self.traffic_spaces {
            if let Some(x) = prop.object.as_mut() {
                x.for_each_feature_mut(f);
            }
        }
        for prop in &mut self.auxiliary_traffic_spaces {
            if let Some(x) = prop.object.as_mut() {
                x.for_each_feature_mut(f);
            }
        }
        for prop in &mut self.markings {
            if let Some(x) = prop.object.as_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_unoccupied_space.compute_envelope()
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_unoccupied_space.apply_transform(m);

        self.traffic_spaces
            .iter_mut()
            .flat_map(|x| x.object.as_mut())
            .for_each(|x| x.apply_transform(m));
        self.auxiliary_traffic_spaces
            .iter_mut()
            .flat_map(|x| x.object.as_mut())
            .for_each(|x| x.apply_transform(m));
        self.markings
            .iter_mut()
            .flat_map(|x| x.object.as_mut())
            .for_each(|x| x.apply_transform(m));
    }
}

pub trait AsAbstractTransportationSpace: AsAbstractUnoccupiedSpace {
    fn abstract_transportation_space(&self) -> &AbstractTransportationSpace;

    fn traffic_direction(&self) -> &Option<TrafficDirectionValue> {
        &self.abstract_transportation_space().traffic_direction
    }

    fn traffic_spaces(&self) -> &Vec<TrafficSpaceProperty> {
        &self.abstract_transportation_space().traffic_spaces
    }

    fn auxiliary_traffic_spaces(&self) -> &Vec<AuxiliaryTrafficSpaceProperty> {
        &self
            .abstract_transportation_space()
            .auxiliary_traffic_spaces
    }

    fn markings(&self) -> &Vec<MarkingProperty> {
        &self.abstract_transportation_space().markings
    }
}

pub trait AsAbstractTransportationSpaceMut:
    AsAbstractUnoccupiedSpaceMut + AsAbstractTransportationSpace
{
    fn abstract_transportation_space_mut(&mut self) -> &mut AbstractTransportationSpace;

    fn set_traffic_direction(&mut self, value: Option<TrafficDirectionValue>) {
        self.abstract_transportation_space_mut().traffic_direction = value;
    }

    fn set_traffic_spaces(&mut self, values: Vec<TrafficSpaceProperty>) {
        self.abstract_transportation_space_mut().traffic_spaces = values;
    }

    fn set_auxiliary_traffic_spaces(&mut self, values: Vec<AuxiliaryTrafficSpaceProperty>) {
        self.abstract_transportation_space_mut()
            .auxiliary_traffic_spaces = values;
    }

    fn markings_mut(&mut self) -> &mut Vec<MarkingProperty> {
        &mut self.abstract_transportation_space_mut().markings
    }

    fn set_markings(&mut self, values: Vec<MarkingProperty>) {
        self.abstract_transportation_space_mut().markings = values;
    }
}

impl AsAbstractTransportationSpace for AbstractTransportationSpace {
    fn abstract_transportation_space(&self) -> &AbstractTransportationSpace {
        self
    }
}

impl AsAbstractTransportationSpaceMut for AbstractTransportationSpace {
    fn abstract_transportation_space_mut(&mut self) -> &mut AbstractTransportationSpace {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_transportation_space_traits {
    ($type:ty) => {
        $crate::impl_abstract_unoccupied_space_traits!($type);

        impl $crate::model::core::AsAbstractUnoccupiedSpace for $type {
            fn abstract_unoccupied_space(&self) -> &$crate::model::core::AbstractUnoccupiedSpace {
                use $crate::model::transportation::AsAbstractTransportationSpace;
                &self
                    .abstract_transportation_space()
                    .abstract_unoccupied_space
            }
        }

        impl $crate::model::core::AsAbstractUnoccupiedSpaceMut for $type {
            fn abstract_unoccupied_space_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractUnoccupiedSpace {
                use $crate::model::transportation::AsAbstractTransportationSpaceMut;
                &mut self
                    .abstract_transportation_space_mut()
                    .abstract_unoccupied_space
            }
        }
    };
}

impl_abstract_transportation_space_traits!(AbstractTransportationSpace);
