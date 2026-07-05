use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use crate::model::core::{
    AbstractUnoccupiedSpace, AsAbstractFeatureMut, AsAbstractUnoccupiedSpace,
    AsAbstractUnoccupiedSpaceMut,
};
use crate::model::transportation::GranularityValue;
use egml::model::base::Id;
use egml::model::basic::Code;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AuxiliaryTrafficSpace {
    pub(crate) abstract_unoccupied_space: AbstractUnoccupiedSpace,
    class: Option<Code>,
    functions: Vec<Code>,
    usages: Vec<Code>,
    granularity: GranularityValue,
}

impl AuxiliaryTrafficSpace {
    pub fn new(id: Id, granularity: GranularityValue) -> Self {
        Self::from_abstract_unoccupied_space(AbstractUnoccupiedSpace::new(id), granularity)
    }

    pub fn from_abstract_unoccupied_space(
        abstract_unoccupied_space: AbstractUnoccupiedSpace,
        granularity: GranularityValue,
    ) -> Self {
        Self {
            abstract_unoccupied_space,
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
            granularity,
        }
    }

    pub fn class(&self) -> Option<&Code> {
        self.class.as_ref()
    }

    pub fn set_class(&mut self, class: Option<Code>) {
        self.class = class;
    }

    pub fn functions(&self) -> &[Code] {
        &self.functions
    }

    pub fn set_functions(&mut self, functions: Vec<Code>) {
        self.functions = functions;
    }

    pub fn push_function(&mut self, function: Code) {
        self.functions.push(function);
    }

    pub fn extend_functions(&mut self, functions: impl IntoIterator<Item = Code>) {
        self.functions.extend(functions);
    }

    pub fn usages(&self) -> &[Code] {
        &self.usages
    }

    pub fn set_usages(&mut self, usages: Vec<Code>) {
        self.usages = usages;
    }

    pub fn push_usage(&mut self, usage: Code) {
        self.usages.push(usage);
    }

    pub fn extend_usages(&mut self, usages: impl IntoIterator<Item = Code>) {
        self.usages.extend(usages);
    }

    pub fn granularity(&self) -> &GranularityValue {
        &self.granularity
    }

    pub fn set_granularity(&mut self, granularity: GranularityValue) {
        self.granularity = granularity;
    }
}

impl AuxiliaryTrafficSpace {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_unoccupied_space.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_unoccupied_space.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_unoccupied_space.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_unoccupied_space.apply_transform(m);
    }
}

impl AsAbstractUnoccupiedSpace for AuxiliaryTrafficSpace {
    fn abstract_unoccupied_space(&self) -> &AbstractUnoccupiedSpace {
        &self.abstract_unoccupied_space
    }
}

impl AsAbstractUnoccupiedSpaceMut for AuxiliaryTrafficSpace {
    fn abstract_unoccupied_space_mut(&mut self) -> &mut AbstractUnoccupiedSpace {
        &mut self.abstract_unoccupied_space
    }
}

crate::impl_abstract_unoccupied_space_traits!(AuxiliaryTrafficSpace);
crate::impl_abstract_unoccupied_space_mut_traits!(AuxiliaryTrafficSpace);
crate::impl_has_feature_type!(AuxiliaryTrafficSpace, AuxiliaryTrafficSpace);
