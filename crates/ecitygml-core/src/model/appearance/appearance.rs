use crate::model::appearance::SurfaceDataProperty;
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use crate::model::core::{
    AbstractAppearance, AsAbstractAppearance, AsAbstractAppearanceMut, AsAbstractFeatureMut,
};
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct Appearance {
    pub(crate) abstract_appearance: AbstractAppearance,
    theme: Option<String>,
    surface_data: Vec<SurfaceDataProperty>,
}

impl Appearance {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_appearance(AbstractAppearance::new(id))
    }

    pub fn from_abstract_appearance(abstract_appearance: AbstractAppearance) -> Self {
        Self {
            abstract_appearance,
            theme: None,
            surface_data: Vec::new(),
        }
    }

    pub fn theme(&self) -> Option<&str> {
        self.theme.as_deref()
    }

    pub fn set_theme(&mut self, theme: Option<String>) {
        self.theme = theme;
    }

    pub fn surface_data(&self) -> &[SurfaceDataProperty] {
        &self.surface_data
    }

    pub fn set_surface_data(&mut self, surface_data: Vec<SurfaceDataProperty>) {
        self.surface_data = surface_data;
    }
}

impl Appearance {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_appearance.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_appearance.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_appearance.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_appearance.apply_transform(m);
    }
}

impl AsAbstractAppearance for Appearance {
    fn abstract_appearance(&self) -> &AbstractAppearance {
        &self.abstract_appearance
    }
}

impl AsAbstractAppearanceMut for Appearance {
    fn abstract_appearance_mut(&mut self) -> &mut AbstractAppearance {
        &mut self.abstract_appearance
    }
}

crate::impl_abstract_appearance_traits!(Appearance);
crate::impl_abstract_appearance_mut_traits!(Appearance);
crate::impl_has_feature_type!(Appearance, Appearance);
