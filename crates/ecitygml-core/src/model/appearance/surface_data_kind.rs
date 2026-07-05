use crate::impl_abstract_surface_data_mut_traits;
use crate::impl_abstract_surface_data_traits;
use crate::model::appearance::{
    AbstractSurfaceData, AsAbstractSurfaceData, AsAbstractSurfaceDataMut, TextureKind, X3DMaterial,
};
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use auto_enums::auto_enum;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum SurfaceDataKind {
    TextureKind(TextureKind),
    X3DMaterial(X3DMaterial),
}

impl SurfaceDataKind {
    #[auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        match self {
            SurfaceDataKind::TextureKind(x) => x.iter_features(),
            SurfaceDataKind::X3DMaterial(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            SurfaceDataKind::TextureKind(x) => x.for_each_feature_mut(f),
            SurfaceDataKind::X3DMaterial(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            SurfaceDataKind::TextureKind(x) => x.compute_envelope(),
            SurfaceDataKind::X3DMaterial(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            SurfaceDataKind::TextureKind(x) => x.recompute_bounding_shape(),
            SurfaceDataKind::X3DMaterial(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            SurfaceDataKind::TextureKind(x) => x.apply_transform(m),
            SurfaceDataKind::X3DMaterial(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractSurfaceData for SurfaceDataKind {
    fn abstract_surface_data(&self) -> &AbstractSurfaceData {
        match self {
            SurfaceDataKind::TextureKind(x) => x.abstract_surface_data(),
            SurfaceDataKind::X3DMaterial(x) => x.abstract_surface_data(),
        }
    }
}

impl AsAbstractSurfaceDataMut for SurfaceDataKind {
    fn abstract_surface_data_mut(&mut self) -> &mut AbstractSurfaceData {
        match self {
            SurfaceDataKind::TextureKind(x) => x.abstract_surface_data_mut(),
            SurfaceDataKind::X3DMaterial(x) => x.abstract_surface_data_mut(),
        }
    }
}

impl_abstract_surface_data_traits!(SurfaceDataKind);
impl_abstract_surface_data_mut_traits!(SurfaceDataKind);

impl HasFeatureType for SurfaceDataKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::TextureKind(x) => x.feature_type(),
            Self::X3DMaterial(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_surface_data_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::appearance::SurfaceDataKind {
            fn from(x: $type) -> Self {
                $crate::model::appearance::SurfaceDataKind::$variant(x.into())
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_from_for_surface_data_kind!($variant, $variant);
    };
}
impl_from_for_surface_data_kind!(TextureKind);
impl_from_for_surface_data_kind!(X3DMaterial);

#[macro_export]
macro_rules! impl_try_from_for_surface_data_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::appearance::SurfaceDataKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::appearance::SurfaceDataKind) -> Result<Self, ()> {
                match x {
                    $crate::model::appearance::SurfaceDataKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_surface_data_kind!($variant, $variant);
    };
}
impl_try_from_for_surface_data_kind!(X3DMaterial);
