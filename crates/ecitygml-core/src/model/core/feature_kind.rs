use crate::model::core::feature_with_lifespan_kind::FeatureWithLifespanKind;
use crate::model::core::{AbstractFeature, AsAbstractFeature, AsAbstractFeatureMut};

#[derive(Debug, Clone, PartialEq)]
pub enum FeatureKind {
    FeatureWithLifespanKind(FeatureWithLifespanKind),
}

impl AsAbstractFeature for FeatureKind {
    fn abstract_feature(&self) -> &AbstractFeature {
        match self {
            FeatureKind::FeatureWithLifespanKind(x) => x.abstract_feature(),
        }
    }
}

impl AsAbstractFeatureMut for FeatureKind {
    fn abstract_feature_mut(&mut self) -> &mut AbstractFeature {
        match self {
            FeatureKind::FeatureWithLifespanKind(x) => x.abstract_feature_mut(),
        }
    }
}

// impl_abstract_feature_traits!(FeatureKind);
