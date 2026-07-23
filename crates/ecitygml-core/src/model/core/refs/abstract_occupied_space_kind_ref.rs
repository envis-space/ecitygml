use crate::impl_try_from_physical_space_kind_ref_for_enum;
use crate::model::city_furniture::CityFurniture;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::construction::refs::{
    AbstractConstructionKindRef, AbstractConstructiveElementKindRef, AbstractFillingElementKindRef,
    AbstractInstallationKindRef,
};
use crate::model::construction::{
    AbstractConstructionKind, AbstractConstructiveElementKind, AbstractFillingElementKind,
    AbstractInstallationKind,
};
use crate::model::core::{
    AbstractOccupiedSpace, AbstractOccupiedSpaceKind, AsAbstractOccupiedSpace,
};
use crate::model::generics::GenericOccupiedSpace;
use crate::model::vegetation::AbstractVegetationObjectKind;
use crate::model::vegetation::refs::AbstractVegetationObjectKindRef;
use crate::model::water_body::WaterBody;

#[derive(Debug, Clone, Copy)]
pub enum AbstractOccupiedSpaceKindRef<'a> {
    CityFurniture(&'a CityFurniture),
    AbstractConstructionKind(AbstractConstructionKindRef<'a>),
    AbstractConstructiveElementKind(AbstractConstructiveElementKindRef<'a>),
    AbstractFillingElementKind(AbstractFillingElementKindRef<'a>),
    GenericOccupiedSpace(&'a GenericOccupiedSpace),
    AbstractInstallationKind(AbstractInstallationKindRef<'a>),
    AbstractVegetationObjectKind(AbstractVegetationObjectKindRef<'a>),
    WaterBody(&'a WaterBody),
}

impl<'a> From<&'a AbstractOccupiedSpaceKind> for AbstractOccupiedSpaceKindRef<'a> {
    fn from(item: &'a AbstractOccupiedSpaceKind) -> Self {
        match item {
            AbstractOccupiedSpaceKind::CityFurniture(x) => Self::CityFurniture(x),
            AbstractOccupiedSpaceKind::AbstractConstructionKind(x) => {
                Self::AbstractConstructionKind(x.into())
            }
            AbstractOccupiedSpaceKind::AbstractConstructiveElementKind(x) => {
                Self::AbstractConstructiveElementKind(x.into())
            }
            AbstractOccupiedSpaceKind::AbstractFillingElementKind(x) => {
                Self::AbstractFillingElementKind(x.into())
            }
            AbstractOccupiedSpaceKind::GenericOccupiedSpace(x) => Self::GenericOccupiedSpace(x),
            AbstractOccupiedSpaceKind::AbstractInstallationKind(x) => {
                Self::AbstractInstallationKind(x.into())
            }
            AbstractOccupiedSpaceKind::AbstractVegetationObjectKind(x) => {
                Self::AbstractVegetationObjectKind(x.into())
            }
            AbstractOccupiedSpaceKind::WaterBody(x) => Self::WaterBody(x),
        }
    }
}

impl<'a> AsAbstractOccupiedSpace for AbstractOccupiedSpaceKindRef<'a> {
    fn abstract_occupied_space(&self) -> &AbstractOccupiedSpace {
        match self {
            Self::CityFurniture(x) => x.abstract_occupied_space(),
            Self::AbstractConstructionKind(x) => x.abstract_occupied_space(),
            Self::AbstractConstructiveElementKind(x) => x.abstract_occupied_space(),
            Self::AbstractFillingElementKind(x) => x.abstract_occupied_space(),
            Self::GenericOccupiedSpace(x) => x.abstract_occupied_space(),
            Self::AbstractInstallationKind(x) => x.abstract_occupied_space(),
            Self::AbstractVegetationObjectKind(x) => x.abstract_occupied_space(),
            Self::WaterBody(x) => x.abstract_occupied_space(),
        }
    }
}
crate::impl_abstract_occupied_space_traits!(AbstractOccupiedSpaceKindRef<'_>);

impl<'a> HasFeatureType for AbstractOccupiedSpaceKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::CityFurniture(_) => FeatureType::CityFurniture,
            Self::AbstractConstructionKind(x) => x.feature_type(),
            Self::AbstractConstructiveElementKind(x) => x.feature_type(),
            Self::AbstractFillingElementKind(x) => x.feature_type(),
            Self::GenericOccupiedSpace(_) => FeatureType::GenericOccupiedSpace,
            Self::AbstractInstallationKind(x) => x.feature_type(),
            Self::AbstractVegetationObjectKind(x) => x.feature_type(),
            Self::WaterBody(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_occupied_space_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::AbstractOccupiedSpaceKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::AbstractOccupiedSpaceKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_physical_space_kind_ref!(AbstractOccupiedSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_occupied_space_kind_ref!($variant, $variant);
    };
}
impl_from_for_occupied_space_kind_ref!(CityFurniture);
impl_from_for_occupied_space_kind_ref!(AbstractConstructionKind);
impl_from_for_occupied_space_kind_ref!(AbstractConstructiveElementKind);
impl_from_for_occupied_space_kind_ref!(AbstractFillingElementKind);
impl_from_for_occupied_space_kind_ref!(GenericOccupiedSpace);
impl_from_for_occupied_space_kind_ref!(AbstractInstallationKind);
impl_from_for_occupied_space_kind_ref!(AbstractVegetationObjectKind);
impl_from_for_occupied_space_kind_ref!(WaterBody);

#[macro_export]
macro_rules! impl_try_from_for_occupied_space_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractOccupiedSpaceKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractOccupiedSpaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractOccupiedSpaceKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_physical_space_kind_ref!(AbstractOccupiedSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_occupied_space_kind_ref!($variant, $variant);
    };
}
impl_try_from_for_occupied_space_kind_ref!(CityFurniture);
impl_try_from_for_occupied_space_kind_ref!(GenericOccupiedSpace);
impl_try_from_for_occupied_space_kind_ref!(WaterBody);

#[macro_export]
macro_rules! impl_try_from_occupied_space_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractOccupiedSpaceKindRef<'a>>
            for $EnumRef<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractOccupiedSpaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractOccupiedSpaceKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_physical_space_kind_ref_for_enum!(
            AbstractOccupiedSpaceKind,
            $EnumRef
        );
    };
}
impl_try_from_physical_space_kind_ref_for_enum!(
    AbstractOccupiedSpaceKind,
    AbstractOccupiedSpaceKindRef
);
