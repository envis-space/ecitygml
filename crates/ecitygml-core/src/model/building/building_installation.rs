use crate::model::common::{FeatureRef, FeatureRefMut};
use crate::model::construction::{
    AbstractInstallation, AsAbstractInstallation, AsAbstractInstallationMut,
};
use crate::model::core::AsAbstractFeatureMut;
use egml::model::basic::Code;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingInstallation {
    pub abstract_installation: AbstractInstallation,
    pub(crate) class: Option<Code>,
    pub(crate) functions: Vec<Code>,
    pub(crate) usages: Vec<Code>,
}

impl BuildingInstallation {
    pub fn new(abstract_installation: AbstractInstallation) -> Self {
        Self {
            abstract_installation,
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        std::iter::once(self.into()).chain(self.abstract_installation.iter_features())
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_installation.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_installation.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_installation.apply_transform(m);
    }

    pub fn class(&self) -> &Option<Code> {
        &self.class
    }

    pub fn set_class(&mut self, class: Option<Code>) {
        self.class = class;
    }

    pub fn functions(&self) -> &Vec<Code> {
        &self.functions
    }

    pub fn set_functions(&mut self, functions: Vec<Code>) {
        self.functions = functions;
    }

    pub fn usages(&self) -> &Vec<Code> {
        &self.usages
    }

    pub fn set_usages(&mut self, usages: Vec<Code>) {
        self.usages = usages;
    }
}

impl AsAbstractInstallation for BuildingInstallation {
    fn abstract_installation(&self) -> &AbstractInstallation {
        &self.abstract_installation
    }
}

impl AsAbstractInstallationMut for BuildingInstallation {
    fn abstract_installation_mut(&mut self) -> &mut AbstractInstallation {
        &mut self.abstract_installation
    }
}

crate::impl_abstract_installation_traits!(BuildingInstallation);

impl<'a> From<&'a BuildingInstallation> for FeatureRef<'a> {
    fn from(item: &'a BuildingInstallation) -> Self {
        FeatureRef::BuildingInstallation(item)
    }
}

impl<'a> From<&'a mut BuildingInstallation> for FeatureRefMut<'a> {
    fn from(item: &'a mut BuildingInstallation) -> Self {
        FeatureRefMut::BuildingInstallation(item)
    }
}
