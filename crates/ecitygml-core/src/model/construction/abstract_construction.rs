use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractOccupiedSpace, AsAbstractOccupiedSpace, AsAbstractOccupiedSpaceMut, Occupancy,
};
use chrono::NaiveDate;
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractConstruction {
    pub(crate) abstract_occupied_space: AbstractOccupiedSpace,
    date_of_construction: Option<NaiveDate>,
    date_of_demolition: Option<NaiveDate>,
    occupancies: Vec<Occupancy>,
}

impl AbstractConstruction {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_occupied_space(AbstractOccupiedSpace::new(id))
    }

    pub fn from_abstract_occupied_space(abstract_occupied_space: AbstractOccupiedSpace) -> Self {
        Self {
            abstract_occupied_space,
            date_of_construction: None,
            date_of_demolition: None,
            occupancies: Vec::new(),
        }
    }
}

pub trait AsAbstractConstruction: AsAbstractOccupiedSpace {
    fn abstract_construction(&self) -> &AbstractConstruction;

    fn date_of_construction(&self) -> Option<&NaiveDate> {
        self.abstract_construction().date_of_construction.as_ref()
    }

    fn date_of_demolition(&self) -> Option<&NaiveDate> {
        self.abstract_construction().date_of_demolition.as_ref()
    }

    fn occupancies(&self) -> &[Occupancy] {
        self.abstract_construction().occupancies.as_ref()
    }
}

pub trait AsAbstractConstructionMut: AsAbstractOccupiedSpaceMut + AsAbstractConstruction {
    fn abstract_construction_mut(&mut self) -> &mut AbstractConstruction;

    fn set_date_of_construction(&mut self, date_of_construction: Option<NaiveDate>) {
        self.abstract_construction_mut().date_of_construction = date_of_construction;
    }

    fn set_date_of_demolition(&mut self, date_of_demolition: Option<NaiveDate>) {
        self.abstract_construction_mut().date_of_demolition = date_of_demolition;
    }

    fn occupancies_mut(&mut self) -> &mut Vec<Occupancy> {
        &mut self.abstract_construction_mut().occupancies
    }

    fn set_occupancies(&mut self, values: Vec<Occupancy>) {
        self.abstract_construction_mut().occupancies = values;
    }

    fn push_occupancy(&mut self, occupancy: Occupancy) {
        self.abstract_construction_mut().occupancies.push(occupancy);
    }

    fn extend_occupancies(&mut self, occupancies: impl IntoIterator<Item = Occupancy>) {
        self.abstract_construction_mut()
            .occupancies
            .extend(occupancies);
    }
}

impl AsAbstractConstruction for AbstractConstruction {
    fn abstract_construction(&self) -> &AbstractConstruction {
        self
    }
}

impl AsAbstractConstructionMut for AbstractConstruction {
    fn abstract_construction_mut(&mut self) -> &mut AbstractConstruction {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_construction_traits {
    ($type:ty) => {
        $crate::impl_abstract_occupied_space_traits!($type);

        impl $crate::model::core::AsAbstractOccupiedSpace for $type {
            fn abstract_occupied_space(&self) -> &$crate::model::core::AbstractOccupiedSpace {
                &<$type as $crate::model::construction::AsAbstractConstruction>::abstract_construction(self).abstract_occupied_space
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_construction_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_occupied_space_mut_traits!($type);

        impl $crate::model::core::AsAbstractOccupiedSpaceMut for $type {
            fn abstract_occupied_space_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractOccupiedSpace {
                &mut <$type as $crate::model::construction::AsAbstractConstructionMut>::abstract_construction_mut(self).abstract_occupied_space
            }
        }
    };
}

impl_abstract_construction_traits!(AbstractConstruction);
impl_abstract_construction_mut_traits!(AbstractConstruction);

impl IterFeatures for AbstractConstruction {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        self.abstract_occupied_space.iter_features()
    }
}

impl ForEachFeatureMut for AbstractConstruction {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_occupied_space.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for AbstractConstruction {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_occupied_space.compute_envelope()
    }
}

impl ApplyTransform for AbstractConstruction {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_occupied_space.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_occupied_space.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_occupied_space.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_occupied_space.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_occupied_space.apply_scale(scale);
    }
}
