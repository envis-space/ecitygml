use crate::model::construction::{GroundSurface, RoofSurface, WallSurface};
use crate::model::core::{
    AbstractThematicSurface, AsAbstractThematicSurface, AsAbstractThematicSurfaceMut,
};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractConstructionSurface {
    pub abstract_thematic_surface: AbstractThematicSurface,
}

impl AbstractConstructionSurface {
    pub fn new(abstract_thematic_surface: AbstractThematicSurface) -> Self {
        Self {
            abstract_thematic_surface,
        }
    }
}

pub trait AsAbstractConstructionSurface: AsAbstractThematicSurface {
    fn abstract_construction_surface(&self) -> &AbstractConstructionSurface;
}

pub trait AsAbstractConstructionSurfaceMut:
    AsAbstractThematicSurfaceMut + AsAbstractThematicSurface
{
    fn abstract_construction_surface_mut(&mut self) -> &mut AbstractConstructionSurface;
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

#[derive(Debug, Clone, PartialEq)]
pub enum ConstructionSurfaceKind {
    GroundSurface(GroundSurface),
    RoofSurface(RoofSurface),
    WallSurface(WallSurface),
}

impl AsAbstractConstructionSurface for ConstructionSurfaceKind {
    fn abstract_construction_surface(&self) -> &AbstractConstructionSurface {
        match self {
            ConstructionSurfaceKind::GroundSurface(surface) => {
                surface.abstract_construction_surface()
            }
            ConstructionSurfaceKind::RoofSurface(surface) => {
                surface.abstract_construction_surface()
            }
            ConstructionSurfaceKind::WallSurface(surface) => {
                surface.abstract_construction_surface()
            }
        }
    }
}

impl AsAbstractConstructionSurfaceMut for ConstructionSurfaceKind {
    fn abstract_construction_surface_mut(&mut self) -> &mut AbstractConstructionSurface {
        match self {
            ConstructionSurfaceKind::GroundSurface(surface) => {
                surface.abstract_construction_surface_mut()
            }
            ConstructionSurfaceKind::RoofSurface(surface) => {
                surface.abstract_construction_surface_mut()
            }
            ConstructionSurfaceKind::WallSurface(surface) => {
                surface.abstract_construction_surface_mut()
            }
        }
    }
}

impl_abstract_construction_surface_traits!(ConstructionSurfaceKind);
