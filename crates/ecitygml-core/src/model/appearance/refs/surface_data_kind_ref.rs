use crate::impl_try_from_feature_kind_ref_for_enum;
use crate::model::appearance::refs::TextureKindRef;
use crate::model::appearance::{
    AbstractSurfaceData, AsAbstractSurfaceData, SurfaceDataKind, TextureKind, X3DMaterial,
};
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;

#[derive(Debug, Clone, Copy)]
pub enum SurfaceDataKindRef<'a> {
    TextureKind(TextureKindRef<'a>),
    X3DMaterial(&'a X3DMaterial),
}

impl<'a> From<&'a SurfaceDataKind> for SurfaceDataKindRef<'a> {
    fn from(item: &'a SurfaceDataKind) -> Self {
        match item {
            SurfaceDataKind::TextureKind(x) => Self::TextureKind(x.into()),
            SurfaceDataKind::X3DMaterial(x) => Self::X3DMaterial(x),
        }
    }
}

impl<'a> AsAbstractSurfaceData for SurfaceDataKindRef<'a> {
    fn abstract_surface_data(&self) -> &AbstractSurfaceData {
        match self {
            Self::TextureKind(x) => x.abstract_surface_data(),
            Self::X3DMaterial(x) => x.abstract_surface_data(),
        }
    }
}
crate::impl_abstract_surface_data_traits!(SurfaceDataKindRef<'_>);

impl<'a> HasFeatureType for SurfaceDataKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::TextureKind(x) => x.feature_type(),
            Self::X3DMaterial(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_surface_data_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::appearance::refs::SurfaceDataKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::appearance::refs::SurfaceDataKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_feature_kind_ref!(SurfaceDataKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_surface_data_kind_ref!($variant, $variant);
    };
}
impl_from_for_surface_data_kind_ref!(TextureKind);
impl_from_for_surface_data_kind_ref!(X3DMaterial);

#[macro_export]
macro_rules! impl_try_from_for_surface_data_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::appearance::refs::SurfaceDataKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(
                x: $crate::model::appearance::refs::SurfaceDataKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::appearance::refs::SurfaceDataKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_surface_data_kind_ref!($variant, $variant);
    };
}
impl_try_from_for_surface_data_kind_ref!(X3DMaterial);

#[macro_export]
macro_rules! impl_try_from_surface_data_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::appearance::refs::SurfaceDataKindRef<'a>> for $EnumRef<'a> {
            type Error = ();
            fn try_from(
                x: $crate::model::appearance::refs::SurfaceDataKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::appearance::refs::SurfaceDataKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_feature_kind_ref_for_enum!(SurfaceDataKind, $EnumRef);
    };
}
impl_try_from_feature_kind_ref_for_enum!(SurfaceDataKind, SurfaceDataKindRef);
