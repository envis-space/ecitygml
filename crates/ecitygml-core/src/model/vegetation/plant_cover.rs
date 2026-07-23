use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::vegetation::values::{
    PlantCoverClassValue, PlantCoverFunctionValue, PlantCoverUsageValue,
};
use crate::model::vegetation::{
    AbstractVegetationObject, AsAbstractVegetationObject, AsAbstractVegetationObjectMut,
};
use egml::model::base::Id;
use egml::model::basic_types::Measure;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct PlantCover {
    pub(crate) abstract_vegetation_object: AbstractVegetationObject,
    class: Option<PlantCoverClassValue>,
    functions: Vec<PlantCoverFunctionValue>,
    usages: Vec<PlantCoverUsageValue>,
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

    pub fn class(&self) -> Option<&PlantCoverClassValue> {
        self.class.as_ref()
    }

    pub fn set_class(&mut self, class: PlantCoverClassValue) {
        self.class = Some(class);
    }

    pub fn set_class_opt(&mut self, class: Option<PlantCoverClassValue>) {
        self.class = class;
    }

    pub fn clear_class(&mut self) {
        self.class = None;
    }

    pub fn functions(&self) -> &[PlantCoverFunctionValue] {
        &self.functions
    }

    pub fn functions_mut(&mut self) -> &mut [PlantCoverFunctionValue] {
        &mut self.functions
    }

    pub fn set_functions(&mut self, functions: Vec<PlantCoverFunctionValue>) {
        self.functions = functions;
    }

    pub fn push_function(&mut self, function: PlantCoverFunctionValue) {
        self.functions.push(function);
    }

    pub fn extend_functions(
        &mut self,
        functions: impl IntoIterator<Item = PlantCoverFunctionValue>,
    ) {
        self.functions.extend(functions);
    }

    pub fn usages(&self) -> &[PlantCoverUsageValue] {
        &self.usages
    }

    pub fn usages_mut(&mut self) -> &mut [PlantCoverUsageValue] {
        &mut self.usages
    }

    pub fn set_usages(&mut self, usages: Vec<PlantCoverUsageValue>) {
        self.usages = usages;
    }

    pub fn push_usage(&mut self, usage: PlantCoverUsageValue) {
        self.usages.push(usage);
    }

    pub fn extend_usages(&mut self, usages: impl IntoIterator<Item = PlantCoverUsageValue>) {
        self.usages.extend(usages);
    }

    pub fn average_height(&self) -> Option<&Measure> {
        self.average_height.as_ref()
    }

    pub fn set_average_height(&mut self, average_height: Measure) {
        self.average_height = Some(average_height);
    }

    pub fn set_average_height_opt(&mut self, average_height: Option<Measure>) {
        self.average_height = average_height;
    }

    pub fn clear_average_height(&mut self) {
        self.average_height = None;
    }

    pub fn min_height(&self) -> Option<&Measure> {
        self.min_height.as_ref()
    }

    pub fn set_min_height(&mut self, min_height: Measure) {
        self.min_height = Some(min_height);
    }

    pub fn set_min_height_opt(&mut self, min_height: Option<Measure>) {
        self.min_height = min_height;
    }

    pub fn clear_min_height(&mut self) {
        self.min_height = None;
    }

    pub fn max_height(&self) -> Option<&Measure> {
        self.max_height.as_ref()
    }

    pub fn set_max_height(&mut self, max_height: Measure) {
        self.max_height = Some(max_height);
    }

    pub fn set_max_height_opt(&mut self, max_height: Option<Measure>) {
        self.max_height = max_height;
    }

    pub fn clear_max_height(&mut self) {
        self.max_height = None;
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

impl IterFeatures for PlantCover {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into()).chain(self.abstract_vegetation_object.iter_features()),
        )
    }
}

impl ForEachFeatureMut for PlantCover {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_vegetation_object.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for PlantCover {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_vegetation_object.compute_envelope()
    }
}

impl ApplyTransform for PlantCover {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_vegetation_object.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_vegetation_object.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_vegetation_object.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_vegetation_object.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_vegetation_object.apply_scale(scale);
    }
}
