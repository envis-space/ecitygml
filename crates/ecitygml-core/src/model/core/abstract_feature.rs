use egml::model::base::Id;
use egml::model::feature::BoundingShape;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractFeature {
    pub(crate) abstract_feature: egml::model::feature::AbstractFeature,
}

impl AbstractFeature {
    pub fn new(id: Id) -> Self {
        let abstract_gml = egml::model::base::AbstractGml::with_id(id);
        let abstract_feature = egml::model::feature::AbstractFeature::new(abstract_gml);

        Self { abstract_feature }
    }

    pub fn with_gml_abstract_feature(
        abstract_feature: egml::model::feature::AbstractFeature,
    ) -> Self {
        Self { abstract_feature }
    }

    pub fn gml_abstract_feature(&self) -> &egml::model::feature::AbstractFeature {
        &self.abstract_feature
    }

    pub fn id(&self) -> &Id {
        self.abstract_feature
            .abstract_gml
            .id
            .as_ref()
            .expect("id must be set for AbstractFeature")
    }

    pub fn name(&self) -> &Vec<String> {
        &self.abstract_feature.abstract_gml.name
    }

    pub fn set_name(&mut self, name: Vec<String>) {
        self.abstract_feature.abstract_gml.name = name;
    }

    pub fn bounded_by(&self) -> Option<&Envelope> {
        self.abstract_feature.bounded_by.as_ref()?.envelope.as_ref()
    }

    pub fn set_bounding_shape(&mut self, bounding_shape: Option<BoundingShape>) {
        self.abstract_feature.bounded_by = bounding_shape;
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        if let Some(bounding_shape) = self.abstract_feature.bounded_by.as_mut()
            && let Some(envelope) = &mut bounding_shape.envelope
        {
            envelope.apply_transform(m);
        }
    }
}

pub trait AsAbstractFeature {
    fn abstract_feature(&self) -> &AbstractFeature;

    fn id(&self) -> &Id {
        self.abstract_feature().id()
    }

    fn name(&self) -> &Vec<String> {
        self.abstract_feature().name()
    }

    fn bounded_by(&self) -> Option<&Envelope> {
        self.abstract_feature().bounded_by()
    }
}

pub trait AsAbstractFeatureMut: AsAbstractFeature {
    fn abstract_feature_mut(&mut self) -> &mut AbstractFeature;

    fn set_name(&mut self, name: Vec<String>) {
        self.abstract_feature_mut().set_name(name);
    }

    fn set_bounding_shape_from_envelope(&mut self, envelope: Option<Envelope>) {
        let bounding_shape = envelope.map(BoundingShape::new);
        self.abstract_feature_mut()
            .set_bounding_shape(bounding_shape);
    }

    fn set_bounding_shape(&mut self, bounding_shape: Option<BoundingShape>) {
        self.abstract_feature_mut()
            .set_bounding_shape(bounding_shape);
    }
}

impl AsAbstractFeature for AbstractFeature {
    fn abstract_feature(&self) -> &AbstractFeature {
        self
    }
}

impl AsAbstractFeatureMut for AbstractFeature {
    fn abstract_feature_mut(&mut self) -> &mut AbstractFeature {
        self
    }
}
