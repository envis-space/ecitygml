use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::{AbstractFeatureKindRef, AbstractFeatureKindRefMut};
use crate::model::core::{
    AbstractFeatureWithLifespan, AsAbstractFeatureWithLifespan, AsAbstractFeatureWithLifespanMut,
};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

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
                &<$type as $crate::model::core::AsAbstractAppearance>::abstract_appearance(self)
                    .abstract_feature_with_lifespan
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
                &mut <$type as $crate::model::core::AsAbstractAppearanceMut>::abstract_appearance_mut(self).abstract_feature_with_lifespan
            }
        }
    };
}

impl_abstract_appearance_traits!(AbstractAppearance);
impl_abstract_appearance_mut_traits!(AbstractAppearance);

impl IterFeatures for AbstractAppearance {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(std::iter::empty())
    }
}

impl ForEachFeatureMut for AbstractAppearance {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, _f: &mut F) {}
}

impl ComputeEnvelope for AbstractAppearance {
    fn compute_envelope(&self) -> Option<Envelope> {
        None
    }
}

impl ApplyTransform for AbstractAppearance {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_feature_with_lifespan.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_feature_with_lifespan.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_feature_with_lifespan
            .apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_feature_with_lifespan.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_feature_with_lifespan.apply_scale(scale);
    }
}
