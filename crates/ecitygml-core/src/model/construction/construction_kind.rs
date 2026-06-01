use crate::impl_abstract_construction_traits;
use crate::model::building::BuildingKind;
use crate::model::common::{FeatureRef, FeatureRefMut, TopLevelFeatureRef};
use crate::model::construction::{
    AbstractConstruction, AsAbstractConstruction, AsAbstractConstructionMut,
};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum ConstructionKind {
    BuildingKind(BuildingKind),
}

impl ConstructionKind {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        match self {
            ConstructionKind::BuildingKind(x) => x.iter_features(),
        }
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
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

impl<'a> From<&'a ConstructionKind> for FeatureRef<'a> {
    fn from(item: &'a ConstructionKind) -> Self {
        match item {
            ConstructionKind::BuildingKind(x) => x.into(),
        }
    }
}

impl<'a> TryFrom<&'a ConstructionKind> for TopLevelFeatureRef<'a> {
    type Error = ();
    fn try_from(item: &'a ConstructionKind) -> Result<Self, ()> {
        match item {
            ConstructionKind::BuildingKind(x) => x.try_into(),
        }
    }
}

impl<'a> From<&'a mut ConstructionKind> for FeatureRefMut<'a> {
    fn from(item: &'a mut ConstructionKind) -> Self {
        match item {
            ConstructionKind::BuildingKind(x) => x.into(),
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
