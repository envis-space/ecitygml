use crate::model::appearance::basic_types::ColorPlusOpacity;
use crate::model::appearance::enums::{TextureType, WrapMode};
use crate::model::appearance::{
    AbstractSurfaceData, AsAbstractSurfaceData, AsAbstractSurfaceDataMut,
};
use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::{AbstractFeatureKindRef, AbstractFeatureKindRefMut};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractTexture {
    pub(crate) abstract_surface_data: AbstractSurfaceData,
    image_uri: Option<String>,
    texture_type: Option<TextureType>,
    wrap_mode: Option<WrapMode>,
    border_color: Option<ColorPlusOpacity>,
}

impl AbstractTexture {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_surface_data(AbstractSurfaceData::new(id))
    }

    pub fn from_abstract_surface_data(abstract_surface_data: AbstractSurfaceData) -> Self {
        Self {
            abstract_surface_data,
            image_uri: None,
            texture_type: None,
            wrap_mode: None,
            border_color: None,
        }
    }
}

pub trait AsAbstractTexture: AsAbstractSurfaceData {
    fn abstract_texture(&self) -> &AbstractTexture;

    fn image_uri(&self) -> Option<&str> {
        self.abstract_texture().image_uri.as_deref()
    }

    fn texture_type(&self) -> Option<TextureType> {
        self.abstract_texture().texture_type
    }

    fn wrap_mode(&self) -> Option<WrapMode> {
        self.abstract_texture().wrap_mode
    }

    fn border_color(&self) -> Option<ColorPlusOpacity> {
        self.abstract_texture().border_color
    }
}

pub trait AsAbstractTextureMut: AsAbstractSurfaceDataMut + AsAbstractTexture {
    fn abstract_texture_mut(&mut self) -> &mut AbstractTexture;

    fn set_image_uri(&mut self, image_uri: Option<String>) {
        self.abstract_texture_mut().image_uri = image_uri;
    }

    fn set_texture_type(&mut self, texture_type: Option<TextureType>) {
        self.abstract_texture_mut().texture_type = texture_type;
    }

    fn set_wrap_mode(&mut self, wrap_mode: Option<WrapMode>) {
        self.abstract_texture_mut().wrap_mode = wrap_mode;
    }

    fn set_border_color(&mut self, border_color: Option<ColorPlusOpacity>) {
        self.abstract_texture_mut().border_color = border_color;
    }
}

impl AsAbstractTexture for AbstractTexture {
    fn abstract_texture(&self) -> &AbstractTexture {
        self
    }
}

impl AsAbstractTextureMut for AbstractTexture {
    fn abstract_texture_mut(&mut self) -> &mut AbstractTexture {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_texture_traits {
    ($type:ty) => {
        $crate::impl_abstract_surface_data_traits!($type);

        impl $crate::model::appearance::AsAbstractSurfaceData for $type {
            fn abstract_surface_data(&self) -> &$crate::model::appearance::AbstractSurfaceData {
                &<$type as $crate::model::appearance::AsAbstractTexture>::abstract_texture(self)
                    .abstract_surface_data
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_texture_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_surface_data_mut_traits!($type);

        impl $crate::model::appearance::AsAbstractSurfaceDataMut for $type {
            fn abstract_surface_data_mut(
                &mut self,
            ) -> &mut $crate::model::appearance::AbstractSurfaceData {
                &mut <$type as $crate::model::appearance::AsAbstractTextureMut>::abstract_texture_mut(self).abstract_surface_data
            }
        }
    };
}

impl_abstract_texture_traits!(AbstractTexture);
impl_abstract_texture_mut_traits!(AbstractTexture);

impl IterFeatures for AbstractTexture {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(std::iter::empty())
    }
}

impl ForEachFeatureMut for AbstractTexture {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, _f: &mut F) {}
}

impl ComputeEnvelope for AbstractTexture {
    fn compute_envelope(&self) -> Option<Envelope> {
        None
    }
}

impl ApplyTransform for AbstractTexture {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_surface_data.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_surface_data.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_surface_data.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_surface_data.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_surface_data.apply_scale(scale);
    }
}
