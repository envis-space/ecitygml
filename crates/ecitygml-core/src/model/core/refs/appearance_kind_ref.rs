use crate::impl_try_from_feature_with_lifespan_kind_ref_for_enum;
use crate::model::appearance::Appearance;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::core::{AbstractAppearance, AppearanceKind, AsAbstractAppearance};

#[derive(Debug, Clone, Copy)]
pub enum AppearanceKindRef<'a> {
    Appearance(&'a Appearance),
}

impl<'a> From<&'a AppearanceKind> for AppearanceKindRef<'a> {
    fn from(item: &'a AppearanceKind) -> Self {
        match item {
            AppearanceKind::Appearance(x) => Self::Appearance(x),
        }
    }
}

impl<'a> AsAbstractAppearance for AppearanceKindRef<'a> {
    fn abstract_appearance(&self) -> &AbstractAppearance {
        match self {
            Self::Appearance(x) => x.abstract_appearance(),
        }
    }
}
crate::impl_abstract_appearance_traits!(AppearanceKindRef<'_>);

impl<'a> HasFeatureType for AppearanceKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::Appearance(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_appearance_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::AppearanceKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::AppearanceKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_feature_with_lifespan_kind_ref!(AppearanceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_appearance_kind_ref!($variant, $variant);
    };
}
impl_from_for_appearance_kind_ref!(Appearance);

#[macro_export]
macro_rules! impl_try_from_for_appearance_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AppearanceKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(x: $crate::model::core::refs::AppearanceKindRef<'a>) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AppearanceKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_feature_with_lifespan_kind_ref!(AppearanceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_appearance_kind_ref!($variant, $variant);
    };
}
impl_try_from_for_appearance_kind_ref!(Appearance);
impl_try_from_feature_with_lifespan_kind_ref_for_enum!(AppearanceKind, AppearanceKindRef);
