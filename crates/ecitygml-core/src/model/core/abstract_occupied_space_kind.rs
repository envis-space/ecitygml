use crate::impl_abstract_occupied_space_mut_traits;
use crate::impl_abstract_occupied_space_traits;
use crate::model::city_furniture::CityFurniture;
use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::construction::{
    AbstractConstructionKind, AbstractConstructiveElementKind, AbstractFillingElementKind,
    AbstractInstallationKind,
};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractOccupiedSpace, AsAbstractOccupiedSpace, AsAbstractOccupiedSpaceMut,
};
use crate::model::generics::GenericOccupiedSpace;
use crate::model::vegetation::AbstractVegetationObjectKind;
use crate::model::water_body::WaterBody;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractOccupiedSpaceKind {
    CityFurniture(CityFurniture),
    AbstractConstructionKind(AbstractConstructionKind),
    AbstractConstructiveElementKind(AbstractConstructiveElementKind),
    AbstractFillingElementKind(AbstractFillingElementKind),
    GenericOccupiedSpace(GenericOccupiedSpace),
    AbstractInstallationKind(AbstractInstallationKind),
    AbstractVegetationObjectKind(AbstractVegetationObjectKind),
    WaterBody(WaterBody),
}

impl AsAbstractOccupiedSpace for AbstractOccupiedSpaceKind {
    fn abstract_occupied_space(&self) -> &AbstractOccupiedSpace {
        match self {
            Self::CityFurniture(x) => x.abstract_occupied_space(),
            Self::AbstractConstructionKind(x) => x.abstract_occupied_space(),
            Self::AbstractConstructiveElementKind(x) => x.abstract_occupied_space(),
            Self::AbstractFillingElementKind(x) => x.abstract_occupied_space(),
            Self::GenericOccupiedSpace(x) => x.abstract_occupied_space(),
            Self::AbstractVegetationObjectKind(x) => x.abstract_occupied_space(),
            Self::AbstractInstallationKind(x) => x.abstract_occupied_space(),
            Self::WaterBody(x) => x.abstract_occupied_space(),
        }
    }
}

impl AsAbstractOccupiedSpaceMut for AbstractOccupiedSpaceKind {
    fn abstract_occupied_space_mut(&mut self) -> &mut AbstractOccupiedSpace {
        match self {
            Self::CityFurniture(x) => x.abstract_occupied_space_mut(),
            Self::AbstractConstructionKind(x) => x.abstract_occupied_space_mut(),
            Self::AbstractConstructiveElementKind(x) => x.abstract_occupied_space_mut(),
            Self::AbstractFillingElementKind(x) => x.abstract_occupied_space_mut(),
            Self::GenericOccupiedSpace(x) => x.abstract_occupied_space_mut(),
            Self::AbstractVegetationObjectKind(x) => x.abstract_occupied_space_mut(),
            Self::AbstractInstallationKind(x) => x.abstract_occupied_space_mut(),
            Self::WaterBody(x) => x.abstract_occupied_space_mut(),
        }
    }
}

impl_abstract_occupied_space_traits!(AbstractOccupiedSpaceKind);
impl_abstract_occupied_space_mut_traits!(AbstractOccupiedSpaceKind);

impl HasFeatureType for AbstractOccupiedSpaceKind {
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
macro_rules! impl_from_for_occupied_space_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::core::AbstractOccupiedSpaceKind {
            fn from(x: $type) -> Self {
                $crate::model::core::AbstractOccupiedSpaceKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_physical_space_kind!(AbstractOccupiedSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_occupied_space_kind!($variant, $variant);
    };
}
impl_from_for_occupied_space_kind!(CityFurniture);
impl_from_for_occupied_space_kind!(AbstractConstructionKind);
impl_from_for_occupied_space_kind!(AbstractConstructiveElementKind);
impl_from_for_occupied_space_kind!(AbstractFillingElementKind);
impl_from_for_occupied_space_kind!(GenericOccupiedSpace);
impl_from_for_occupied_space_kind!(AbstractInstallationKind);
impl_from_for_occupied_space_kind!(AbstractVegetationObjectKind);
impl_from_for_occupied_space_kind!(WaterBody);

#[macro_export]
macro_rules! impl_try_from_for_occupied_space_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::core::AbstractOccupiedSpaceKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::core::AbstractOccupiedSpaceKind) -> Result<Self, ()> {
                match x {
                    $crate::model::core::AbstractOccupiedSpaceKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_physical_space_kind!(AbstractOccupiedSpaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_occupied_space_kind!($variant, $variant);
    };
}
impl_try_from_for_occupied_space_kind!(CityFurniture);
impl_try_from_for_occupied_space_kind!(GenericOccupiedSpace);
impl_try_from_for_occupied_space_kind!(WaterBody);

impl IterFeatures for AbstractOccupiedSpaceKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractOccupiedSpaceKind::CityFurniture(x) => x.iter_features(),
            AbstractOccupiedSpaceKind::AbstractConstructionKind(x) => x.iter_features(),
            AbstractOccupiedSpaceKind::AbstractConstructiveElementKind(x) => x.iter_features(),
            AbstractOccupiedSpaceKind::AbstractFillingElementKind(x) => x.iter_features(),
            AbstractOccupiedSpaceKind::GenericOccupiedSpace(x) => x.iter_features(),
            AbstractOccupiedSpaceKind::AbstractInstallationKind(x) => x.iter_features(),
            AbstractOccupiedSpaceKind::AbstractVegetationObjectKind(x) => x.iter_features(),
            AbstractOccupiedSpaceKind::WaterBody(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractOccupiedSpaceKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractOccupiedSpaceKind::CityFurniture(x) => x.for_each_feature_mut(f),
            AbstractOccupiedSpaceKind::AbstractConstructionKind(x) => x.for_each_feature_mut(f),
            AbstractOccupiedSpaceKind::AbstractConstructiveElementKind(x) => {
                x.for_each_feature_mut(f)
            }
            AbstractOccupiedSpaceKind::AbstractFillingElementKind(x) => x.for_each_feature_mut(f),
            AbstractOccupiedSpaceKind::GenericOccupiedSpace(x) => x.for_each_feature_mut(f),
            AbstractOccupiedSpaceKind::AbstractInstallationKind(x) => x.for_each_feature_mut(f),
            AbstractOccupiedSpaceKind::AbstractVegetationObjectKind(x) => x.for_each_feature_mut(f),
            AbstractOccupiedSpaceKind::WaterBody(x) => x.for_each_feature_mut(f),
        }
    }
}

impl ComputeEnvelope for AbstractOccupiedSpaceKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractOccupiedSpaceKind::CityFurniture(x) => x.compute_envelope(),
            AbstractOccupiedSpaceKind::AbstractConstructionKind(x) => x.compute_envelope(),
            AbstractOccupiedSpaceKind::AbstractConstructiveElementKind(x) => x.compute_envelope(),
            AbstractOccupiedSpaceKind::AbstractFillingElementKind(x) => x.compute_envelope(),
            AbstractOccupiedSpaceKind::GenericOccupiedSpace(x) => x.compute_envelope(),
            AbstractOccupiedSpaceKind::AbstractInstallationKind(x) => x.compute_envelope(),
            AbstractOccupiedSpaceKind::AbstractVegetationObjectKind(x) => x.compute_envelope(),
            AbstractOccupiedSpaceKind::WaterBody(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractOccupiedSpaceKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractOccupiedSpaceKind::CityFurniture(x) => x.apply_transform(m),
            AbstractOccupiedSpaceKind::AbstractConstructionKind(x) => x.apply_transform(m),
            AbstractOccupiedSpaceKind::AbstractConstructiveElementKind(x) => x.apply_transform(m),
            AbstractOccupiedSpaceKind::AbstractFillingElementKind(x) => x.apply_transform(m),
            AbstractOccupiedSpaceKind::GenericOccupiedSpace(x) => x.apply_transform(m),
            AbstractOccupiedSpaceKind::AbstractInstallationKind(x) => x.apply_transform(m),
            AbstractOccupiedSpaceKind::AbstractVegetationObjectKind(x) => x.apply_transform(m),
            AbstractOccupiedSpaceKind::WaterBody(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractOccupiedSpaceKind::CityFurniture(x) => x.apply_isometry(isometry),
            AbstractOccupiedSpaceKind::AbstractConstructionKind(x) => x.apply_isometry(isometry),
            AbstractOccupiedSpaceKind::AbstractConstructiveElementKind(x) => {
                x.apply_isometry(isometry)
            }
            AbstractOccupiedSpaceKind::AbstractFillingElementKind(x) => x.apply_isometry(isometry),
            AbstractOccupiedSpaceKind::GenericOccupiedSpace(x) => x.apply_isometry(isometry),
            AbstractOccupiedSpaceKind::AbstractInstallationKind(x) => x.apply_isometry(isometry),
            AbstractOccupiedSpaceKind::AbstractVegetationObjectKind(x) => {
                x.apply_isometry(isometry)
            }
            AbstractOccupiedSpaceKind::WaterBody(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractOccupiedSpaceKind::CityFurniture(x) => x.apply_translation(vector),
            AbstractOccupiedSpaceKind::AbstractConstructionKind(x) => x.apply_translation(vector),
            AbstractOccupiedSpaceKind::AbstractConstructiveElementKind(x) => {
                x.apply_translation(vector)
            }
            AbstractOccupiedSpaceKind::AbstractFillingElementKind(x) => x.apply_translation(vector),
            AbstractOccupiedSpaceKind::GenericOccupiedSpace(x) => x.apply_translation(vector),
            AbstractOccupiedSpaceKind::AbstractInstallationKind(x) => x.apply_translation(vector),
            AbstractOccupiedSpaceKind::AbstractVegetationObjectKind(x) => {
                x.apply_translation(vector)
            }
            AbstractOccupiedSpaceKind::WaterBody(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractOccupiedSpaceKind::CityFurniture(x) => x.apply_rotation(rotation),
            AbstractOccupiedSpaceKind::AbstractConstructionKind(x) => x.apply_rotation(rotation),
            AbstractOccupiedSpaceKind::AbstractConstructiveElementKind(x) => {
                x.apply_rotation(rotation)
            }
            AbstractOccupiedSpaceKind::AbstractFillingElementKind(x) => x.apply_rotation(rotation),
            AbstractOccupiedSpaceKind::GenericOccupiedSpace(x) => x.apply_rotation(rotation),
            AbstractOccupiedSpaceKind::AbstractInstallationKind(x) => x.apply_rotation(rotation),
            AbstractOccupiedSpaceKind::AbstractVegetationObjectKind(x) => {
                x.apply_rotation(rotation)
            }
            AbstractOccupiedSpaceKind::WaterBody(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractOccupiedSpaceKind::CityFurniture(x) => x.apply_scale(scale),
            AbstractOccupiedSpaceKind::AbstractConstructionKind(x) => x.apply_scale(scale),
            AbstractOccupiedSpaceKind::AbstractConstructiveElementKind(x) => x.apply_scale(scale),
            AbstractOccupiedSpaceKind::AbstractFillingElementKind(x) => x.apply_scale(scale),
            AbstractOccupiedSpaceKind::GenericOccupiedSpace(x) => x.apply_scale(scale),
            AbstractOccupiedSpaceKind::AbstractInstallationKind(x) => x.apply_scale(scale),
            AbstractOccupiedSpaceKind::AbstractVegetationObjectKind(x) => x.apply_scale(scale),
            AbstractOccupiedSpaceKind::WaterBody(x) => x.apply_scale(scale),
        }
    }
}
