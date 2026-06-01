use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::core::AsAbstractFeatureMut;
use crate::model::relief::{
    AbstractReliefComponent, AsAbstractReliefComponent, AsAbstractReliefComponentMut,
};
use egml::model::geometry::Envelope;
use egml::model::geometry::primitives::TriangulatedSurface;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct TinRelief {
    pub(crate) abstract_relief_component: AbstractReliefComponent,
    tin: Option<TriangulatedSurface>,
}

impl TinRelief {
    pub fn new(
        abstract_relief_component: AbstractReliefComponent,
        tin: Option<TriangulatedSurface>,
    ) -> Self {
        Self {
            abstract_relief_component,
            tin,
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_relief_component.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_relief_component.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.tin.as_ref().map(|x| x.compute_envelope())
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        if let Some(x) = &mut self.tin {
            x.apply_transform(m);
        }
    }

    pub fn tin(&self) -> &Option<TriangulatedSurface> {
        &self.tin
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

impl<'a> From<&'a TinRelief> for FeatureRef<'a> {
    fn from(item: &'a TinRelief) -> Self {
        FeatureRef::TinRelief(item)
    }
}

impl<'a> From<&'a mut TinRelief> for FeatureRefMut<'a> {
    fn from(item: &'a mut TinRelief) -> Self {
        FeatureRefMut::TinRelief(item)
    }
}
