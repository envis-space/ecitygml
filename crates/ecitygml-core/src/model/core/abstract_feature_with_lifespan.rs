use crate::model::core::{AbstractFeature, AsAbstractFeature, AsAbstractFeatureMut};
use chrono::{DateTime, FixedOffset};
use egml::model::base::Id;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractFeatureWithLifespan {
    pub(crate) abstract_feature: AbstractFeature,
    creation_date: Option<DateTime<FixedOffset>>,
    termination_date: Option<DateTime<FixedOffset>>,
    valid_from: Option<DateTime<FixedOffset>>,
    valid_to: Option<DateTime<FixedOffset>>,
}

impl AbstractFeatureWithLifespan {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_feature(AbstractFeature::new(id))
    }

    pub fn from_abstract_feature(abstract_feature: AbstractFeature) -> Self {
        Self {
            abstract_feature,
            creation_date: None,
            termination_date: None,
            valid_from: None,
            valid_to: None,
        }
    }
}
impl AbstractFeatureWithLifespan {
    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_feature.apply_transform(m);
    }
}

pub trait AsAbstractFeatureWithLifespan: AsAbstractFeature {
    fn abstract_feature_with_lifespan(&self) -> &AbstractFeatureWithLifespan;

    fn creation_date(&self) -> Option<&DateTime<FixedOffset>> {
        self.abstract_feature_with_lifespan().creation_date.as_ref()
    }

    fn termination_date(&self) -> Option<&DateTime<FixedOffset>> {
        self.abstract_feature_with_lifespan()
            .termination_date
            .as_ref()
    }

    fn valid_from(&self) -> Option<&DateTime<FixedOffset>> {
        self.abstract_feature_with_lifespan().valid_from.as_ref()
    }

    fn valid_to(&self) -> Option<&DateTime<FixedOffset>> {
        self.abstract_feature_with_lifespan().valid_to.as_ref()
    }
}

pub trait AsAbstractFeatureWithLifespanMut:
    AsAbstractFeatureMut + AsAbstractFeatureWithLifespan
{
    fn abstract_feature_with_lifespan_mut(&mut self) -> &mut AbstractFeatureWithLifespan;

    fn set_creation_date(&mut self, creation_date: Option<DateTime<FixedOffset>>) {
        self.abstract_feature_with_lifespan_mut().creation_date = creation_date;
    }

    fn set_termination_date(&mut self, termination_date: Option<DateTime<FixedOffset>>) {
        self.abstract_feature_with_lifespan_mut().termination_date = termination_date;
    }

    fn set_valid_from(&mut self, valid_from: Option<DateTime<FixedOffset>>) {
        self.abstract_feature_with_lifespan_mut().valid_from = valid_from;
    }

    fn set_valid_to(&mut self, valid_to: Option<DateTime<FixedOffset>>) {
        self.abstract_feature_with_lifespan_mut().valid_to = valid_to;
    }
}

impl AsAbstractFeatureWithLifespan for AbstractFeatureWithLifespan {
    fn abstract_feature_with_lifespan(&self) -> &AbstractFeatureWithLifespan {
        self
    }
}

impl AsAbstractFeatureWithLifespanMut for AbstractFeatureWithLifespan {
    fn abstract_feature_with_lifespan_mut(&mut self) -> &mut AbstractFeatureWithLifespan {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_feature_with_lifespan_traits {
    ($type:ty) => {
        impl $crate::model::core::AsAbstractFeature for $type {
            fn abstract_feature(&self) -> &$crate::model::core::AbstractFeature {
                use $crate::model::core::AsAbstractFeatureWithLifespan;
                &self.abstract_feature_with_lifespan().abstract_feature
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_feature_with_lifespan_mut_traits {
    ($type:ty) => {
        impl $crate::model::core::AsAbstractFeatureMut for $type {
            fn abstract_feature_mut(&mut self) -> &mut $crate::model::core::AbstractFeature {
                use $crate::model::core::AsAbstractFeatureWithLifespanMut;
                &mut self.abstract_feature_with_lifespan_mut().abstract_feature
            }
        }
    };
}

impl_abstract_feature_with_lifespan_traits!(AbstractFeatureWithLifespan);
impl_abstract_feature_with_lifespan_mut_traits!(AbstractFeatureWithLifespan);
