use crate::model::appearance::AbstractSurfaceDataProperty;
use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{AbstractAppearance, AsAbstractAppearance, AsAbstractAppearanceMut};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct Appearance {
    pub(crate) abstract_appearance: AbstractAppearance,
    theme: Option<String>,
    surface_data: Vec<AbstractSurfaceDataProperty>,
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

    pub fn set_theme(&mut self, theme: String) {
        self.theme = Some(theme);
    }

    pub fn set_theme_opt(&mut self, theme: Option<String>) {
        self.theme = theme;
    }

    pub fn clear_theme(&mut self) {
        self.theme = None;
    }

    pub fn surface_data(&self) -> &[AbstractSurfaceDataProperty] {
        &self.surface_data
    }

    pub fn surface_data_mut(&mut self) -> &mut [AbstractSurfaceDataProperty] {
        &mut self.surface_data
    }

    pub fn set_surface_data(&mut self, surface_data: Vec<AbstractSurfaceDataProperty>) {
        self.surface_data = surface_data;
    }

    pub fn push_surface_data(&mut self, surface_data: AbstractSurfaceDataProperty) {
        self.surface_data.push(surface_data);
    }

    pub fn extend_surface_data(
        &mut self,
        surface_data: impl IntoIterator<Item = AbstractSurfaceDataProperty>,
    ) {
        self.surface_data.extend(surface_data);
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

impl IterFeatures for Appearance {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into())
                .chain(self.abstract_appearance.iter_features())
                .chain(
                    self.surface_data
                        .iter()
                        .filter_map(|x| x.object())
                        .flat_map(|x| x.iter_features()),
                ),
        )
    }
}

impl ForEachFeatureMut for Appearance {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_appearance.for_each_feature_mut(f);

        for prop in &mut self.surface_data {
            if let Some(x) = prop.object_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }
}

impl ComputeEnvelope for Appearance {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_appearance.compute_envelope()
    }
}

impl ApplyTransform for Appearance {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_appearance.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_appearance.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_appearance.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_appearance.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_appearance.apply_scale(scale);
    }
}
