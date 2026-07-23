use crate::impl_abstract_surface_data_mut_traits;
use crate::impl_abstract_surface_data_traits;
use crate::model::appearance::{
    AbstractSurfaceData, AbstractTextureKind, AsAbstractSurfaceData, AsAbstractSurfaceDataMut,
    X3DMaterial,
};
use crate::model::common::{FeatureType, ForEachFeatureMut, HasFeatureType, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractSurfaceDataKind {
    AbstractTextureKind(AbstractTextureKind),
    X3DMaterial(X3DMaterial),
}

impl AsAbstractSurfaceData for AbstractSurfaceDataKind {
    fn abstract_surface_data(&self) -> &AbstractSurfaceData {
        match self {
            AbstractSurfaceDataKind::AbstractTextureKind(x) => x.abstract_surface_data(),
            AbstractSurfaceDataKind::X3DMaterial(x) => x.abstract_surface_data(),
        }
    }
}

impl AsAbstractSurfaceDataMut for AbstractSurfaceDataKind {
    fn abstract_surface_data_mut(&mut self) -> &mut AbstractSurfaceData {
        match self {
            AbstractSurfaceDataKind::AbstractTextureKind(x) => x.abstract_surface_data_mut(),
            AbstractSurfaceDataKind::X3DMaterial(x) => x.abstract_surface_data_mut(),
        }
    }
}

impl_abstract_surface_data_traits!(AbstractSurfaceDataKind);
impl_abstract_surface_data_mut_traits!(AbstractSurfaceDataKind);

impl HasFeatureType for AbstractSurfaceDataKind {
    fn feature_type(&self) -> FeatureType {
        match self {
            Self::AbstractTextureKind(x) => x.feature_type(),
            Self::X3DMaterial(x) => x.feature_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_surface_data_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::appearance::AbstractSurfaceDataKind {
            fn from(x: $type) -> Self {
                $crate::model::appearance::AbstractSurfaceDataKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_feature_kind!(AbstractSurfaceDataKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_surface_data_kind!($variant, $variant);
    };
}
impl_from_for_surface_data_kind!(AbstractTextureKind);
impl_from_for_surface_data_kind!(X3DMaterial);

#[macro_export]
macro_rules! impl_try_from_for_surface_data_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::appearance::AbstractSurfaceDataKind> for $type {
            type Error = ();
            fn try_from(x: $crate::model::appearance::AbstractSurfaceDataKind) -> Result<Self, ()> {
                match x {
                    $crate::model::appearance::AbstractSurfaceDataKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_surface_data_kind!($variant, $variant);
    };
}
impl_try_from_for_surface_data_kind!(X3DMaterial);

impl IterFeatures for AbstractSurfaceDataKind {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        match self {
            AbstractSurfaceDataKind::AbstractTextureKind(x) => x.iter_features(),
            AbstractSurfaceDataKind::X3DMaterial(x) => x.iter_features(),
        }
    }
}

impl ForEachFeatureMut for AbstractSurfaceDataKind {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        match self {
            AbstractSurfaceDataKind::AbstractTextureKind(x) => x.for_each_feature_mut(f),
            AbstractSurfaceDataKind::X3DMaterial(x) => x.for_each_feature_mut(f),
        }
    }
}

impl ComputeEnvelope for AbstractSurfaceDataKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractSurfaceDataKind::AbstractTextureKind(x) => x.compute_envelope(),
            AbstractSurfaceDataKind::X3DMaterial(x) => x.compute_envelope(),
        }
    }
}

impl ApplyTransform for AbstractSurfaceDataKind {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        match self {
            AbstractSurfaceDataKind::AbstractTextureKind(x) => x.apply_transform(m),
            AbstractSurfaceDataKind::X3DMaterial(x) => x.apply_transform(m),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractSurfaceDataKind::AbstractTextureKind(x) => x.apply_isometry(isometry),
            AbstractSurfaceDataKind::X3DMaterial(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractSurfaceDataKind::AbstractTextureKind(x) => x.apply_translation(vector),
            AbstractSurfaceDataKind::X3DMaterial(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractSurfaceDataKind::AbstractTextureKind(x) => x.apply_rotation(rotation),
            AbstractSurfaceDataKind::X3DMaterial(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractSurfaceDataKind::AbstractTextureKind(x) => x.apply_scale(scale),
            AbstractSurfaceDataKind::X3DMaterial(x) => x.apply_scale(scale),
        }
    }
}
