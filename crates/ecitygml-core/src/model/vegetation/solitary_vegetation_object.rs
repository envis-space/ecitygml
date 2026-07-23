use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::vegetation::values::{
    SolitaryVegetationObjectClassValue, SolitaryVegetationObjectFunctionValue,
    SolitaryVegetationObjectUsageValue, SpeciesValue,
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
pub struct SolitaryVegetationObject {
    pub(crate) abstract_vegetation_object: AbstractVegetationObject,
    class: Option<SolitaryVegetationObjectClassValue>,
    functions: Vec<SolitaryVegetationObjectFunctionValue>,
    usages: Vec<SolitaryVegetationObjectUsageValue>,
    species: Option<SpeciesValue>,
    height: Option<Measure>,
    trunk_diameter: Option<Measure>,
    crown_diameter: Option<Measure>,
    root_ball_diameter: Option<Measure>,
    max_root_ball_depth: Option<Measure>,
}

impl SolitaryVegetationObject {
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
            species: None,
            height: None,
            trunk_diameter: None,
            crown_diameter: None,
            root_ball_diameter: None,
            max_root_ball_depth: None,
        }
    }

    pub fn class(&self) -> Option<&SolitaryVegetationObjectClassValue> {
        self.class.as_ref()
    }

    pub fn set_class(&mut self, class: SolitaryVegetationObjectClassValue) {
        self.class = Some(class);
    }

    pub fn set_class_opt(&mut self, class: Option<SolitaryVegetationObjectClassValue>) {
        self.class = class;
    }

    pub fn clear_class(&mut self) {
        self.class = None;
    }

    pub fn functions(&self) -> &[SolitaryVegetationObjectFunctionValue] {
        &self.functions
    }

    pub fn functions_mut(&mut self) -> &mut [SolitaryVegetationObjectFunctionValue] {
        &mut self.functions
    }

    pub fn set_functions(&mut self, functions: Vec<SolitaryVegetationObjectFunctionValue>) {
        self.functions = functions;
    }

    pub fn push_function(&mut self, function: SolitaryVegetationObjectFunctionValue) {
        self.functions.push(function);
    }

    pub fn extend_functions(
        &mut self,
        functions: impl IntoIterator<Item = SolitaryVegetationObjectFunctionValue>,
    ) {
        self.functions.extend(functions);
    }

    pub fn usages(&self) -> &[SolitaryVegetationObjectUsageValue] {
        &self.usages
    }

    pub fn usages_mut(&mut self) -> &mut [SolitaryVegetationObjectUsageValue] {
        &mut self.usages
    }

    pub fn set_usages(&mut self, usages: Vec<SolitaryVegetationObjectUsageValue>) {
        self.usages = usages;
    }

    pub fn push_usage(&mut self, usage: SolitaryVegetationObjectUsageValue) {
        self.usages.push(usage);
    }

    pub fn extend_usages(
        &mut self,
        usages: impl IntoIterator<Item = SolitaryVegetationObjectUsageValue>,
    ) {
        self.usages.extend(usages);
    }

    pub fn species(&self) -> Option<&SpeciesValue> {
        self.species.as_ref()
    }

    pub fn set_species(&mut self, species: SpeciesValue) {
        self.species = Some(species);
    }

    pub fn set_species_opt(&mut self, species: Option<SpeciesValue>) {
        self.species = species;
    }

    pub fn clear_species(&mut self) {
        self.species = None;
    }

    pub fn height(&self) -> Option<&Measure> {
        self.height.as_ref()
    }

    pub fn trunk_diameter(&self) -> Option<&Measure> {
        self.trunk_diameter.as_ref()
    }

    pub fn crown_diameter(&self) -> Option<&Measure> {
        self.crown_diameter.as_ref()
    }

    pub fn root_ball_diameter(&self) -> Option<&Measure> {
        self.root_ball_diameter.as_ref()
    }

    pub fn max_root_ball_depth(&self) -> Option<&Measure> {
        self.max_root_ball_depth.as_ref()
    }

    pub fn set_height(&mut self, height: Measure) {
        self.height = Some(height);
    }

    pub fn set_height_opt(&mut self, height: Option<Measure>) {
        self.height = height;
    }

    pub fn clear_height(&mut self) {
        self.height = None;
    }

    pub fn set_trunk_diameter(&mut self, trunk_diameter: Measure) {
        self.trunk_diameter = Some(trunk_diameter);
    }

    pub fn set_trunk_diameter_opt(&mut self, trunk_diameter: Option<Measure>) {
        self.trunk_diameter = trunk_diameter;
    }

    pub fn clear_trunk_diameter(&mut self) {
        self.trunk_diameter = None;
    }

    pub fn set_crown_diameter(&mut self, crown_diameter: Measure) {
        self.crown_diameter = Some(crown_diameter);
    }

    pub fn set_crown_diameter_opt(&mut self, crown_diameter: Option<Measure>) {
        self.crown_diameter = crown_diameter;
    }

    pub fn clear_crown_diameter(&mut self) {
        self.crown_diameter = None;
    }

    pub fn set_root_ball_diameter(&mut self, root_ball_diameter: Measure) {
        self.root_ball_diameter = Some(root_ball_diameter);
    }

    pub fn set_root_ball_diameter_opt(&mut self, root_ball_diameter: Option<Measure>) {
        self.root_ball_diameter = root_ball_diameter;
    }

    pub fn clear_root_ball_diameter(&mut self) {
        self.root_ball_diameter = None;
    }

    pub fn set_max_root_ball_depth(&mut self, max_root_ball_depth: Measure) {
        self.max_root_ball_depth = Some(max_root_ball_depth);
    }

    pub fn set_max_root_ball_depth_opt(&mut self, max_root_ball_depth: Option<Measure>) {
        self.max_root_ball_depth = max_root_ball_depth;
    }

    pub fn clear_max_root_ball_depth(&mut self) {
        self.max_root_ball_depth = None;
    }
}

impl AsAbstractVegetationObject for SolitaryVegetationObject {
    fn abstract_vegetation_object(&self) -> &AbstractVegetationObject {
        &self.abstract_vegetation_object
    }
}

impl AsAbstractVegetationObjectMut for SolitaryVegetationObject {
    fn abstract_vegetation_object_mut(&mut self) -> &mut AbstractVegetationObject {
        &mut self.abstract_vegetation_object
    }
}

crate::impl_abstract_vegetation_object_traits!(SolitaryVegetationObject);
crate::impl_abstract_vegetation_object_mut_traits!(SolitaryVegetationObject);
crate::impl_has_feature_type!(SolitaryVegetationObject, SolitaryVegetationObject);

impl IterFeatures for SolitaryVegetationObject {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into()).chain(self.abstract_vegetation_object.iter_features()),
        )
    }
}

impl ForEachFeatureMut for SolitaryVegetationObject {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_vegetation_object.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for SolitaryVegetationObject {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_vegetation_object.compute_envelope()
    }
}

impl ApplyTransform for SolitaryVegetationObject {
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
