use crate::model::core::refs::{FeatureKindRef, FeatureKindRefMut};
use crate::model::core::{
    AbstractFeatureWithLifespan, AsAbstractFeatureWithLifespan, AsAbstractFeatureWithLifespanMut,
};
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractAppearance {
    pub(crate) abstract_feature_with_lifespan: AbstractFeatureWithLifespan,
}

impl AbstractAppearance {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_feature_with_lifespan(AbstractFeatureWithLifespan::new(id))
    }

    pub fn from_abstract_feature_with_lifespan(
        abstract_feature_with_lifespan: AbstractFeatureWithLifespan,
    ) -> Self {
        Self {
            abstract_feature_with_lifespan,
        }
    }
}
impl AbstractAppearance {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        std::iter::empty()
    }
    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, _f: &mut F) {}

    pub fn compute_envelope(&self) -> Option<Envelope> {
        None
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_feature_with_lifespan.apply_transform(m);
    }
}

pub trait AsAbstractAppearance: AsAbstractFeatureWithLifespan {
    fn abstract_appearance(&self) -> &AbstractAppearance;
}

pub trait AsAbstractAppearanceMut: AsAbstractFeatureWithLifespanMut + AsAbstractAppearance {
    fn abstract_appearance_mut(&mut self) -> &mut AbstractAppearance;
}

impl AsAbstractAppearance for AbstractAppearance {
    fn abstract_appearance(&self) -> &AbstractAppearance {
        self
    }
}

impl AsAbstractAppearanceMut for AbstractAppearance {
    fn abstract_appearance_mut(&mut self) -> &mut AbstractAppearance {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_appearance_traits {
    ($type:ty) => {
        $crate::impl_abstract_feature_with_lifespan_traits!($type);

        impl $crate::model::core::AsAbstractFeatureWithLifespan for $type {
            fn abstract_feature_with_lifespan(
                &self,
            ) -> &$crate::model::core::AbstractFeatureWithLifespan {
                use $crate::model::core::AsAbstractAppearance;
                &self.abstract_appearance().abstract_feature_with_lifespan
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_appearance_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_feature_with_lifespan_mut_traits!($type);

        impl $crate::model::core::AsAbstractFeatureWithLifespanMut for $type {
            fn abstract_feature_with_lifespan_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractFeatureWithLifespan {
                use $crate::model::core::AsAbstractAppearanceMut;
                &mut self
                    .abstract_appearance_mut()
                    .abstract_feature_with_lifespan
            }
        }
    };
}

impl_abstract_appearance_traits!(AbstractAppearance);
impl_abstract_appearance_mut_traits!(AbstractAppearance);
