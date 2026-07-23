use crate::model::core::refs::AbstractFeatureKindRefMut;

pub trait ForEachFeatureMut {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F);
}
