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
pub struct SolitaryVegetationObject {
    pub(crate) abstract_vegetation_object: AbstractVegetationObject,
    class: Option<Code>,
    functions: Vec<Code>,
    usages: Vec<Code>,
    species: Option<Code>,
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

    pub fn species(&self) -> Option<&Code> {
        self.species.as_ref()
    }

    pub fn set_species(&mut self, species: Option<Code>) {
        self.species = species;
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

    pub fn set_height(&mut self, height: Option<Measure>) {
        self.height = height;
    }

    pub fn set_trunk_diameter(&mut self, trunk_diameter: Option<Measure>) {
        self.trunk_diameter = trunk_diameter;
    }

    pub fn set_crown_diameter(&mut self, crown_diameter: Option<Measure>) {
        self.crown_diameter = crown_diameter;
    }

    pub fn set_root_ball_diameter(&mut self, root_ball_diameter: Option<Measure>) {
        self.root_ball_diameter = root_ball_diameter;
    }

    pub fn set_max_root_ball_depth(&mut self, max_root_ball_depth: Option<Measure>) {
        self.max_root_ball_depth = max_root_ball_depth;
    }
}

impl SolitaryVegetationObject {
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
