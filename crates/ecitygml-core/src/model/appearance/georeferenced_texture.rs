use crate::model::appearance::{AbstractTexture, AsAbstractTexture, AsAbstractTextureMut};
use crate::model::core::AsAbstractFeatureMut;
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct GeoreferencedTexture {
    pub(crate) abstract_texture: AbstractTexture,
    prefer_world_file: Option<bool>,
}

impl GeoreferencedTexture {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_texture(AbstractTexture::new(id))
    }

    pub fn from_abstract_texture(abstract_texture: AbstractTexture) -> Self {
        Self {
            abstract_texture,
            prefer_world_file: None,
        }
    }

    pub fn prefer_world_file(&self) -> Option<bool> {
        self.prefer_world_file
    }

    pub fn set_prefer_world_file(&mut self, prefer_world_file: Option<bool>) {
        self.prefer_world_file = prefer_world_file;
    }
}

impl GeoreferencedTexture {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_texture.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_texture.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_texture.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_texture.apply_transform(m);
    }
}

impl AsAbstractTexture for GeoreferencedTexture {
    fn abstract_texture(&self) -> &AbstractTexture {
        &self.abstract_texture
    }
}

impl AsAbstractTextureMut for GeoreferencedTexture {
    fn abstract_texture_mut(&mut self) -> &mut AbstractTexture {
        &mut self.abstract_texture
    }
}

crate::impl_abstract_texture_traits!(GeoreferencedTexture);
crate::impl_abstract_texture_mut_traits!(GeoreferencedTexture);
crate::impl_has_feature_type!(GeoreferencedTexture, GeoreferencedTexture);
