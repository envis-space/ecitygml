use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::construction::AbstractFillingElementProperty;
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractOccupiedSpace, AsAbstractOccupiedSpace, AsAbstractOccupiedSpaceMut,
};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractConstructiveElement {
    pub(crate) abstract_occupied_space: AbstractOccupiedSpace,
    is_structural_element: Option<bool>,
    fillings: Vec<AbstractFillingElementProperty>,
}

impl AbstractConstructiveElement {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_occupied_space(AbstractOccupiedSpace::new(id))
    }

    pub fn from_abstract_occupied_space(abstract_occupied_space: AbstractOccupiedSpace) -> Self {
        Self {
            abstract_occupied_space,
            is_structural_element: None,
            fillings: Vec::new(),
        }
    }
}

pub trait AsAbstractConstructiveElement: AsAbstractOccupiedSpace {
    fn abstract_constructive_element(&self) -> &AbstractConstructiveElement;

    fn is_structural_element(&self) -> Option<bool> {
        self.abstract_constructive_element().is_structural_element
    }

    fn fillings(&self) -> &[AbstractFillingElementProperty] {
        &self.abstract_constructive_element().fillings
    }
}

pub trait AsAbstractConstructiveElementMut:
    AsAbstractOccupiedSpaceMut + AsAbstractConstructiveElement
{
    fn abstract_constructive_element_mut(&mut self) -> &mut AbstractConstructiveElement;

    fn set_is_structural_element(&mut self, is_structural_element: Option<bool>) {
        self.abstract_constructive_element_mut()
            .is_structural_element = is_structural_element;
    }

    fn fillings_mut(&mut self) -> &mut [AbstractFillingElementProperty] {
        &mut self.abstract_constructive_element_mut().fillings
    }

    fn set_fillings(&mut self, values: Vec<AbstractFillingElementProperty>) {
        self.abstract_constructive_element_mut().fillings = values;
    }

    fn push_filling(&mut self, filling: AbstractFillingElementProperty) {
        self.abstract_constructive_element_mut()
            .fillings
            .push(filling);
    }

    fn extend_fillings(
        &mut self,
        fillings: impl IntoIterator<Item = AbstractFillingElementProperty>,
    ) {
        self.abstract_constructive_element_mut()
            .fillings
            .extend(fillings);
    }
}

impl AsAbstractConstructiveElement for AbstractConstructiveElement {
    fn abstract_constructive_element(&self) -> &AbstractConstructiveElement {
        self
    }
}

impl AsAbstractConstructiveElementMut for AbstractConstructiveElement {
    fn abstract_constructive_element_mut(&mut self) -> &mut AbstractConstructiveElement {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_constructive_element_traits {
    ($type:ty) => {
        $crate::impl_abstract_occupied_space_traits!($type);

        impl $crate::model::core::AsAbstractOccupiedSpace for $type {
            fn abstract_occupied_space(&self) -> &$crate::model::core::AbstractOccupiedSpace {
                &<$type as $crate::model::construction::AsAbstractConstructiveElement>::abstract_constructive_element(self).abstract_occupied_space
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_constructive_element_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_occupied_space_mut_traits!($type);

        impl $crate::model::core::AsAbstractOccupiedSpaceMut for $type {
            fn abstract_occupied_space_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractOccupiedSpace {
                &mut <$type as $crate::model::construction::AsAbstractConstructiveElementMut>::abstract_constructive_element_mut(self).abstract_occupied_space
            }
        }
    };
}

impl_abstract_constructive_element_traits!(AbstractConstructiveElement);
impl_abstract_constructive_element_mut_traits!(AbstractConstructiveElement);

impl IterFeatures for AbstractConstructiveElement {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            self.abstract_occupied_space.iter_features().chain(
                self.fillings
                    .iter()
                    .filter_map(|x| x.object())
                    .flat_map(|x| x.iter_features()),
            ),
        )
    }
}

impl ForEachFeatureMut for AbstractConstructiveElement {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_occupied_space.for_each_feature_mut(f);
        for prop in &mut self.fillings {
            if let Some(x) = prop.object_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }
}

impl ComputeEnvelope for AbstractConstructiveElement {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_occupied_space.compute_envelope()
    }
}

impl ApplyTransform for AbstractConstructiveElement {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_occupied_space.apply_transform(m);

        for prop in &mut self.fillings {
            if let Some(x) = prop.object_mut() {
                x.apply_transform(m);
            }
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_occupied_space.apply_isometry(isometry);

        for prop in &mut self.fillings {
            if let Some(x) = prop.object_mut() {
                x.apply_isometry(isometry);
            }
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_occupied_space.apply_translation(vector);

        for prop in &mut self.fillings {
            if let Some(x) = prop.object_mut() {
                x.apply_translation(vector);
            }
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_occupied_space.apply_rotation(rotation);

        for prop in &mut self.fillings {
            if let Some(x) = prop.object_mut() {
                x.apply_rotation(rotation);
            }
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_occupied_space.apply_scale(scale);

        for prop in &mut self.fillings {
            if let Some(x) = prop.object_mut() {
                x.apply_scale(scale);
            }
        }
    }
}
