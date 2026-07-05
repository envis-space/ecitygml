use crate::impl_try_from_occupied_space_kind_ref_mut_for_enum;
use crate::model::building::BuildingKind;
use crate::model::building::refs::BuildingKindRefMut;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::construction::{
    AbstractConstruction, AsAbstractConstruction, AsAbstractConstructionMut, ConstructionKind,
};

#[derive(Debug)]
pub enum ConstructionKindRefMut<'a> {
    BuildingKind(BuildingKindRefMut<'a>),
}

impl<'a> From<&'a mut ConstructionKind> for ConstructionKindRefMut<'a> {
    fn from(item: &'a mut ConstructionKind) -> Self {
        match item {
            ConstructionKind::BuildingKind(x) => Self::BuildingKind(x.into()),
        }
    }
}

impl<'a> AsAbstractConstruction for ConstructionKindRefMut<'a> {
    fn abstract_construction(&self) -> &AbstractConstruction {
        match self {
            Self::BuildingKind(x) => x.abstract_construction(),
        }
    }
}

impl<'a> AsAbstractConstructionMut for ConstructionKindRefMut<'a> {
    fn abstract_construction_mut(&mut self) -> &mut AbstractConstruction {
        match self {
            Self::BuildingKind(x) => x.abstract_construction_mut(),
        }
    }
}
crate::impl_abstract_construction_traits!(ConstructionKindRefMut<'_>);
crate::impl_abstract_construction_mut_traits!(ConstructionKindRefMut<'_>);

impl<'a> ConstructionKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::BuildingKind(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for ConstructionKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::BuildingKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_construction_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::construction::refs::ConstructionKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::construction::refs::ConstructionKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_occupied_space_kind_ref_mut!(ConstructionKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_construction_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_construction_kind_ref_mut!(BuildingKind);

#[macro_export]
macro_rules! impl_try_from_for_construction_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::construction::refs::ConstructionKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::construction::refs::ConstructionKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::refs::ConstructionKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_occupied_space_kind_ref_mut!(ConstructionKind, $type);
    };
}

#[macro_export]
macro_rules! impl_try_from_construction_kind_ref_mut_for_enum {
    ($variant:ident, $EnumRefMut:ident) => {
        impl<'a> TryFrom<$crate::model::construction::refs::ConstructionKindRefMut<'a>>
            for $EnumRefMut<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::construction::refs::ConstructionKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::refs::ConstructionKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_occupied_space_kind_ref_mut_for_enum!(ConstructionKind, $EnumRefMut);
    };
}
impl_try_from_occupied_space_kind_ref_mut_for_enum!(ConstructionKind, ConstructionKindRefMut);
