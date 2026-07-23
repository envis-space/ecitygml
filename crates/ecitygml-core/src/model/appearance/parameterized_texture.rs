use crate::model::appearance::{AbstractTexture, AsAbstractTexture, AsAbstractTextureMut};
use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterizedTexture {
    pub(crate) abstract_texture: AbstractTexture,
}

impl ParameterizedTexture {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_texture(AbstractTexture::new(id))
    }

    pub fn from_abstract_texture(abstract_texture: AbstractTexture) -> Self {
        Self { abstract_texture }
    }
}

impl AsAbstractTexture for ParameterizedTexture {
    fn abstract_texture(&self) -> &AbstractTexture {
        &self.abstract_texture
    }
}

impl AsAbstractTextureMut for ParameterizedTexture {
    fn abstract_texture_mut(&mut self) -> &mut AbstractTexture {
        &mut self.abstract_texture
    }
}

crate::impl_abstract_texture_traits!(ParameterizedTexture);
crate::impl_abstract_texture_mut_traits!(ParameterizedTexture);
crate::impl_has_feature_type!(ParameterizedTexture, ParameterizedTexture);

impl IterFeatures for ParameterizedTexture {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(std::iter::once(self.into()).chain(self.abstract_texture.iter_features()))
    }
}

impl ForEachFeatureMut for ParameterizedTexture {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_texture.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for ParameterizedTexture {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_texture.compute_envelope()
    }
}

impl ApplyTransform for ParameterizedTexture {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_texture.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_texture.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_texture.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_texture.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_texture.apply_scale(scale);
    }
}
