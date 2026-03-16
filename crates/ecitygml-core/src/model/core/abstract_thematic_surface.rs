use crate::model::common::LevelOfDetail;
use crate::model::core::{
    AbstractSpaceBoundary, AsAbstractCityObject, AsAbstractSpaceBoundary,
    AsAbstractSpaceBoundaryMut,
};
use crate::model::core::{AsAbstractCityObjectMut, AsAbstractFeature};
use egml::model::geometry::Envelope;
use egml::model::geometry::aggregates::MultiSurface;
use nalgebra::Isometry3;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractThematicSurface {
    pub(crate) abstract_space_boundary: AbstractSpaceBoundary,
    pub(crate) lod0_multi_surface: Option<MultiSurface>,
    pub(crate) lod1_multi_surface: Option<MultiSurface>,
    pub(crate) lod2_multi_surface: Option<MultiSurface>,
    pub(crate) lod3_multi_surface: Option<MultiSurface>,
}

impl AbstractThematicSurface {
    pub fn new(abstract_space_boundary: AbstractSpaceBoundary) -> Self {
        Self {
            abstract_space_boundary,
            lod0_multi_surface: None,
            lod1_multi_surface: None,
            lod2_multi_surface: None,
            lod3_multi_surface: None,
        }
    }
}

pub trait AsAbstractThematicSurface: AsAbstractSpaceBoundary {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface;

    fn lod0_multi_surface(&self) -> Option<&MultiSurface> {
        self.abstract_thematic_surface().lod0_multi_surface.as_ref()
    }

    fn lod1_multi_surface(&self) -> Option<&MultiSurface> {
        self.abstract_thematic_surface().lod1_multi_surface.as_ref()
    }

    fn lod2_multi_surface(&self) -> Option<&MultiSurface> {
        self.abstract_thematic_surface().lod2_multi_surface.as_ref()
    }

    fn lod3_multi_surface(&self) -> Option<&MultiSurface> {
        self.abstract_thematic_surface().lod3_multi_surface.as_ref()
    }

    fn multi_surfaces_by_lod(&self) -> HashMap<LevelOfDetail, &MultiSurface> {
        let mut map = HashMap::new();
        if let Some(x) = self.lod0_multi_surface() {
            map.insert(LevelOfDetail::Zero, x);
        }
        if let Some(x) = self.lod1_multi_surface() {
            map.insert(LevelOfDetail::One, x);
        }
        if let Some(x) = self.lod2_multi_surface() {
            map.insert(LevelOfDetail::Two, x);
        }
        if let Some(x) = self.lod3_multi_surface() {
            map.insert(LevelOfDetail::Three, x);
        }
        map
    }

    fn compute_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = vec![
            self.lod0_multi_surface().map(|x| x.compute_envelope()),
            self.lod1_multi_surface().map(|x| x.compute_envelope()),
            self.lod2_multi_surface().map(|x| x.compute_envelope()),
            self.lod3_multi_surface().map(|x| x.compute_envelope()),
        ]
        .into_iter()
        .flatten()
        .collect();

        Envelope::from_envelopes(&envelopes)
    }
}

pub trait AsAbstractThematicSurfaceMut:
    AsAbstractSpaceBoundaryMut + AsAbstractThematicSurface
{
    fn abstract_thematic_surface_mut(&mut self) -> &mut AbstractThematicSurface;

    fn set_lod0_multi_surface(&mut self, value: Option<MultiSurface>) {
        self.abstract_thematic_surface_mut().lod0_multi_surface = value;
    }

    fn set_lod1_multi_surface(&mut self, value: Option<MultiSurface>) {
        self.abstract_thematic_surface_mut().lod1_multi_surface = value;
    }

    fn set_lod2_multi_surface(&mut self, value: Option<MultiSurface>) {
        self.abstract_thematic_surface_mut().lod2_multi_surface = value;
    }

    fn set_lod3_multi_surface(&mut self, value: Option<MultiSurface>) {
        self.abstract_thematic_surface_mut().lod3_multi_surface = value;
    }

    fn refresh_bounded_by(&mut self) {
        let envelope = self.compute_envelope();
        self.set_bounded_by(envelope);
    }

    fn apply_transform(&mut self, m: &Isometry3<f64>) {
        if let Some(g) = &mut self.abstract_thematic_surface_mut().lod0_multi_surface {
            g.apply_transform(m);
        }
        if let Some(g) = &mut self.abstract_thematic_surface_mut().lod1_multi_surface {
            g.apply_transform(m);
        }
        if let Some(g) = &mut self.abstract_thematic_surface_mut().lod2_multi_surface {
            g.apply_transform(m);
        }
        if let Some(g) = &mut self.abstract_thematic_surface_mut().lod3_multi_surface {
            g.apply_transform(m);
        }
    }
}

impl AsAbstractThematicSurface for AbstractThematicSurface {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface {
        self
    }
}

impl AsAbstractThematicSurfaceMut for AbstractThematicSurface {
    fn abstract_thematic_surface_mut(&mut self) -> &mut AbstractThematicSurface {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_thematic_surface_traits {
    ($type:ty) => {
        $crate::impl_abstract_space_boundary_traits!($type);

        impl $crate::model::core::AsAbstractSpaceBoundary for $type {
            fn abstract_space_boundary(&self) -> &$crate::model::core::AbstractSpaceBoundary {
                use $crate::model::core::AsAbstractThematicSurface;
                &self.abstract_thematic_surface().abstract_space_boundary
            }
        }

        impl $crate::model::core::AsAbstractSpaceBoundaryMut for $type {
            fn abstract_space_boundary_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractSpaceBoundary {
                use $crate::model::core::AsAbstractThematicSurfaceMut;
                &mut self.abstract_thematic_surface_mut().abstract_space_boundary
            }
        }
    };
}

impl_abstract_thematic_surface_traits!(AbstractThematicSurface);
