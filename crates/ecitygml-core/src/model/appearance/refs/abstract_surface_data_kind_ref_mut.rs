use crate::impl_try_from_feature_kind_ref_mut_for_enum;
use crate::model::appearance::AbstractTextureKind;
use crate::model::appearance::refs::AbstractTextureKindRefMut;
use crate::model::appearance::{
    AbstractSurfaceData, AbstractSurfaceDataKind, AsAbstractSurfaceData, AsAbstractSurfaceDataMut,
    X3DMaterial,
};
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractSurfaceDataKindRefMut<'a> {
    AbstractTextureKind(AbstractTextureKindRefMut<'a>),
    X3DMaterial(&'a mut X3DMaterial),
}

impl<'a> From<&'a mut AbstractSurfaceDataKind> for AbstractSurfaceDataKindRefMut<'a> {
    fn from(item: &'a mut AbstractSurfaceDataKind) -> Self {
        match item {
            AbstractSurfaceDataKind::AbstractTextureKind(x) => Self::AbstractTextureKind(x.into()),
            AbstractSurfaceDataKind::X3DMaterial(x) => Self::X3DMaterial(x),
        }
    }
}

impl<'a> AsAbstractSurfaceData for AbstractSurfaceDataKindRefMut<'a> {
    fn abstract_surface_data(&self) -> &AbstractSurfaceData {
        match self {
            Self::AbstractTextureKind(x) => x.abstract_surface_data(),
            Self::X3DMaterial(x) => x.abstract_surface_data(),
        }
    }
}

impl<'a> AsAbstractSurfaceDataMut for AbstractSurfaceDataKindRefMut<'a> {
    fn abstract_surface_data_mut(&mut self) -> &mut AbstractSurfaceData {
        match self {
            Self::AbstractTextureKind(x) => x.abstract_surface_data_mut(),
            Self::X3DMaterial(x) => x.abstract_surface_data_mut(),
        }
    }
}
crate::impl_abstract_surface_data_traits!(AbstractSurfaceDataKindRefMut<'_>);
crate::impl_abstract_surface_data_mut_traits!(AbstractSurfaceDataKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractSurfaceDataKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AbstractTextureKind(x) => x.feature_type(),
            Self::X3DMaterial(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_surface_data_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::appearance::refs::AbstractSurfaceDataKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::appearance::refs::AbstractSurfaceDataKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_feature_kind_ref_mut!(AbstractSurfaceDataKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_surface_data_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_surface_data_kind_ref_mut!(AbstractTextureKind);
impl_from_for_surface_data_kind_ref_mut!(X3DMaterial);

#[macro_export]
macro_rules! impl_try_from_for_surface_data_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::appearance::refs::AbstractSurfaceDataKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::appearance::refs::AbstractSurfaceDataKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::appearance::refs::AbstractSurfaceDataKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_surface_data_kind_ref_mut!($variant, $variant);
    };
}
impl_try_from_for_surface_data_kind_ref_mut!(X3DMaterial);

#[macro_export]
macro_rules! impl_try_from_surface_data_kind_ref_mut_for_enum {
    ($variant:ident, $EnumRefMut:ident) => {
        impl<'a> TryFrom<$crate::model::appearance::refs::AbstractSurfaceDataKindRefMut<'a>>
            for $EnumRefMut<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::appearance::refs::AbstractSurfaceDataKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::appearance::refs::AbstractSurfaceDataKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_feature_kind_ref_mut_for_enum!(AbstractSurfaceDataKind, $EnumRefMut);
    };
}
impl_try_from_feature_kind_ref_mut_for_enum!(
    AbstractSurfaceDataKind,
    AbstractSurfaceDataKindRefMut
);

impl<'a> RecomputeBoundingShape for AbstractSurfaceDataKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::AbstractTextureKind(x) => x.recompute_bounding_shape(),
            Self::X3DMaterial(x) => x.recompute_bounding_shape(),
        }
    }
}
