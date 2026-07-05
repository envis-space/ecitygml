use crate::model::core::AsAbstractFeatureMut;
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use crate::model::vegetation::{
    AbstractVegetationObject, AsAbstractVegetationObject, AsAbstractVegetationObjectMut,
};
use egml::model::base::Id;
use egml::model::basic::{Code, Measure};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct PlantCover {
    pub(crate) abstract_vegetation_object: AbstractVegetationObject,
    class: Option<Code>,
    functions: Vec<Code>,
    usages: Vec<Code>,
    average_height: Option<Measure>,
    min_height: Option<Measure>,
    max_height: Option<Measure>,
}

impl PlantCover {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_vegetation_object(AbstractVegetationObject::new(id))
    }

    pub fn from_abstract_vegetation_object(
        abstract_vegetation_object: AbstractVegetationObject,
    ) -> Self {
        Self {
            abstract_vegetation_object,
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
            average_height: None,
            min_height: None,
            max_height: None,
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

    pub fn average_height(&self) -> Option<&Measure> {
        self.average_height.as_ref()
    }

    pub fn set_average_height(&mut self, average_height: Option<Measure>) {
        self.average_height = average_height;
    }

    pub fn min_height(&self) -> Option<&Measure> {
        self.min_height.as_ref()
    }

    pub fn set_min_height(&mut self, min_height: Option<Measure>) {
        self.min_height = min_height;
    }

    pub fn max_height(&self) -> Option<&Measure> {
        self.max_height.as_ref()
    }

    pub fn set_max_height(&mut self, max_height: Option<Measure>) {
        self.max_height = max_height;
    }
}

impl PlantCover {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_vegetation_object.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_vegetation_object.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_vegetation_object.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_vegetation_object.apply_transform(m);
    }
}

impl AsAbstractVegetationObject for PlantCover {
    fn abstract_vegetation_object(&self) -> &AbstractVegetationObject {
        &self.abstract_vegetation_object
    }
}

impl AsAbstractVegetationObjectMut for PlantCover {
    fn abstract_vegetation_object_mut(&mut self) -> &mut AbstractVegetationObject {
        &mut self.abstract_vegetation_object
    }
}

crate::impl_abstract_vegetation_object_traits!(PlantCover);
crate::impl_abstract_vegetation_object_mut_traits!(PlantCover);
crate::impl_has_feature_type!(PlantCover, PlantCover);
