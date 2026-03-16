use crate::model::construction::{DoorSurface, WindowSurface};
use crate::model::core::{
    AbstractThematicSurface, AsAbstractThematicSurface, AsAbstractThematicSurfaceMut,
};

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

#[derive(Debug, Clone, PartialEq)]
pub enum FillingSurfaceKind {
    DoorSurface(DoorSurface),
    WindowSurface(WindowSurface),
}

impl AsAbstractFillingSurface for FillingSurfaceKind {
    fn abstract_filling_surface(&self) -> &AbstractFillingSurface {
        match self {
            FillingSurfaceKind::DoorSurface(surface) => surface.abstract_filling_surface(),
            FillingSurfaceKind::WindowSurface(surface) => surface.abstract_filling_surface(),
        }
    }
}

impl AsAbstractFillingSurfaceMut for FillingSurfaceKind {
    fn abstract_filling_surface_mut(&mut self) -> &mut AbstractFillingSurface {
        match self {
            FillingSurfaceKind::DoorSurface(surface) => surface.abstract_filling_surface_mut(),
            FillingSurfaceKind::WindowSurface(surface) => surface.abstract_filling_surface_mut(),
        }
    }
}

impl_abstract_filling_surface_traits!(FillingSurfaceKind);
