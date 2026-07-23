use crate::model::common::{ForEachFeatureMut, IterFeatures, LevelOfDetail};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::relief::{
    AbstractReliefComponent, AsAbstractReliefComponent, AsAbstractReliefComponentMut, TinProperty,
};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct TinRelief {
    pub(crate) abstract_relief_component: AbstractReliefComponent,
    tin: Option<TinProperty>,
}

impl TinRelief {
    pub fn new(id: Id, lod: LevelOfDetail) -> Self {
        Self::from_abstract_relief_component(AbstractReliefComponent::new(id, lod))
    }

    pub fn from_abstract_relief_component(
        abstract_relief_component: AbstractReliefComponent,
    ) -> Self {
        Self {
            abstract_relief_component,
            tin: None,
        }
    }

    pub fn tin(&self) -> Option<&TinProperty> {
        self.tin.as_ref()
    }

    pub fn set_tin(&mut self, tin: TinProperty) {
        self.tin = Some(tin);
    }

    pub fn set_tin_opt(&mut self, tin: Option<TinProperty>) {
        self.tin = tin;
    }

    pub fn clear_tin(&mut self) {
        self.tin = None;
    }
}

impl AsAbstractReliefComponent for TinRelief {
    fn abstract_relief_component(&self) -> &AbstractReliefComponent {
        &self.abstract_relief_component
    }
}

impl AsAbstractReliefComponentMut for TinRelief {
    fn abstract_relief_component_mut(&mut self) -> &mut AbstractReliefComponent {
        &mut self.abstract_relief_component
    }
}

crate::impl_abstract_relief_component_traits!(TinRelief);
crate::impl_abstract_relief_component_mut_traits!(TinRelief);
crate::impl_has_feature_type!(TinRelief, TinRelief);

impl IterFeatures for TinRelief {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(std::iter::once(self.into()).chain(self.abstract_relief_component.iter_features()))
    }
}

impl ForEachFeatureMut for TinRelief {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_relief_component.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for TinRelief {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.tin
            .as_ref()
            .and_then(|x| x.object())
            .and_then(|x| x.compute_envelope())
    }
}

impl ApplyTransform for TinRelief {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        if let Some(triangulated_surface) = self.tin.as_mut().and_then(|p| p.object_mut()) {
            triangulated_surface.apply_transform(m);
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        if let Some(triangulated_surface) = self.tin.as_mut().and_then(|p| p.object_mut()) {
            triangulated_surface.apply_isometry(isometry);
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        if let Some(triangulated_surface) = self.tin.as_mut().and_then(|p| p.object_mut()) {
            triangulated_surface.apply_translation(vector);
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        if let Some(triangulated_surface) = self.tin.as_mut().and_then(|p| p.object_mut()) {
            triangulated_surface.apply_rotation(rotation);
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        if let Some(triangulated_surface) = self.tin.as_mut().and_then(|p| p.object_mut()) {
            triangulated_surface.apply_scale(scale);
        }
    }
}
