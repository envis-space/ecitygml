use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractUnoccupiedSpace, AsAbstractUnoccupiedSpace, AsAbstractUnoccupiedSpaceMut,
};
use crate::model::transportation::values::HoleClassValue;
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct Hole {
    pub(crate) abstract_unoccupied_space: AbstractUnoccupiedSpace,
    class: Option<HoleClassValue>,
}

impl Hole {
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

    pub fn class(&self) -> Option<&HoleClassValue> {
        self.class.as_ref()
    }

    pub fn set_class(&mut self, class: HoleClassValue) {
        self.class = Some(class);
    }

    pub fn set_class_opt(&mut self, class: Option<HoleClassValue>) {
        self.class = class;
    }

    pub fn clear_class(&mut self) {
        self.class = None;
    }
}

impl AsAbstractUnoccupiedSpace for Hole {
    fn abstract_unoccupied_space(&self) -> &AbstractUnoccupiedSpace {
        &self.abstract_unoccupied_space
    }
}

impl AsAbstractUnoccupiedSpaceMut for Hole {
    fn abstract_unoccupied_space_mut(&mut self) -> &mut AbstractUnoccupiedSpace {
        &mut self.abstract_unoccupied_space
    }
}

crate::impl_abstract_unoccupied_space_traits!(Hole);
crate::impl_abstract_unoccupied_space_mut_traits!(Hole);
crate::impl_has_feature_type!(Hole, Hole);

impl IterFeatures for Hole {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(std::iter::once(self.into()).chain(self.abstract_unoccupied_space.iter_features()))
    }
}

impl ForEachFeatureMut for Hole {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_unoccupied_space.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for Hole {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_unoccupied_space.compute_envelope()
    }
}

impl ApplyTransform for Hole {
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
