use crate::impl_try_from_occupied_space_kind_ref_for_enum;
use crate::model::building::BuildingKind;
use crate::model::building::refs::BuildingKindRef;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::construction::{AbstractConstruction, AsAbstractConstruction, ConstructionKind};

#[derive(Debug, Clone, Copy)]
pub enum ConstructionKindRef<'a> {
    BuildingKind(BuildingKindRef<'a>),
}

impl<'a> From<&'a ConstructionKind> for ConstructionKindRef<'a> {
    fn from(item: &'a ConstructionKind) -> Self {
        match item {
            ConstructionKind::BuildingKind(x) => Self::BuildingKind(x.into()),
        }
    }
}

impl<'a> AsAbstractConstruction for ConstructionKindRef<'a> {
    fn abstract_construction(&self) -> &AbstractConstruction {
        match self {
            Self::BuildingKind(x) => x.abstract_construction(),
        }
    }
}
crate::impl_abstract_construction_traits!(ConstructionKindRef<'_>);

impl<'a> HasFeatureType for ConstructionKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::BuildingKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_construction_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::construction::refs::ConstructionKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::construction::refs::ConstructionKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_occupied_space_kind_ref!(ConstructionKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_construction_kind_ref!($variant, $variant);
    };
}
impl_from_for_construction_kind_ref!(BuildingKind);

#[macro_export]
macro_rules! impl_try_from_for_construction_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::construction::refs::ConstructionKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(
                x: $crate::model::construction::refs::ConstructionKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::refs::ConstructionKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_occupied_space_kind_ref!(ConstructionKind, $type);
    };
}

#[macro_export]
macro_rules! impl_try_from_construction_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::construction::refs::ConstructionKindRef<'a>>
            for $EnumRef<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::construction::refs::ConstructionKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::refs::ConstructionKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_occupied_space_kind_ref_for_enum!(ConstructionKind, $EnumRef);
    };
}
impl_try_from_occupied_space_kind_ref_for_enum!(ConstructionKind, ConstructionKindRef);
