use crate::impl_try_from_physical_space_kind_ref_for_enum;
use crate::model::city_furniture::CityFurniture;
use crate::model::common::FeatureType;
use crate::model::common::HasFeatureType;
use crate::model::construction::refs::{
    ConstructionKindRef, ConstructiveElementKindRef, FillingElementKindRef, InstallationKindRef,
};
use crate::model::construction::{
    ConstructionKind, ConstructiveElementKind, FillingElementKind, InstallationKind,
};
use crate::model::core::{AbstractOccupiedSpace, AsAbstractOccupiedSpace, OccupiedSpaceKind};
use crate::model::generics::GenericOccupiedSpace;
use crate::model::vegetation::VegetationObjectKind;
use crate::model::vegetation::refs::VegetationObjectKindRef;
use crate::model::water_body::WaterBody;

#[derive(Debug, Clone, Copy)]
pub enum OccupiedSpaceKindRef<'a> {
    CityFurniture(&'a CityFurniture),
    ConstructionKind(ConstructionKindRef<'a>),
    ConstructiveElementKind(ConstructiveElementKindRef<'a>),
    FillingElementKind(FillingElementKindRef<'a>),
    GenericOccupiedSpace(&'a GenericOccupiedSpace),
    InstallationKind(InstallationKindRef<'a>),
    VegetationObjectKind(VegetationObjectKindRef<'a>),
    WaterBody(&'a WaterBody),
}

impl<'a> From<&'a OccupiedSpaceKind> for OccupiedSpaceKindRef<'a> {
    fn from(item: &'a OccupiedSpaceKind) -> Self {
        match item {
            OccupiedSpaceKind::CityFurniture(x) => Self::CityFurniture(x),
            OccupiedSpaceKind::ConstructionKind(x) => Self::ConstructionKind(x.into()),
            OccupiedSpaceKind::ConstructiveElementKind(x) => {
                Self::ConstructiveElementKind(x.into())
            }
            OccupiedSpaceKind::FillingElementKind(x) => Self::FillingElementKind(x.into()),
            OccupiedSpaceKind::GenericOccupiedSpace(x) => Self::GenericOccupiedSpace(x),
            OccupiedSpaceKind::InstallationKind(x) => Self::InstallationKind(x.into()),
            OccupiedSpaceKind::VegetationObjectKind(x) => Self::VegetationObjectKind(x.into()),
            OccupiedSpaceKind::WaterBody(x) => Self::WaterBody(x),
        }
    }
}

impl<'a> AsAbstractOccupiedSpace for OccupiedSpaceKindRef<'a> {
    fn abstract_occupied_space(&self) -> &AbstractOccupiedSpace {
        match self {
            Self::CityFurniture(x) => x.abstract_occupied_space(),
            Self::ConstructionKind(x) => x.abstract_occupied_space(),
            Self::ConstructiveElementKind(x) => x.abstract_occupied_space(),
            Self::FillingElementKind(x) => x.abstract_occupied_space(),
            Self::GenericOccupiedSpace(x) => x.abstract_occupied_space(),
            Self::InstallationKind(x) => x.abstract_occupied_space(),
            Self::VegetationObjectKind(x) => x.abstract_occupied_space(),
            Self::WaterBody(x) => x.abstract_occupied_space(),
        }
    }
}
crate::impl_abstract_occupied_space_traits!(OccupiedSpaceKindRef<'_>);

impl<'a> HasFeatureType for OccupiedSpaceKindRef<'a> {
    fn feature_type(&self) -> FeatureType {
        match *self {
            Self::CityFurniture(_) => FeatureType::CityFurniture,
            Self::ConstructionKind(x) => x.feature_type(),
            Self::ConstructiveElementKind(x) => x.feature_type(),
            Self::FillingElementKind(x) => x.feature_type(),
            Self::GenericOccupiedSpace(_) => FeatureType::GenericOccupiedSpace,
            Self::InstallationKind(x) => x.feature_type(),
            Self::VegetationObjectKind(x) => x.feature_type(),
            Self::WaterBody(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_occupied_space_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::core::refs::OccupiedSpaceKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::core::refs::OccupiedSpaceKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_physical_space_kind_ref!(OccupiedSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_occupied_space_kind_ref!($variant, $variant);
    };
}
impl_from_for_occupied_space_kind_ref!(CityFurniture);
impl_from_for_occupied_space_kind_ref!(ConstructionKind);
impl_from_for_occupied_space_kind_ref!(ConstructiveElementKind);
impl_from_for_occupied_space_kind_ref!(FillingElementKind);
impl_from_for_occupied_space_kind_ref!(GenericOccupiedSpace);
impl_from_for_occupied_space_kind_ref!(InstallationKind);
impl_from_for_occupied_space_kind_ref!(VegetationObjectKind);
impl_from_for_occupied_space_kind_ref!(WaterBody);

#[macro_export]
macro_rules! impl_try_from_for_occupied_space_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::OccupiedSpaceKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::OccupiedSpaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::OccupiedSpaceKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_physical_space_kind_ref!(OccupiedSpaceKind, $type);
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
        impl<'a> TryFrom<$crate::model::core::refs::OccupiedSpaceKindRef<'a>> for $EnumRef<'a> {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::OccupiedSpaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::OccupiedSpaceKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_physical_space_kind_ref_for_enum!(OccupiedSpaceKind, $EnumRef);
    };
}
impl_try_from_physical_space_kind_ref_for_enum!(OccupiedSpaceKind, OccupiedSpaceKindRef);
