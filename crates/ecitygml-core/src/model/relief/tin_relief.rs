use crate::model::common::LevelOfDetail;
use crate::model::core::AsAbstractFeatureMut;
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use crate::model::relief::{
    AbstractReliefComponent, AsAbstractReliefComponent, AsAbstractReliefComponentMut, TinProperty,
};
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

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

    pub fn set_tin(&mut self, tin: Option<TinProperty>) {
        self.tin = tin;
    }
}

impl TinRelief {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_relief_component.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_relief_component.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.tin
            .as_ref()
            .and_then(|x| x.object.as_ref())
            .and_then(|x| x.compute_envelope())
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        if let Some(triangulated_surface) = self.tin.as_mut().and_then(|p| p.object.as_mut()) {
            triangulated_surface.apply_transform(m);
        }
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
