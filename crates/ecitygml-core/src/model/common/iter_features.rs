use crate::model::core::refs::AbstractFeatureKindRef;

pub trait IterFeatures {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_>;
}
