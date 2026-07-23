use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::construction::abstract_filling_surface_property::AbstractFillingSurfaceProperty;
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
pub struct AbstractConstructionSurface {
    pub(crate) abstract_thematic_surface: AbstractThematicSurface,
    filling_surfaces: Vec<AbstractFillingSurfaceProperty>,
}

impl AbstractConstructionSurface {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_thematic_surface(AbstractThematicSurface::new(id))
    }

    pub fn from_abstract_thematic_surface(
        abstract_thematic_surface: AbstractThematicSurface,
    ) -> Self {
        Self {
            abstract_thematic_surface,
            filling_surfaces: Vec::new(),
        }
    }
}

pub trait AsAbstractConstructionSurface: AsAbstractThematicSurface {
    fn abstract_construction_surface(&self) -> &AbstractConstructionSurface;

    fn filling_surfaces(&self) -> &[AbstractFillingSurfaceProperty] {
        self.abstract_construction_surface()
            .filling_surfaces
            .as_ref()
    }
}

pub trait AsAbstractConstructionSurfaceMut:
    AsAbstractThematicSurfaceMut + AsAbstractThematicSurface
{
    fn abstract_construction_surface_mut(&mut self) -> &mut AbstractConstructionSurface;

    fn filling_surfaces_mut(&mut self) -> &mut [AbstractFillingSurfaceProperty] {
        &mut self.abstract_construction_surface_mut().filling_surfaces
    }

    fn set_filling_surfaces(&mut self, value: Vec<AbstractFillingSurfaceProperty>) {
        self.abstract_construction_surface_mut().filling_surfaces = value;
    }

    fn push_filling_surface(&mut self, surface: AbstractFillingSurfaceProperty) {
        self.abstract_construction_surface_mut()
            .filling_surfaces
            .push(surface);
    }

    fn extend_filling_surfaces(
        &mut self,
        surfaces: impl IntoIterator<Item = AbstractFillingSurfaceProperty>,
    ) {
        self.abstract_construction_surface_mut()
            .filling_surfaces
            .extend(surfaces);
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
                &<$type as $crate::model::construction::AsAbstractConstructionSurface>::abstract_construction_surface(self).abstract_thematic_surface
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_construction_surface_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_thematic_surface_mut_traits!($type);

        impl $crate::model::core::AsAbstractThematicSurfaceMut for $type {
            fn abstract_thematic_surface_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractThematicSurface {
                &mut <$type as $crate::model::construction::AsAbstractConstructionSurfaceMut>::abstract_construction_surface_mut(self).abstract_thematic_surface
            }
        }
    };
}

impl_abstract_construction_surface_traits!(AbstractConstructionSurface);
impl_abstract_construction_surface_mut_traits!(AbstractConstructionSurface);

impl IterFeatures for AbstractConstructionSurface {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            self.abstract_thematic_surface.iter_features().chain(
                self.filling_surfaces
                    .iter()
                    .filter_map(|x| x.object())
                    .flat_map(|x| x.iter_features()),
            ),
        )
    }
}

impl ForEachFeatureMut for AbstractConstructionSurface {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_thematic_surface.for_each_feature_mut(f);
        for prop in &mut self.filling_surfaces {
            if let Some(x) = prop.object_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }
}

impl ComputeEnvelope for AbstractConstructionSurface {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_thematic_surface.compute_envelope()
    }
}

impl ApplyTransform for AbstractConstructionSurface {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_thematic_surface.apply_transform(m);
        for prop in &mut self.filling_surfaces {
            if let Some(x) = prop.object_mut() {
                x.apply_transform(m);
            }
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_thematic_surface.apply_isometry(isometry);
        for prop in &mut self.filling_surfaces {
            if let Some(x) = prop.object_mut() {
                x.apply_isometry(isometry);
            }
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_thematic_surface.apply_translation(vector);
        for prop in &mut self.filling_surfaces {
            if let Some(x) = prop.object_mut() {
                x.apply_translation(vector);
            }
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_thematic_surface.apply_rotation(rotation);
        for prop in &mut self.filling_surfaces {
            if let Some(x) = prop.object_mut() {
                x.apply_rotation(rotation);
            }
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_thematic_surface.apply_scale(scale);
        for prop in &mut self.filling_surfaces {
            if let Some(x) = prop.object_mut() {
                x.apply_scale(scale);
            }
        }
    }
}
