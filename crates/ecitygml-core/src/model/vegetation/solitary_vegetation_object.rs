use crate::model::common::{FeatureRef, FeatureRefMut, TopLevelFeatureRef};
use crate::model::core::AsAbstractFeatureMut;
use crate::model::vegetation::{
    AbstractVegetationObject, AsAbstractVegetationObject, AsAbstractVegetationObjectMut,
};
use egml::model::basic::Measure;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct SolitaryVegetationObject {
    pub(crate) abstract_vegetation_object: AbstractVegetationObject,
    pub(crate) height: Option<Measure>,
    pub(crate) trunk_diameter: Option<Measure>,
}

impl SolitaryVegetationObject {
    pub fn new(abstract_vegetation_object: AbstractVegetationObject) -> Self {
        Self {
            abstract_vegetation_object,
            height: None,
            trunk_diameter: None,
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_vegetation_object.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
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

    pub fn height(&self) -> Option<&Measure> {
        self.height.as_ref()
    }

    pub fn trunk_diameter(&self) -> Option<&Measure> {
        self.trunk_diameter.as_ref()
    }

    pub fn set_height(&mut self, height: Option<Measure>) {
        self.height = height;
    }

    pub fn set_trunk_diameter(&mut self, trunk_diameter: Option<Measure>) {
        self.trunk_diameter = trunk_diameter;
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

impl<'a> From<&'a SolitaryVegetationObject> for FeatureRef<'a> {
    fn from(item: &'a SolitaryVegetationObject) -> Self {
        FeatureRef::SolitaryVegetationObject(item)
    }
}

impl<'a> From<&'a mut SolitaryVegetationObject> for FeatureRefMut<'a> {
    fn from(item: &'a mut SolitaryVegetationObject) -> Self {
        FeatureRefMut::SolitaryVegetationObject(item)
    }
}

impl<'a> From<&'a SolitaryVegetationObject> for TopLevelFeatureRef<'a> {
    fn from(item: &'a SolitaryVegetationObject) -> Self {
        TopLevelFeatureRef::SolitaryVegetationObject(item)
    }
}
