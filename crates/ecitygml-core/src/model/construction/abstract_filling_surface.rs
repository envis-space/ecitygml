use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractThematicSurface, AsAbstractThematicSurface, AsAbstractThematicSurfaceMut,
};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractFillingSurface {
    pub(crate) abstract_thematic_surface: AbstractThematicSurface,
}

impl AbstractFillingSurface {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_thematic_surface(AbstractThematicSurface::new(id))
    }

    pub fn from_abstract_thematic_surface(
        abstract_thematic_surface: AbstractThematicSurface,
    ) -> Self {
        Self {
            abstract_thematic_surface,
        }
    }
}

pub trait AsAbstractFillingSurface: AsAbstractThematicSurface {
    fn abstract_filling_surface(&self) -> &AbstractFillingSurface;
}

pub trait AsAbstractFillingSurfaceMut:
    AsAbstractThematicSurfaceMut + AsAbstractFillingSurface
{
    fn abstract_filling_surface_mut(&mut self) -> &mut AbstractFillingSurface;
}

impl AsAbstractFillingSurface for AbstractFillingSurface {
    fn abstract_filling_surface(&self) -> &AbstractFillingSurface {
        self
    }
}

impl AsAbstractFillingSurfaceMut for AbstractFillingSurface {
    fn abstract_filling_surface_mut(&mut self) -> &mut AbstractFillingSurface {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_filling_surface_traits {
    ($type:ty) => {
        $crate::impl_abstract_thematic_surface_traits!($type);

        impl $crate::model::core::AsAbstractThematicSurface for $type {
            fn abstract_thematic_surface(&self) -> &$crate::model::core::AbstractThematicSurface {
                &<$type as $crate::model::construction::AsAbstractFillingSurface>::abstract_filling_surface(self).abstract_thematic_surface
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_filling_surface_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_thematic_surface_mut_traits!($type);

        impl $crate::model::core::AsAbstractThematicSurfaceMut for $type {
            fn abstract_thematic_surface_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractThematicSurface {
                &mut <$type as $crate::model::construction::AsAbstractFillingSurfaceMut>::abstract_filling_surface_mut(self).abstract_thematic_surface
            }
        }
    };
}

impl_abstract_filling_surface_traits!(AbstractFillingSurface);
impl_abstract_filling_surface_mut_traits!(AbstractFillingSurface);

impl IterFeatures for AbstractFillingSurface {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        self.abstract_thematic_surface.iter_features()
    }
}

impl ForEachFeatureMut for AbstractFillingSurface {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_thematic_surface.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for AbstractFillingSurface {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_thematic_surface.compute_envelope()
    }
}

impl ApplyTransform for AbstractFillingSurface {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_thematic_surface.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_thematic_surface.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_thematic_surface.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_thematic_surface.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_thematic_surface.apply_scale(scale);
    }
}
