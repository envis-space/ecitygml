use crate::impl_abstract_construction_mut_traits;
use crate::impl_abstract_construction_traits;
use crate::model::building::BuildingKind;
use crate::model::common::{FeatureType, HasFeatureType};
use crate::model::construction::{
    AbstractConstruction, AsAbstractConstruction, AsAbstractConstructionMut,
};
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum ConstructionKind {
    BuildingKind(BuildingKind),
}

impl ConstructionKind {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        match self {
            ConstructionKind::BuildingKind(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            ConstructionKind::BuildingKind(x) => x.for_each_feature_mut(f),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            ConstructionKind::BuildingKind(x) => x.compute_envelope(),
        }
    }

    pub fn recompute_bounding_shape(&mut self) {
        match self {
            ConstructionKind::BuildingKind(x) => x.recompute_bounding_shape(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            ConstructionKind::BuildingKind(x) => x.apply_transform(m),
        }
    }
}

impl AsAbstractConstruction for ConstructionKind {
    fn abstract_construction(&self) -> &AbstractConstruction {
        match self {
            ConstructionKind::BuildingKind(x) => x.abstract_construction(),
        }
    }
}

impl AsAbstractConstructionMut for ConstructionKind {
    fn abstract_construction_mut(&mut self) -> &mut AbstractConstruction {
        match self {
            ConstructionKind::BuildingKind(x) => x.abstract_construction_mut(),
        }
    }
}

impl_abstract_construction_traits!(ConstructionKind);
impl_abstract_construction_mut_traits!(ConstructionKind);

impl HasFeatureType for ConstructionKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::BuildingKind(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_construction_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::construction::ConstructionKind {
            fn from(x: $type) -> Self {
                $crate::model::construction::ConstructionKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_occupied_space_kind!(ConstructionKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_construction_kind!($variant, $variant);
    };
}
impl_from_for_construction_kind!(BuildingKind);

#[macro_export]
macro_rules! impl_try_from_for_construction_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::construction::ConstructionKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::construction::ConstructionKind) -> Result<Self, ()> {
                match x {
                    $crate::model::construction::ConstructionKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_occupied_space_kind!(ConstructionKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_construction_kind!($variant, $variant);
    };
}
impl_try_from_for_construction_kind!(BuildingKind);
