use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::construction::{
    AbstractConstructionSurface, AsAbstractConstructionSurface, AsAbstractConstructionSurfaceMut,
};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct OuterFloorSurface {
    pub(crate) abstract_construction_surface: AbstractConstructionSurface,
}

impl OuterFloorSurface {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_construction_surface(AbstractConstructionSurface::new(id))
    }

    pub fn from_abstract_construction_surface(
        abstract_construction_surface: AbstractConstructionSurface,
    ) -> Self {
        Self {
            abstract_construction_surface,
        }
    }
}

impl AsAbstractConstructionSurface for OuterFloorSurface {
    fn abstract_construction_surface(&self) -> &AbstractConstructionSurface {
        &self.abstract_construction_surface
    }
}

impl AsAbstractConstructionSurfaceMut for OuterFloorSurface {
    fn abstract_construction_surface_mut(&mut self) -> &mut AbstractConstructionSurface {
        &mut self.abstract_construction_surface
    }
}

crate::impl_abstract_construction_surface_traits!(OuterFloorSurface);
crate::impl_abstract_construction_surface_mut_traits!(OuterFloorSurface);
crate::impl_has_feature_type!(OuterFloorSurface, OuterFloorSurface);

impl IterFeatures for OuterFloorSurface {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into()).chain(self.abstract_construction_surface.iter_features()),
        )
    }
}

impl ForEachFeatureMut for OuterFloorSurface {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_construction_surface.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for OuterFloorSurface {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_construction_surface.compute_envelope()
    }
}

impl ApplyTransform for OuterFloorSurface {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_construction_surface.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_construction_surface.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_construction_surface.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_construction_surface.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_construction_surface.apply_scale(scale);
    }
}
