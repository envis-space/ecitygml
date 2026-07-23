use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractUnoccupiedSpace, AsAbstractUnoccupiedSpace, AsAbstractUnoccupiedSpaceMut,
};
use crate::model::transportation::values::ClearanceSpaceClassValue;
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct ClearanceSpace {
    pub(crate) abstract_unoccupied_space: AbstractUnoccupiedSpace,
    class: Option<ClearanceSpaceClassValue>,
}

impl ClearanceSpace {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_unoccupied_space(AbstractUnoccupiedSpace::new(id))
    }

    pub fn from_abstract_unoccupied_space(
        abstract_unoccupied_space: AbstractUnoccupiedSpace,
    ) -> Self {
        Self {
            abstract_unoccupied_space,
            class: None,
        }
    }

    pub fn class(&self) -> Option<&ClearanceSpaceClassValue> {
        self.class.as_ref()
    }

    pub fn set_class(&mut self, class: ClearanceSpaceClassValue) {
        self.class = Some(class);
    }

    pub fn set_class_opt(&mut self, class: Option<ClearanceSpaceClassValue>) {
        self.class = class;
    }

    pub fn clear_class(&mut self) {
        self.class = None;
    }
}

impl AsAbstractUnoccupiedSpace for ClearanceSpace {
    fn abstract_unoccupied_space(&self) -> &AbstractUnoccupiedSpace {
        &self.abstract_unoccupied_space
    }
}

impl AsAbstractUnoccupiedSpaceMut for ClearanceSpace {
    fn abstract_unoccupied_space_mut(&mut self) -> &mut AbstractUnoccupiedSpace {
        &mut self.abstract_unoccupied_space
    }
}

crate::impl_abstract_unoccupied_space_traits!(ClearanceSpace);
crate::impl_abstract_unoccupied_space_mut_traits!(ClearanceSpace);
crate::impl_has_feature_type!(ClearanceSpace, ClearanceSpace);

impl IterFeatures for ClearanceSpace {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(std::iter::once(self.into()).chain(self.abstract_unoccupied_space.iter_features()))
    }
}

impl ForEachFeatureMut for ClearanceSpace {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_unoccupied_space.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for ClearanceSpace {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_unoccupied_space.compute_envelope()
    }
}

impl ApplyTransform for ClearanceSpace {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_unoccupied_space.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_unoccupied_space.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_unoccupied_space.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_unoccupied_space.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_unoccupied_space.apply_scale(scale);
    }
}
