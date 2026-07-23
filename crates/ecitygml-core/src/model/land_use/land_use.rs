use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractThematicSurface, AsAbstractThematicSurface, AsAbstractThematicSurfaceMut,
};
use crate::model::land_use::values::{LandUseClassValue, LandUseFunctionValue, LandUseUsageValue};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct LandUse {
    pub(crate) abstract_thematic_surface: AbstractThematicSurface,
    class: Option<LandUseClassValue>,
    functions: Vec<LandUseFunctionValue>,
    usages: Vec<LandUseUsageValue>,
}

impl LandUse {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_thematic_surface(AbstractThematicSurface::new(id))
    }

    pub fn from_abstract_thematic_surface(
        abstract_thematic_surface: AbstractThematicSurface,
    ) -> Self {
        Self {
            abstract_thematic_surface,
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
        }
    }

    pub fn class(&self) -> Option<&LandUseClassValue> {
        self.class.as_ref()
    }

    pub fn set_class(&mut self, class: LandUseClassValue) {
        self.class = Some(class);
    }

    pub fn set_class_opt(&mut self, class: Option<LandUseClassValue>) {
        self.class = class;
    }

    pub fn clear_class(&mut self) {
        self.class = None;
    }

    pub fn functions(&self) -> &[LandUseFunctionValue] {
        &self.functions
    }

    pub fn functions_mut(&mut self) -> &mut [LandUseFunctionValue] {
        &mut self.functions
    }

    pub fn set_functions(&mut self, functions: Vec<LandUseFunctionValue>) {
        self.functions = functions;
    }

    pub fn push_function(&mut self, function: LandUseFunctionValue) {
        self.functions.push(function);
    }

    pub fn extend_functions(&mut self, functions: impl IntoIterator<Item = LandUseFunctionValue>) {
        self.functions.extend(functions);
    }

    pub fn usages(&self) -> &[LandUseUsageValue] {
        &self.usages
    }

    pub fn usages_mut(&mut self) -> &mut [LandUseUsageValue] {
        &mut self.usages
    }

    pub fn set_usages(&mut self, usages: Vec<LandUseUsageValue>) {
        self.usages = usages;
    }

    pub fn push_usage(&mut self, usage: LandUseUsageValue) {
        self.usages.push(usage);
    }

    pub fn extend_usages(&mut self, usages: impl IntoIterator<Item = LandUseUsageValue>) {
        self.usages.extend(usages);
    }
}

impl AsAbstractThematicSurface for LandUse {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface {
        &self.abstract_thematic_surface
    }
}

impl AsAbstractThematicSurfaceMut for LandUse {
    fn abstract_thematic_surface_mut(&mut self) -> &mut AbstractThematicSurface {
        &mut self.abstract_thematic_surface
    }
}

crate::impl_abstract_thematic_surface_traits!(LandUse);
crate::impl_abstract_thematic_surface_mut_traits!(LandUse);
crate::impl_has_feature_type!(LandUse, LandUse);

impl IterFeatures for LandUse {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(std::iter::once(self.into()).chain(self.abstract_thematic_surface.iter_features()))
    }
}

impl ForEachFeatureMut for LandUse {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_thematic_surface.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for LandUse {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_thematic_surface.compute_envelope()
    }
}

impl ApplyTransform for LandUse {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_thematic_surface.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_thematic_surface.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_thematic_surface.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_thematic_surface.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_thematic_surface.apply_scale(scale);
    }
}
