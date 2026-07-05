use crate::model::common::FeatureType;

pub trait HasFeatureType {
    fn feature_type(&self) -> FeatureType;
}

#[macro_export]
macro_rules! impl_has_feature_type {
    ($type:ty, $variant:ident) => {
        impl $crate::model::common::HasFeatureType for $type {
            fn feature_type(&self) -> $crate::model::common::FeatureType {
                $crate::model::common::FeatureType::$variant
            }
        }
    };
}
