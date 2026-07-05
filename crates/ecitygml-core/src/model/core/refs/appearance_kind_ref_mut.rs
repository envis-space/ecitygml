use crate::impl_try_from_feature_with_lifespan_kind_ref_mut_for_enum;
use crate::model::appearance::Appearance;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::{
    AbstractAppearance, AppearanceKind, AsAbstractAppearance, AsAbstractAppearanceMut,
};

#[derive(Debug)]
pub enum AppearanceKindRefMut<'a> {
    Appearance(&'a mut Appearance),
}

impl<'a> From<&'a mut AppearanceKind> for AppearanceKindRefMut<'a> {
    fn from(item: &'a mut AppearanceKind) -> Self {
        match item {
            AppearanceKind::Appearance(x) => Self::Appearance(x),
        }
    }
}

impl<'a> AsAbstractAppearance for AppearanceKindRefMut<'a> {
    fn abstract_appearance(&self) -> &AbstractAppearance {
        match self {
            Self::Appearance(x) => x.abstract_appearance(),
        }
    }
}

impl<'a> AsAbstractAppearanceMut for AppearanceKindRefMut<'a> {
    fn abstract_appearance_mut(&mut self) -> &mut AbstractAppearance {
        match self {
            Self::Appearance(x) => x.abstract_appearance_mut(),
        }
    }
}
crate::impl_abstract_appearance_traits!(AppearanceKindRefMut<'_>);
crate::impl_abstract_appearance_mut_traits!(AppearanceKindRefMut<'_>);

impl<'a> AppearanceKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::Appearance(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for AppearanceKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::Appearance(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_appearance_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type> for $crate::model::core::refs::AppearanceKindRefMut<'a> {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::AppearanceKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_feature_with_lifespan_kind_ref_mut!(AppearanceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_appearance_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_appearance_kind_ref_mut!(Appearance);

#[macro_export]
macro_rules! impl_try_from_for_appearance_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AppearanceKindRefMut<'a>> for &'a mut $type {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AppearanceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AppearanceKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_with_lifespan_kind_ref_mut!(AppearanceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_appearance_kind_ref_mut!($variant, $variant);
    };
}
impl_try_from_for_appearance_kind_ref_mut!(Appearance);
impl_try_from_feature_with_lifespan_kind_ref_mut_for_enum!(AppearanceKind, AppearanceKindRefMut);
