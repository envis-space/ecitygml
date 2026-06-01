use crate::impl_abstract_occupied_space_traits;
use crate::model::city_furniture::CityFurniture;
use crate::model::common::{FeatureRef, FeatureRefMut, TopLevelFeatureRef};
use crate::model::construction::{
    ConstructionKind, ConstructiveElementKind, FillingElementKind, InstallationKind,
};
use crate::model::core::{
    AbstractOccupiedSpace, AsAbstractOccupiedSpace, AsAbstractOccupiedSpaceMut,
};
use crate::model::vegetation::VegetationObjectKind;
use auto_enums::auto_enum;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum OccupiedSpaceKind {
    CityFurniture(CityFurniture),
    ConstructionKind(ConstructionKind),
    ConstructiveElementKind(ConstructiveElementKind),
    FillingElementKind(FillingElementKind),
    InstallationKind(InstallationKind),
    VegetationObjectKind(VegetationObjectKind),
}

impl OccupiedSpaceKind {
    #[auto_enum(Iterator)]
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        match self {
            OccupiedSpaceKind::CityFurniture(x) => x.iter_features(),
            OccupiedSpaceKind::ConstructionKind(x) => x.iter_features(),
            OccupiedSpaceKind::ConstructiveElementKind(x) => x.iter_features(),
            OccupiedSpaceKind::FillingElementKind(x) => x.iter_features(),
            OccupiedSpaceKind::InstallationKind(x) => x.iter_features(),
            OccupiedSpaceKind::VegetationObjectKind(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            OccupiedSpaceKind::CityFurniture(x) => x.for_each_feature_mut(f),
            OccupiedSpaceKind::ConstructionKind(x) => x.for_each_feature_mut(f),
            OccupiedSpaceKind::ConstructiveElementKind(x) => x.for_each_feature_mut(f),
            OccupiedSpaceKind::FillingElementKind(x) => x.for_each_feature_mut(f),
            OccupiedSpaceKind::InstallationKind(x) => x.for_each_feature_mut(f),
            OccupiedSpaceKind::VegetationObjectKind(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            OccupiedSpaceKind::CityFurniture(x) => x.compute_envelope(),
            OccupiedSpaceKind::ConstructionKind(x) => x.compute_envelope(),
            OccupiedSpaceKind::ConstructiveElementKind(x) => x.compute_envelope(),
            OccupiedSpaceKind::FillingElementKind(x) => x.compute_envelope(),
            OccupiedSpaceKind::InstallationKind(x) => x.compute_envelope(),
            OccupiedSpaceKind::VegetationObjectKind(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            OccupiedSpaceKind::CityFurniture(x) => x.recompute_bounding_shape(),
            OccupiedSpaceKind::ConstructionKind(x) => x.recompute_bounding_shape(),
            OccupiedSpaceKind::ConstructiveElementKind(x) => x.recompute_bounding_shape(),
            OccupiedSpaceKind::FillingElementKind(x) => x.recompute_bounding_shape(),
            OccupiedSpaceKind::InstallationKind(x) => x.recompute_bounding_shape(),
            OccupiedSpaceKind::VegetationObjectKind(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            OccupiedSpaceKind::CityFurniture(x) => x.apply_transform(m),
            OccupiedSpaceKind::ConstructionKind(x) => x.apply_transform(m),
            OccupiedSpaceKind::ConstructiveElementKind(x) => x.apply_transform(m),
            OccupiedSpaceKind::FillingElementKind(x) => x.apply_transform(m),
            OccupiedSpaceKind::InstallationKind(x) => x.apply_transform(m),
            OccupiedSpaceKind::VegetationObjectKind(x) => x.apply_transform(m),
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
            Self::VegetationObjectKind(x) => x.abstract_occupied_space(),
            Self::InstallationKind(x) => x.abstract_occupied_space(),
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
            Self::VegetationObjectKind(x) => x.abstract_occupied_space_mut(),
            Self::InstallationKind(x) => x.abstract_occupied_space_mut(),
        }
    }
}

impl_abstract_occupied_space_traits!(OccupiedSpaceKind);

impl<'a> From<&'a OccupiedSpaceKind> for FeatureRef<'a> {
    fn from(item: &'a OccupiedSpaceKind) -> Self {
        match item {
            OccupiedSpaceKind::CityFurniture(x) => x.into(),
            OccupiedSpaceKind::ConstructionKind(x) => x.into(),
            OccupiedSpaceKind::ConstructiveElementKind(x) => x.into(),
            OccupiedSpaceKind::FillingElementKind(x) => x.into(),
            OccupiedSpaceKind::InstallationKind(x) => x.into(),
            OccupiedSpaceKind::VegetationObjectKind(x) => x.into(),
        }
    }
}

impl<'a> TryFrom<&'a OccupiedSpaceKind> for TopLevelFeatureRef<'a> {
    type Error = ();
    fn try_from(item: &'a OccupiedSpaceKind) -> Result<Self, ()> {
        match item {
            OccupiedSpaceKind::CityFurniture(x) => Ok(x.into()),
            OccupiedSpaceKind::ConstructionKind(x) => x.try_into(),
            OccupiedSpaceKind::ConstructiveElementKind(_) => Err(()),
            OccupiedSpaceKind::FillingElementKind(_) => Err(()),
            OccupiedSpaceKind::InstallationKind(_) => Err(()),
            OccupiedSpaceKind::VegetationObjectKind(x) => Ok(x.into()),
        }
    }
}

impl<'a> From<&'a mut OccupiedSpaceKind> for FeatureRefMut<'a> {
    fn from(item: &'a mut OccupiedSpaceKind) -> Self {
        match item {
            OccupiedSpaceKind::CityFurniture(x) => x.into(),
            OccupiedSpaceKind::ConstructionKind(x) => x.into(),
            OccupiedSpaceKind::ConstructiveElementKind(x) => x.into(),
            OccupiedSpaceKind::FillingElementKind(x) => x.into(),
            OccupiedSpaceKind::InstallationKind(x) => x.into(),
            OccupiedSpaceKind::VegetationObjectKind(x) => x.into(),
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
impl_from_for_occupied_space_kind!(InstallationKind);
impl_from_for_occupied_space_kind!(VegetationObjectKind);
