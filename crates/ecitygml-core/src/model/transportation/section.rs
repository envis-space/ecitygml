use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::transportation::values::SectionClassValue;
use crate::model::transportation::{
    AbstractTransportationSpace, AsAbstractTransportationSpace, AsAbstractTransportationSpaceMut,
};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct Section {
    pub(crate) abstract_transportation_space: AbstractTransportationSpace,
    class: Option<SectionClassValue>,
}

impl Section {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_transportation_space(AbstractTransportationSpace::new(id))
    }

    pub fn from_abstract_transportation_space(
        abstract_transportation_space: AbstractTransportationSpace,
    ) -> Self {
        Self {
            abstract_transportation_space,
            class: None,
        }
    }

    pub fn class(&self) -> Option<&SectionClassValue> {
        self.class.as_ref()
    }

    pub fn set_class(&mut self, class: SectionClassValue) {
        self.class = Some(class);
    }

    pub fn set_class_opt(&mut self, class: Option<SectionClassValue>) {
        self.class = class;
    }

    pub fn clear_class(&mut self) {
        self.class = None;
    }
}

impl AsAbstractTransportationSpace for Section {
    fn abstract_transportation_space(&self) -> &AbstractTransportationSpace {
        &self.abstract_transportation_space
    }
}

impl AsAbstractTransportationSpaceMut for Section {
    fn abstract_transportation_space_mut(&mut self) -> &mut AbstractTransportationSpace {
        &mut self.abstract_transportation_space
    }
}

crate::impl_abstract_transportation_space_traits!(Section);
crate::impl_abstract_transportation_space_mut_traits!(Section);
crate::impl_has_feature_type!(Section, Section);

impl IterFeatures for Section {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into()).chain(self.abstract_transportation_space.iter_features()),
        )
    }
}

impl ForEachFeatureMut for Section {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_transportation_space.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for Section {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_transportation_space.compute_envelope()
    }
}

impl ApplyTransform for Section {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_transportation_space.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_transportation_space.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_transportation_space.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_transportation_space.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_transportation_space.apply_scale(scale);
    }
}
