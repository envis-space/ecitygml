use crate::impl_abstract_occupied_space_mut_traits;
use crate::impl_abstract_occupied_space_traits;
use crate::model::city_furniture::CityFurniture;
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::construction::{
    ConstructionKind, ConstructiveElementKind, FillingElementKind, InstallationKind,
};
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use crate::model::core::{
    AbstractOccupiedSpace, AsAbstractOccupiedSpace, AsAbstractOccupiedSpaceMut,
};
use crate::model::generics::GenericOccupiedSpace;
use crate::model::vegetation::VegetationObjectKind;
use crate::model::water_body::WaterBody;
use auto_enums::auto_enum;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum OccupiedSpaceKind {
    CityFurniture(CityFurniture),
    ConstructionKind(ConstructionKind),
    ConstructiveElementKind(ConstructiveElementKind),
    FillingElementKind(FillingElementKind),
    GenericOccupiedSpace(GenericOccupiedSpace),
    InstallationKind(InstallationKind),
    VegetationObjectKind(VegetationObjectKind),
    WaterBody(WaterBody),
}

impl OccupiedSpaceKind {
    #[auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        match self {
            OccupiedSpaceKind::CityFurniture(x) => x.iter_features(),
            OccupiedSpaceKind::ConstructionKind(x) => x.iter_features(),
            OccupiedSpaceKind::ConstructiveElementKind(x) => x.iter_features(),
            OccupiedSpaceKind::FillingElementKind(x) => x.iter_features(),
            OccupiedSpaceKind::GenericOccupiedSpace(x) => x.iter_features(),
            OccupiedSpaceKind::InstallationKind(x) => x.iter_features(),
            OccupiedSpaceKind::VegetationObjectKind(x) => x.iter_features(),
            OccupiedSpaceKind::WaterBody(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            OccupiedSpaceKind::CityFurniture(x) => x.for_each_feature_mut(f),
            OccupiedSpaceKind::ConstructionKind(x) => x.for_each_feature_mut(f),
            OccupiedSpaceKind::ConstructiveElementKind(x) => x.for_each_feature_mut(f),
            OccupiedSpaceKind::FillingElementKind(x) => x.for_each_feature_mut(f),
            OccupiedSpaceKind::GenericOccupiedSpace(x) => x.for_each_feature_mut(f),
            OccupiedSpaceKind::InstallationKind(x) => x.for_each_feature_mut(f),
            OccupiedSpaceKind::VegetationObjectKind(x) => x.for_each_feature_mut(f),
            OccupiedSpaceKind::WaterBody(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            OccupiedSpaceKind::CityFurniture(x) => x.compute_envelope(),
            OccupiedSpaceKind::ConstructionKind(x) => x.compute_envelope(),
            OccupiedSpaceKind::ConstructiveElementKind(x) => x.compute_envelope(),
            OccupiedSpaceKind::FillingElementKind(x) => x.compute_envelope(),
            OccupiedSpaceKind::GenericOccupiedSpace(x) => x.compute_envelope(),
            OccupiedSpaceKind::InstallationKind(x) => x.compute_envelope(),
            OccupiedSpaceKind::VegetationObjectKind(x) => x.compute_envelope(),
            OccupiedSpaceKind::WaterBody(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            OccupiedSpaceKind::CityFurniture(x) => x.recompute_bounding_shape(),
            OccupiedSpaceKind::ConstructionKind(x) => x.recompute_bounding_shape(),
            OccupiedSpaceKind::ConstructiveElementKind(x) => x.recompute_bounding_shape(),
            OccupiedSpaceKind::FillingElementKind(x) => x.recompute_bounding_shape(),
            OccupiedSpaceKind::GenericOccupiedSpace(x) => x.recompute_bounding_shape(),
            OccupiedSpaceKind::InstallationKind(x) => x.recompute_bounding_shape(),
            OccupiedSpaceKind::VegetationObjectKind(x) => x.recompute_bounding_shape(),
            OccupiedSpaceKind::WaterBody(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            OccupiedSpaceKind::CityFurniture(x) => x.apply_transform(m),
            OccupiedSpaceKind::ConstructionKind(x) => x.apply_transform(m),
            OccupiedSpaceKind::ConstructiveElementKind(x) => x.apply_transform(m),
            OccupiedSpaceKind::FillingElementKind(x) => x.apply_transform(m),
            OccupiedSpaceKind::GenericOccupiedSpace(x) => x.apply_transform(m),
            OccupiedSpaceKind::InstallationKind(x) => x.apply_transform(m),
            OccupiedSpaceKind::VegetationObjectKind(x) => x.apply_transform(m),
            OccupiedSpaceKind::WaterBody(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractOccupiedSpace for OccupiedSpaceKind {
    fn abstract_occupied_space(&self) -> &AbstractOccupiedSpace {
        match self {
            Self::CityFurniture(x) => x.abstract_occupied_space(),
            Self::ConstructionKind(x) => x.abstract_occupied_space(),
            Self::ConstructiveElementKind(x) => x.abstract_occupied_space(),
            Self::FillingElementKind(x) => x.abstract_occupied_space(),
            Self::GenericOccupiedSpace(x) => x.abstract_occupied_space(),
            Self::VegetationObjectKind(x) => x.abstract_occupied_space(),
            Self::InstallationKind(x) => x.abstract_occupied_space(),
            Self::WaterBody(x) => x.abstract_occupied_space(),
        }
    }
}

impl AsAbstractOccupiedSpaceMut for OccupiedSpaceKind {
    fn abstract_occupied_space_mut(&mut self) -> &mut AbstractOccupiedSpace {
        match self {
            Self::CityFurniture(x) => x.abstract_occupied_space_mut(),
            Self::ConstructionKind(x) => x.abstract_occupied_space_mut(),
            Self::ConstructiveElementKind(x) => x.abstract_occupied_space_mut(),
            Self::FillingElementKind(x) => x.abstract_occupied_space_mut(),
            Self::GenericOccupiedSpace(x) => x.abstract_occupied_space_mut(),
            Self::VegetationObjectKind(x) => x.abstract_occupied_space_mut(),
            Self::InstallationKind(x) => x.abstract_occupied_space_mut(),
            Self::WaterBody(x) => x.abstract_occupied_space_mut(),
        }
    }
}

impl_abstract_occupied_space_traits!(OccupiedSpaceKind);
impl_abstract_occupied_space_mut_traits!(OccupiedSpaceKind);

impl HasFeatureType for OccupiedSpaceKind {
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
macro_rules! impl_from_for_occupied_space_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::OccupiedSpaceKind {
            fn from(x: $type) -> Self {
                $crate::model::core::OccupiedSpaceKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_physical_space_kind!(OccupiedSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_occupied_space_kind!($variant, $variant);
    };
}
impl_from_for_occupied_space_kind!(CityFurniture);
impl_from_for_occupied_space_kind!(ConstructionKind);
impl_from_for_occupied_space_kind!(ConstructiveElementKind);
impl_from_for_occupied_space_kind!(FillingElementKind);
impl_from_for_occupied_space_kind!(GenericOccupiedSpace);
impl_from_for_occupied_space_kind!(InstallationKind);
impl_from_for_occupied_space_kind!(VegetationObjectKind);
impl_from_for_occupied_space_kind!(WaterBody);

#[macro_export]
macro_rules! impl_try_from_for_occupied_space_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::OccupiedSpaceKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::OccupiedSpaceKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::OccupiedSpaceKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_physical_space_kind!(OccupiedSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_occupied_space_kind!($variant, $variant);
    };
}
impl_try_from_for_occupied_space_kind!(CityFurniture);
impl_try_from_for_occupied_space_kind!(GenericOccupiedSpace);
impl_try_from_for_occupied_space_kind!(WaterBody);
