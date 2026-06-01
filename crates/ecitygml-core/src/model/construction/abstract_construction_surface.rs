use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::construction::filling_surface_property::FillingSurfaceProperty;
use crate::model::core::{
    AbstractThematicSurface, AsAbstractThematicSurface, AsAbstractThematicSurfaceMut,
};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractConstructionSurface {
    pub abstract_thematic_surface: AbstractThematicSurface,
    pub filling_surfaces: Vec<FillingSurfaceProperty>,
}

impl AbstractConstructionSurface {
    pub fn new(abstract_thematic_surface: AbstractThematicSurface) -> Self {
        Self {
            abstract_thematic_surface,
            filling_surfaces: Vec::new(),
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        self.abstract_thematic_surface.iter_features().chain(
            self.filling_surfaces
                .iter()
                .filter_map(|x| x.object.as_ref())
                .flat_map(|x| x.iter_features()),
        )
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_thematic_surface.for_each_feature_mut(f);
        for prop in &mut self.filling_surfaces {
            if let Some(x) = prop.object.as_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_thematic_surface.compute_envelope()
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_thematic_surface.apply_transform(m);
        for prop in &mut self.filling_surfaces {
            if let Some(x) = prop.object.as_mut() {
                x.apply_transform(m);
            }
        }
    }
}

pub trait AsAbstractConstructionSurface: AsAbstractThematicSurface {
    fn abstract_construction_surface(&self) -> &AbstractConstructionSurface;

    fn filling_surfaces(&self) -> &Vec<FillingSurfaceProperty> {
        self.abstract_construction_surface()
            .filling_surfaces
            .as_ref()
    }
}

pub trait AsAbstractConstructionSurfaceMut:
    AsAbstractThematicSurfaceMut + AsAbstractThematicSurface
{
    fn abstract_construction_surface_mut(&mut self) -> &mut AbstractConstructionSurface;

    fn set_filling_surfaces(&mut self, value: Vec<FillingSurfaceProperty>) {
        self.abstract_construction_surface_mut().filling_surfaces = value;
    }
}

impl AsAbstractConstructionSurface for AbstractConstructionSurface {
    fn abstract_construction_surface(&self) -> &AbstractConstructionSurface {
        self
    }
}

impl AsAbstractConstructionSurfaceMut for AbstractConstructionSurface {
    fn abstract_construction_surface_mut(&mut self) -> &mut AbstractConstructionSurface {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_construction_surface_traits {
    ($type:ty) => {
        $crate::impl_abstract_thematic_surface_traits!($type);

        impl $crate::model::core::AsAbstractThematicSurface for $type {
            fn abstract_thematic_surface(&self) -> &$crate::model::core::AbstractThematicSurface {
                use $crate::model::construction::AsAbstractConstructionSurface;
                &self
                    .abstract_construction_surface()
                    .abstract_thematic_surface
            }
        }

        impl $crate::model::core::AsAbstractThematicSurfaceMut for $type {
            fn abstract_thematic_surface_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractThematicSurface {
                use $crate::model::construction::AsAbstractConstructionSurfaceMut;
                &mut self
                    .abstract_construction_surface_mut()
                    .abstract_thematic_surface
            }
        }
    };
}

impl_abstract_construction_surface_traits!(AbstractConstructionSurface);
