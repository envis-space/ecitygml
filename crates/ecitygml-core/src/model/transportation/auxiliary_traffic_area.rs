use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractThematicSurface, AsAbstractThematicSurface, AsAbstractThematicSurfaceMut,
};
use crate::model::transportation::values::{
    AuxiliaryTrafficAreaClassValue, AuxiliaryTrafficAreaFunctionValue,
    AuxiliaryTrafficAreaUsageValue, SurfaceMaterialValue,
};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct AuxiliaryTrafficArea {
    pub(crate) abstract_thematic_surface: AbstractThematicSurface,
    class: Option<AuxiliaryTrafficAreaClassValue>,
    functions: Vec<AuxiliaryTrafficAreaFunctionValue>,
    usages: Vec<AuxiliaryTrafficAreaUsageValue>,
    surface_material: Option<SurfaceMaterialValue>,
}

impl AuxiliaryTrafficArea {
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
            surface_material: None,
        }
    }

    pub fn class(&self) -> Option<&AuxiliaryTrafficAreaClassValue> {
        self.class.as_ref()
    }

    pub fn set_class(&mut self, class: AuxiliaryTrafficAreaClassValue) {
        self.class = Some(class);
    }

    pub fn set_class_opt(&mut self, class: Option<AuxiliaryTrafficAreaClassValue>) {
        self.class = class;
    }

    pub fn clear_class(&mut self) {
        self.class = None;
    }

    pub fn functions(&self) -> &[AuxiliaryTrafficAreaFunctionValue] {
        &self.functions
    }

    pub fn functions_mut(&mut self) -> &mut [AuxiliaryTrafficAreaFunctionValue] {
        &mut self.functions
    }

    pub fn set_functions(&mut self, functions: Vec<AuxiliaryTrafficAreaFunctionValue>) {
        self.functions = functions;
    }

    pub fn push_function(&mut self, function: AuxiliaryTrafficAreaFunctionValue) {
        self.functions.push(function);
    }

    pub fn extend_functions(
        &mut self,
        functions: impl IntoIterator<Item = AuxiliaryTrafficAreaFunctionValue>,
    ) {
        self.functions.extend(functions);
    }

    pub fn usages(&self) -> &[AuxiliaryTrafficAreaUsageValue] {
        &self.usages
    }

    pub fn usages_mut(&mut self) -> &mut [AuxiliaryTrafficAreaUsageValue] {
        &mut self.usages
    }

    pub fn set_usages(&mut self, usages: Vec<AuxiliaryTrafficAreaUsageValue>) {
        self.usages = usages;
    }

    pub fn push_usage(&mut self, usage: AuxiliaryTrafficAreaUsageValue) {
        self.usages.push(usage);
    }

    pub fn extend_usages(
        &mut self,
        usages: impl IntoIterator<Item = AuxiliaryTrafficAreaUsageValue>,
    ) {
        self.usages.extend(usages);
    }

    pub fn surface_material(&self) -> Option<&SurfaceMaterialValue> {
        self.surface_material.as_ref()
    }

    pub fn set_surface_material(&mut self, surface_material: SurfaceMaterialValue) {
        self.surface_material = Some(surface_material);
    }

    pub fn set_surface_material_opt(&mut self, surface_material: Option<SurfaceMaterialValue>) {
        self.surface_material = surface_material;
    }

    pub fn clear_surface_material(&mut self) {
        self.surface_material = None;
    }
}

impl AsAbstractThematicSurface for AuxiliaryTrafficArea {
    fn abstract_thematic_surface(&self) -> &AbstractThematicSurface {
        &self.abstract_thematic_surface
    }
}

impl AsAbstractThematicSurfaceMut for AuxiliaryTrafficArea {
    fn abstract_thematic_surface_mut(&mut self) -> &mut AbstractThematicSurface {
        &mut self.abstract_thematic_surface
    }
}

crate::impl_abstract_thematic_surface_traits!(AuxiliaryTrafficArea);
crate::impl_abstract_thematic_surface_mut_traits!(AuxiliaryTrafficArea);
crate::impl_has_feature_type!(AuxiliaryTrafficArea, AuxiliaryTrafficArea);

impl IterFeatures for AuxiliaryTrafficArea {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(std::iter::once(self.into()).chain(self.abstract_thematic_surface.iter_features()))
    }
}

impl ForEachFeatureMut for AuxiliaryTrafficArea {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_thematic_surface.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for AuxiliaryTrafficArea {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_thematic_surface.compute_envelope()
    }
}

impl ApplyTransform for AuxiliaryTrafficArea {
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
