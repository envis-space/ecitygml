use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::core::{
    AbstractThematicSurface, AsAbstractThematicSurface, AsAbstractThematicSurfaceMut,
};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractFillingSurface {
    pub(crate) abstract_thematic_surface: AbstractThematicSurface,
}

impl AbstractFillingSurface {
    pub fn new(abstract_thematic_surface: AbstractThematicSurface) -> Self {
        Self {
            abstract_thematic_surface,
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        self.abstract_thematic_surface.iter_features()
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_thematic_surface.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_thematic_surface.compute_envelope()
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_thematic_surface.apply_transform(m);
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
                use $crate::model::construction::AsAbstractFillingSurface;
                &self.abstract_filling_surface().abstract_thematic_surface
            }
        }

        impl $crate::model::core::AsAbstractThematicSurfaceMut for $type {
            fn abstract_thematic_surface_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractThematicSurface {
                use $crate::model::construction::AsAbstractFillingSurfaceMut;
                &mut self
                    .abstract_filling_surface_mut()
                    .abstract_thematic_surface
            }
        }
    };
}

impl_abstract_filling_surface_traits!(AbstractFillingSurface);
