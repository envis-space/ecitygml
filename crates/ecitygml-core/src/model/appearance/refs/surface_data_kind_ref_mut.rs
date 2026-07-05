use crate::impl_try_from_feature_kind_ref_mut_for_enum;
use crate::model::appearance::TextureKind;
use crate::model::appearance::refs::TextureKindRefMut;
use crate::model::appearance::{
    AbstractSurfaceData, AsAbstractSurfaceData, AsAbstractSurfaceDataMut, SurfaceDataKind,
    X3DMaterial,
};
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;

#[derive(Debug)]
pub enum SurfaceDataKindRefMut<'a> {
    TextureKind(TextureKindRefMut<'a>),
    X3DMaterial(&'a mut X3DMaterial),
}

impl<'a> From<&'a mut SurfaceDataKind> for SurfaceDataKindRefMut<'a> {
    fn from(item: &'a mut SurfaceDataKind) -> Self {
        match item {
            SurfaceDataKind::TextureKind(x) => Self::TextureKind(x.into()),
            SurfaceDataKind::X3DMaterial(x) => Self::X3DMaterial(x),
        }
    }
}

impl<'a> AsAbstractSurfaceData for SurfaceDataKindRefMut<'a> {
    fn abstract_surface_data(&self) -> &AbstractSurfaceData {
        match self {
            Self::TextureKind(x) => x.abstract_surface_data(),
            Self::X3DMaterial(x) => x.abstract_surface_data(),
        }
    }
}

impl<'a> AsAbstractSurfaceDataMut for SurfaceDataKindRefMut<'a> {
    fn abstract_surface_data_mut(&mut self) -> &mut AbstractSurfaceData {
        match self {
            Self::TextureKind(x) => x.abstract_surface_data_mut(),
            Self::X3DMaterial(x) => x.abstract_surface_data_mut(),
        }
    }
}
crate::impl_abstract_surface_data_traits!(SurfaceDataKindRefMut<'_>);
crate::impl_abstract_surface_data_mut_traits!(SurfaceDataKindRefMut<'_>);

impl<'a> SurfaceDataKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::TextureKind(x) => x.recompute_bounding_shape(),
            Self::X3DMaterial(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for SurfaceDataKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::TextureKind(x) => x.feature_type(),
            Self::X3DMaterial(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_surface_data_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::appearance::refs::SurfaceDataKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::appearance::refs::SurfaceDataKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_feature_kind_ref_mut!(SurfaceDataKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_surface_data_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_surface_data_kind_ref_mut!(TextureKind);
impl_from_for_surface_data_kind_ref_mut!(X3DMaterial);

#[macro_export]
macro_rules! impl_try_from_for_surface_data_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::appearance::refs::SurfaceDataKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::appearance::refs::SurfaceDataKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::appearance::refs::SurfaceDataKindRefMut::$variant(k) => {
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
        impl<'a> TryFrom<$crate::model::appearance::refs::SurfaceDataKindRefMut<'a>>
            for $EnumRefMut<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::appearance::refs::SurfaceDataKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::appearance::refs::SurfaceDataKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_feature_kind_ref_mut_for_enum!(SurfaceDataKind, $EnumRefMut);
    };
}
impl_try_from_feature_kind_ref_mut_for_enum!(SurfaceDataKind, SurfaceDataKindRefMut);
