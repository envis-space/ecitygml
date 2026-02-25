use crate::model::core::{AsAbstractFeatureMut, CityObjectKind, CityObjectRef};
use crate::model::relief::{
    AbstractReliefComponent, AsAbstractReliefComponent, AsAbstractReliefComponentMut,
};
use crate::operations::{Visitable, Visitor};
use egml::model::geometry::Envelope;
use egml::model::geometry::primitives::TriangulatedSurface;

#[derive(Debug, Clone, PartialEq)]
pub struct TinRelief {
    pub(crate) abstract_relief_component: AbstractReliefComponent,
    tin: TriangulatedSurface,
}

impl TinRelief {
    pub fn new(
        abstract_relief_component: AbstractReliefComponent,
        tin: TriangulatedSurface,
    ) -> Self {
        Self {
            abstract_relief_component,
            tin,
        }
    }

    pub fn tin(&self) -> &TriangulatedSurface {
        &self.tin
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::TinRelief(self))
    }

    pub fn compute_envelope(&self) -> Envelope {
        self.tin.compute_envelope()
    }

    pub fn refresh_bounded_by_recursive(&mut self) {
        let envelope = self.compute_envelope();
        self.set_bounded_by(Some(envelope));
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

impl From<TinRelief> for CityObjectKind {
    fn from(item: TinRelief) -> Self {
        CityObjectKind::TinRelief(item)
    }
}

impl Visitable for TinRelief {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_tin_relief(self);
    }
}
