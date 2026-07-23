use crate::impl_try_from_surface_data_kind_ref_mut_for_enum;
use crate::model::appearance::{
    AbstractTexture, AbstractTextureKind, AsAbstractTexture, AsAbstractTextureMut,
    GeoreferencedTexture, ParameterizedTexture,
};
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractTextureKindRefMut<'a> {
    GeoreferencedTexture(&'a mut GeoreferencedTexture),
    ParameterizedTexture(&'a mut ParameterizedTexture),
}

impl<'a> From<&'a mut AbstractTextureKind> for AbstractTextureKindRefMut<'a> {
    fn from(item: &'a mut AbstractTextureKind) -> Self {
        match item {
            AbstractTextureKind::GeoreferencedTexture(x) => Self::GeoreferencedTexture(x),
            AbstractTextureKind::ParameterizedTexture(x) => Self::ParameterizedTexture(x),
        }
    }
}

impl<'a> AsAbstractTexture for AbstractTextureKindRefMut<'a> {
    fn abstract_texture(&self) -> &AbstractTexture {
        match self {
            Self::GeoreferencedTexture(x) => x.abstract_texture(),
            Self::ParameterizedTexture(x) => x.abstract_texture(),
        }
    }
}

impl<'a> AsAbstractTextureMut for AbstractTextureKindRefMut<'a> {
    fn abstract_texture_mut(&mut self) -> &mut AbstractTexture {
        match self {
            Self::GeoreferencedTexture(x) => x.abstract_texture_mut(),
            Self::ParameterizedTexture(x) => x.abstract_texture_mut(),
        }
    }
}
crate::impl_abstract_texture_traits!(AbstractTextureKindRefMut<'_>);
crate::impl_abstract_texture_mut_traits!(AbstractTextureKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractTextureKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::GeoreferencedTexture(x) => x.feature_type(),
            Self::ParameterizedTexture(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_texture_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::appearance::refs::AbstractTextureKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::appearance::refs::AbstractTextureKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_surface_data_kind_ref_mut!(AbstractTextureKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_texture_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_texture_kind_ref_mut!(GeoreferencedTexture);
impl_from_for_texture_kind_ref_mut!(ParameterizedTexture);

#[macro_export]
macro_rules! impl_try_from_for_texture_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::appearance::refs::AbstractTextureKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::appearance::refs::AbstractTextureKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::appearance::refs::AbstractTextureKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_texture_kind_ref_mut!($variant, $variant);
    };
}
impl_try_from_for_texture_kind_ref_mut!(GeoreferencedTexture);
impl_try_from_for_texture_kind_ref_mut!(ParameterizedTexture);
impl_try_from_surface_data_kind_ref_mut_for_enum!(AbstractTextureKind, AbstractTextureKindRefMut);

impl<'a> RecomputeBoundingShape for AbstractTextureKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::GeoreferencedTexture(x) => x.recompute_bounding_shape(),
            Self::ParameterizedTexture(x) => x.recompute_bounding_shape(),
        }
    }
}
