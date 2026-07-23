use crate::impl_try_from_feature_kind_ref_for_enum;
use crate::model::appearance::refs::AbstractTextureKindRef;
use crate::model::appearance::{
    AbstractSurfaceData, AbstractSurfaceDataKind, AbstractTextureKind, AsAbstractSurfaceData,
    X3DMaterial,
};
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;

#[derive(Debug, Clone, Copy)]
pub enum AbstractSurfaceDataKindRef<'a> {
    AbstractTextureKind(AbstractTextureKindRef<'a>),
    X3DMaterial(&'a X3DMaterial),
}

impl<'a> From<&'a AbstractSurfaceDataKind> for AbstractSurfaceDataKindRef<'a> {
    fn from(item: &'a AbstractSurfaceDataKind) -> Self {
        match item {
            AbstractSurfaceDataKind::AbstractTextureKind(x) => Self::AbstractTextureKind(x.into()),
            AbstractSurfaceDataKind::X3DMaterial(x) => Self::X3DMaterial(x),
        }
    }
}

impl<'a> AsAbstractSurfaceData for AbstractSurfaceDataKindRef<'a> {
    fn abstract_surface_data(&self) -> &AbstractSurfaceData {
        match self {
            Self::AbstractTextureKind(x) => x.abstract_surface_data(),
            Self::X3DMaterial(x) => x.abstract_surface_data(),
        }
    }
}
crate::impl_abstract_surface_data_traits!(AbstractSurfaceDataKindRef<'_>);

impl<'a> HasFeatureType for AbstractSurfaceDataKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::AbstractTextureKind(x) => x.feature_type(),
            Self::X3DMaterial(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_surface_data_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type>
            for $crate::model::appearance::refs::AbstractSurfaceDataKindRef<'a>
        {
            fn from(x: &'a $type) -> Self {
                $crate::model::appearance::refs::AbstractSurfaceDataKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_feature_kind_ref!(AbstractSurfaceDataKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_surface_data_kind_ref!($variant, $variant);
    };
}
impl_from_for_surface_data_kind_ref!(AbstractTextureKind);
impl_from_for_surface_data_kind_ref!(X3DMaterial);

#[macro_export]
macro_rules! impl_try_from_for_surface_data_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::appearance::refs::AbstractSurfaceDataKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::appearance::refs::AbstractSurfaceDataKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::appearance::refs::AbstractSurfaceDataKindRef::$variant(k) => {
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
        impl<'a> TryFrom<$crate::model::appearance::refs::AbstractSurfaceDataKindRef<'a>>
            for $EnumRef<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::appearance::refs::AbstractSurfaceDataKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::appearance::refs::AbstractSurfaceDataKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_feature_kind_ref_for_enum!(AbstractSurfaceDataKind, $EnumRef);
    };
}
impl_try_from_feature_kind_ref_for_enum!(AbstractSurfaceDataKind, AbstractSurfaceDataKindRef);
