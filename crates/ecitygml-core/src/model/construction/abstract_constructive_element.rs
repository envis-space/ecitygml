use crate::model::construction::FillingElementProperty;
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use crate::model::core::{
    AbstractOccupiedSpace, AsAbstractOccupiedSpace, AsAbstractOccupiedSpaceMut,
};
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractConstructiveElement {
    pub(crate) abstract_occupied_space: AbstractOccupiedSpace,
    is_structural_element: Option<bool>,
    fillings: Vec<FillingElementProperty>,
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
impl AbstractConstructiveElement {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        self.abstract_occupied_space.iter_features().chain(
            self.fillings
                .iter()
                .filter_map(|x| x.object.as_ref())
                .flat_map(|x| x.iter_features()),
        )
    }
    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_occupied_space.for_each_feature_mut(f);
        for prop in &mut self.fillings {
            if let Some(x) = prop.object.as_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }
    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_occupied_space.compute_envelope()
    }
    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_occupied_space.apply_transform(m);

        for prop in &mut self.fillings {
            if let Some(x) = prop.object.as_mut() {
                x.apply_transform(m);
            }
        }
    }
}

pub trait AsAbstractConstructiveElement: AsAbstractOccupiedSpace {
    fn abstract_constructive_element(&self) -> &AbstractConstructiveElement;

    fn is_structural_element(&self) -> Option<bool> {
        self.abstract_constructive_element().is_structural_element
    }

    fn fillings(&self) -> &[FillingElementProperty] {
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

    fn set_fillings(&mut self, values: Vec<FillingElementProperty>) {
        self.abstract_constructive_element_mut().fillings = values;
    }

    fn push_filling(&mut self, filling: FillingElementProperty) {
        self.abstract_constructive_element_mut()
            .fillings
            .push(filling);
    }

    fn extend_fillings(&mut self, fillings: impl IntoIterator<Item = FillingElementProperty>) {
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
                use $crate::model::construction::AsAbstractConstructiveElement;
                &self.abstract_constructive_element().abstract_occupied_space
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
                use $crate::model::construction::AsAbstractConstructiveElementMut;
                &mut self
                    .abstract_constructive_element_mut()
                    .abstract_occupied_space
            }
        }
    };
}

impl_abstract_constructive_element_traits!(AbstractConstructiveElement);
impl_abstract_constructive_element_mut_traits!(AbstractConstructiveElement);
