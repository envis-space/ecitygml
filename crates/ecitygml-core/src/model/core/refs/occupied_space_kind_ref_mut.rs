use crate::impl_try_from_physical_space_kind_ref_mut_for_enum;
use crate::model::city_furniture::CityFurniture;
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::construction::refs::{
    ConstructionKindRefMut, ConstructiveElementKindRefMut, FillingElementKindRefMut,
    InstallationKindRefMut,
};
use crate::model::construction::{
    ConstructionKind, ConstructiveElementKind, FillingElementKind, InstallationKind,
};
use crate::model::core::{
    AbstractOccupiedSpace, AsAbstractOccupiedSpace, AsAbstractOccupiedSpaceMut, OccupiedSpaceKind,
};
use crate::model::generics::GenericOccupiedSpace;
use crate::model::vegetation::VegetationObjectKind;
use crate::model::vegetation::refs::VegetationObjectKindRefMut;
use crate::model::water_body::WaterBody;

#[derive(Debug)]
pub enum OccupiedSpaceKindRefMut<'a> {
    CityFurniture(&'a mut CityFurniture),
    ConstructionKind(ConstructionKindRefMut<'a>),
    ConstructiveElementKind(ConstructiveElementKindRefMut<'a>),
    FillingElementKind(FillingElementKindRefMut<'a>),
    GenericOccupiedSpace(&'a mut GenericOccupiedSpace),
    InstallationKind(InstallationKindRefMut<'a>),
    VegetationObjectKind(VegetationObjectKindRefMut<'a>),
    WaterBody(&'a mut WaterBody),
}

impl<'a> From<&'a mut OccupiedSpaceKind> for OccupiedSpaceKindRefMut<'a> {
    fn from(item: &'a mut OccupiedSpaceKind) -> Self {
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

impl<'a> AsAbstractOccupiedSpace for OccupiedSpaceKindRefMut<'a> {
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

impl<'a> AsAbstractOccupiedSpaceMut for OccupiedSpaceKindRefMut<'a> {
    fn abstract_occupied_space_mut(&mut self) -> &mut AbstractOccupiedSpace {
        match self {
            Self::CityFurniture(x) => x.abstract_occupied_space_mut(),
            Self::ConstructionKind(x) => x.abstract_occupied_space_mut(),
            Self::ConstructiveElementKind(x) => x.abstract_occupied_space_mut(),
            Self::FillingElementKind(x) => x.abstract_occupied_space_mut(),
            Self::GenericOccupiedSpace(x) => x.abstract_occupied_space_mut(),
            Self::InstallationKind(x) => x.abstract_occupied_space_mut(),
            Self::VegetationObjectKind(x) => x.abstract_occupied_space_mut(),
            Self::WaterBody(x) => x.abstract_occupied_space_mut(),
        }
    }
}
crate::impl_abstract_occupied_space_traits!(OccupiedSpaceKindRefMut<'_>);
crate::impl_abstract_occupied_space_mut_traits!(OccupiedSpaceKindRefMut<'_>);

impl<'a> OccupiedSpaceKindRefMut<'a> {
    pub fn recompute_bounding_shape(&mut self) {
        match self {
            Self::CityFurniture(x) => x.recompute_bounding_shape(),
            Self::ConstructionKind(x) => x.recompute_bounding_shape(),
            Self::ConstructiveElementKind(x) => x.recompute_bounding_shape(),
            Self::FillingElementKind(x) => x.recompute_bounding_shape(),
            Self::GenericOccupiedSpace(x) => x.recompute_bounding_shape(),
            Self::InstallationKind(x) => x.recompute_bounding_shape(),
            Self::VegetationObjectKind(x) => x.recompute_bounding_shape(),
            Self::WaterBody(x) => x.recompute_bounding_shape(),
        }
    }
}

impl<'a> HasFeatureType for OccupiedSpaceKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::CityFurniture(x) => x.feature_type(),
            Self::ConstructionKind(x) => x.feature_type(),
            Self::ConstructiveElementKind(x) => x.feature_type(),
            Self::FillingElementKind(x) => x.feature_type(),
            Self::GenericOccupiedSpace(x) => x.feature_type(),
            Self::InstallationKind(x) => x.feature_type(),
            Self::VegetationObjectKind(x) => x.feature_type(),
            Self::WaterBody(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_occupied_space_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type> for $crate::model::core::refs::OccupiedSpaceKindRefMut<'a> {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::OccupiedSpaceKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_physical_space_kind_ref_mut!(OccupiedSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_occupied_space_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_occupied_space_kind_ref_mut!(CityFurniture);
impl_from_for_occupied_space_kind_ref_mut!(ConstructionKind);
impl_from_for_occupied_space_kind_ref_mut!(ConstructiveElementKind);
impl_from_for_occupied_space_kind_ref_mut!(FillingElementKind);
impl_from_for_occupied_space_kind_ref_mut!(GenericOccupiedSpace);
impl_from_for_occupied_space_kind_ref_mut!(InstallationKind);
impl_from_for_occupied_space_kind_ref_mut!(VegetationObjectKind);
impl_from_for_occupied_space_kind_ref_mut!(WaterBody);

#[macro_export]
macro_rules! impl_try_from_for_occupied_space_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::OccupiedSpaceKindRefMut<'a>> for &'a mut $type {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::OccupiedSpaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::OccupiedSpaceKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_physical_space_kind_ref_mut!(OccupiedSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_occupied_space_kind_ref_mut!($variant, $variant);
    };
}
impl_try_from_for_occupied_space_kind_ref_mut!(CityFurniture);
impl_try_from_for_occupied_space_kind_ref_mut!(GenericOccupiedSpace);
impl_try_from_for_occupied_space_kind_ref_mut!(WaterBody);

#[macro_export]
macro_rules! impl_try_from_occupied_space_kind_ref_mut_for_enum {
    ($variant:ident, $EnumRefMut:ident) => {
        impl<'a> TryFrom<$crate::model::core::refs::OccupiedSpaceKindRefMut<'a>>
            for $EnumRefMut<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::OccupiedSpaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::OccupiedSpaceKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_physical_space_kind_ref_mut_for_enum!(OccupiedSpaceKind, $EnumRefMut);
    };
}
impl_try_from_physical_space_kind_ref_mut_for_enum!(OccupiedSpaceKind, OccupiedSpaceKindRefMut);
