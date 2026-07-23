use crate::impl_try_from_physical_space_kind_ref_mut_for_enum;
use crate::model::city_furniture::CityFurniture;
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::construction::refs::{
    AbstractConstructionKindRefMut, AbstractConstructiveElementKindRefMut,
    AbstractFillingElementKindRefMut, AbstractInstallationKindRefMut,
};
use crate::model::construction::{
    AbstractConstructionKind, AbstractConstructiveElementKind, AbstractFillingElementKind,
    AbstractInstallationKind,
};
use crate::model::core::{
    AbstractOccupiedSpace, AbstractOccupiedSpaceKind, AsAbstractOccupiedSpace,
    AsAbstractOccupiedSpaceMut,
};
use crate::model::generics::GenericOccupiedSpace;
use crate::model::vegetation::AbstractVegetationObjectKind;
use crate::model::vegetation::refs::AbstractVegetationObjectKindRefMut;
use crate::model::water_body::WaterBody;
use egml::model::common::RecomputeBoundingShape;

#[derive(Debug)]
pub enum AbstractOccupiedSpaceKindRefMut<'a> {
    CityFurniture(&'a mut CityFurniture),
    AbstractConstructionKind(AbstractConstructionKindRefMut<'a>),
    AbstractConstructiveElementKind(AbstractConstructiveElementKindRefMut<'a>),
    AbstractFillingElementKind(AbstractFillingElementKindRefMut<'a>),
    GenericOccupiedSpace(&'a mut GenericOccupiedSpace),
    AbstractInstallationKind(AbstractInstallationKindRefMut<'a>),
    AbstractVegetationObjectKind(AbstractVegetationObjectKindRefMut<'a>),
    WaterBody(&'a mut WaterBody),
}

impl<'a> From<&'a mut AbstractOccupiedSpaceKind> for AbstractOccupiedSpaceKindRefMut<'a> {
    fn from(item: &'a mut AbstractOccupiedSpaceKind) -> Self {
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

impl<'a> AsAbstractOccupiedSpace for AbstractOccupiedSpaceKindRefMut<'a> {
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

impl<'a> AsAbstractOccupiedSpaceMut for AbstractOccupiedSpaceKindRefMut<'a> {
    fn abstract_occupied_space_mut(&mut self) -> &mut AbstractOccupiedSpace {
        match self {
            Self::CityFurniture(x) => x.abstract_occupied_space_mut(),
            Self::AbstractConstructionKind(x) => x.abstract_occupied_space_mut(),
            Self::AbstractConstructiveElementKind(x) => x.abstract_occupied_space_mut(),
            Self::AbstractFillingElementKind(x) => x.abstract_occupied_space_mut(),
            Self::GenericOccupiedSpace(x) => x.abstract_occupied_space_mut(),
            Self::AbstractInstallationKind(x) => x.abstract_occupied_space_mut(),
            Self::AbstractVegetationObjectKind(x) => x.abstract_occupied_space_mut(),
            Self::WaterBody(x) => x.abstract_occupied_space_mut(),
        }
    }
}
crate::impl_abstract_occupied_space_traits!(AbstractOccupiedSpaceKindRefMut<'_>);
crate::impl_abstract_occupied_space_mut_traits!(AbstractOccupiedSpaceKindRefMut<'_>);

impl<'a> HasFeatureType for AbstractOccupiedSpaceKindRefMut<'a> {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::CityFurniture(x) => x.feature_type(),
            Self::AbstractConstructionKind(x) => x.feature_type(),
            Self::AbstractConstructiveElementKind(x) => x.feature_type(),
            Self::AbstractFillingElementKind(x) => x.feature_type(),
            Self::GenericOccupiedSpace(x) => x.feature_type(),
            Self::AbstractInstallationKind(x) => x.feature_type(),
            Self::AbstractVegetationObjectKind(x) => x.feature_type(),
            Self::WaterBody(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_occupied_space_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a mut $type>
            for $crate::model::core::refs::AbstractOccupiedSpaceKindRefMut<'a>
        {
            fn from(x: &'a mut $type) -> Self {
                $crate::model::core::refs::AbstractOccupiedSpaceKindRefMut::$variant(x.into())
            }
        }
        $crate::impl_from_for_physical_space_kind_ref_mut!(AbstractOccupiedSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_occupied_space_kind_ref_mut!($variant, $variant);
    };
}
impl_from_for_occupied_space_kind_ref_mut!(CityFurniture);
impl_from_for_occupied_space_kind_ref_mut!(AbstractConstructionKind);
impl_from_for_occupied_space_kind_ref_mut!(AbstractConstructiveElementKind);
impl_from_for_occupied_space_kind_ref_mut!(AbstractFillingElementKind);
impl_from_for_occupied_space_kind_ref_mut!(GenericOccupiedSpace);
impl_from_for_occupied_space_kind_ref_mut!(AbstractInstallationKind);
impl_from_for_occupied_space_kind_ref_mut!(AbstractVegetationObjectKind);
impl_from_for_occupied_space_kind_ref_mut!(WaterBody);

#[macro_export]
macro_rules! impl_try_from_for_occupied_space_kind_ref_mut {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::core::refs::AbstractOccupiedSpaceKindRefMut<'a>>
            for &'a mut $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractOccupiedSpaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractOccupiedSpaceKindRefMut::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_physical_space_kind_ref_mut!(AbstractOccupiedSpaceKind, $type);
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
        impl<'a> TryFrom<$crate::model::core::refs::AbstractOccupiedSpaceKindRefMut<'a>>
            for $EnumRefMut<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::core::refs::AbstractOccupiedSpaceKindRefMut<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::core::refs::AbstractOccupiedSpaceKindRefMut::$variant(k) => {
                        $EnumRefMut::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_physical_space_kind_ref_mut_for_enum!(
            AbstractOccupiedSpaceKind,
            $EnumRefMut
        );
    };
}
impl_try_from_physical_space_kind_ref_mut_for_enum!(
    AbstractOccupiedSpaceKind,
    AbstractOccupiedSpaceKindRefMut
);

impl<'a> RecomputeBoundingShape for AbstractOccupiedSpaceKindRefMut<'a> {
    fn recompute_bounding_shape(&mut self) {
        match self {
            Self::CityFurniture(x) => x.recompute_bounding_shape(),
            Self::AbstractConstructionKind(x) => x.recompute_bounding_shape(),
            Self::AbstractConstructiveElementKind(x) => x.recompute_bounding_shape(),
            Self::AbstractFillingElementKind(x) => x.recompute_bounding_shape(),
            Self::GenericOccupiedSpace(x) => x.recompute_bounding_shape(),
            Self::AbstractInstallationKind(x) => x.recompute_bounding_shape(),
            Self::AbstractVegetationObjectKind(x) => x.recompute_bounding_shape(),
            Self::WaterBody(x) => x.recompute_bounding_shape(),
        }
    }
}
