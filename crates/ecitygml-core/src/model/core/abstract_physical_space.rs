use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use crate::model::core::{AbstractSpace, AsAbstractSpace, AsAbstractSpaceMut, PointCloudProperty};
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use egml::model::geometry::aggregates::MultiCurveProperty;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractPhysicalSpace {
    pub(crate) abstract_space: AbstractSpace,
    lod1_terrain_intersection_curve: Option<MultiCurveProperty>,
    lod2_terrain_intersection_curve: Option<MultiCurveProperty>,
    lod3_terrain_intersection_curve: Option<MultiCurveProperty>,
    point_cloud: Option<PointCloudProperty>,
}

impl AbstractPhysicalSpace {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_space(AbstractSpace::new(id))
    }

    pub fn from_abstract_space(abstract_space: AbstractSpace) -> Self {
        Self {
            abstract_space,
            lod1_terrain_intersection_curve: None,
            lod2_terrain_intersection_curve: None,
            lod3_terrain_intersection_curve: None,
            point_cloud: None,
        }
    }
}

impl AbstractPhysicalSpace {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        self.abstract_space.iter_features()
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_space.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = vec![
            self.abstract_space.compute_envelope(),
            self.lod1_terrain_intersection_curve()
                .and_then(|x| x.object.as_ref())
                .and_then(|x| x.compute_envelope()),
            self.lod2_terrain_intersection_curve()
                .and_then(|x| x.object.as_ref())
                .and_then(|x| x.compute_envelope()),
            self.lod3_terrain_intersection_curve()
                .and_then(|x| x.object.as_ref())
                .and_then(|x| x.compute_envelope()),
        ]
        .into_iter()
        .flatten()
        .collect();

        Envelope::from_envelopes(&envelopes)
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_space.apply_transform(m);
    }
}

pub trait AsAbstractPhysicalSpace: AsAbstractSpace {
    fn abstract_physical_space(&self) -> &AbstractPhysicalSpace;

    fn lod1_terrain_intersection_curve(&self) -> Option<&MultiCurveProperty> {
        self.abstract_physical_space()
            .lod1_terrain_intersection_curve
            .as_ref()
    }

    fn lod2_terrain_intersection_curve(&self) -> Option<&MultiCurveProperty> {
        self.abstract_physical_space()
            .lod2_terrain_intersection_curve
            .as_ref()
    }

    fn lod3_terrain_intersection_curve(&self) -> Option<&MultiCurveProperty> {
        self.abstract_physical_space()
            .lod3_terrain_intersection_curve
            .as_ref()
    }

    fn point_cloud(&self) -> Option<&PointCloudProperty> {
        self.abstract_physical_space().point_cloud.as_ref()
    }
}

pub trait AsAbstractPhysicalSpaceMut: AsAbstractSpaceMut + AsAbstractPhysicalSpace {
    fn abstract_physical_space_mut(&mut self) -> &mut AbstractPhysicalSpace;

    fn set_lod1_terrain_intersection_curve(
        &mut self,
        lod1_terrain_intersection_curve: Option<MultiCurveProperty>,
    ) {
        self.abstract_physical_space_mut()
            .lod1_terrain_intersection_curve = lod1_terrain_intersection_curve;
    }

    fn set_lod2_terrain_intersection_curve(
        &mut self,
        lod2_terrain_intersection_curve: Option<MultiCurveProperty>,
    ) {
        self.abstract_physical_space_mut()
            .lod2_terrain_intersection_curve = lod2_terrain_intersection_curve;
    }

    fn set_lod3_terrain_intersection_curve(
        &mut self,
        lod3_terrain_intersection_curve: Option<MultiCurveProperty>,
    ) {
        self.abstract_physical_space_mut()
            .lod3_terrain_intersection_curve = lod3_terrain_intersection_curve;
    }

    fn set_point_cloud(&mut self, value: Option<PointCloudProperty>) {
        self.abstract_physical_space_mut().point_cloud = value;
    }

    fn lod1_terrain_intersection_curve_mut(&mut self) -> Option<&mut MultiCurveProperty> {
        self.abstract_physical_space_mut()
            .lod1_terrain_intersection_curve
            .as_mut()
    }

    fn lod2_terrain_intersection_curve_mut(&mut self) -> Option<&mut MultiCurveProperty> {
        self.abstract_physical_space_mut()
            .lod2_terrain_intersection_curve
            .as_mut()
    }

    fn lod3_terrain_intersection_curve_mut(&mut self) -> Option<&mut MultiCurveProperty> {
        self.abstract_physical_space_mut()
            .lod3_terrain_intersection_curve
            .as_mut()
    }

    fn point_cloud_mut(&mut self) -> Option<&mut PointCloudProperty> {
        self.abstract_physical_space_mut().point_cloud.as_mut()
    }
}

impl AsAbstractPhysicalSpace for AbstractPhysicalSpace {
    fn abstract_physical_space(&self) -> &AbstractPhysicalSpace {
        self
    }
}

impl AsAbstractPhysicalSpaceMut for AbstractPhysicalSpace {
    fn abstract_physical_space_mut(&mut self) -> &mut AbstractPhysicalSpace {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_physical_space_traits {
    ($type:ty) => {
        $crate::impl_abstract_space_traits!($type);

        impl $crate::model::core::AsAbstractSpace for $type {
            fn abstract_space(&self) -> &$crate::model::core::AbstractSpace {
                use $crate::model::core::AsAbstractPhysicalSpace;
                &self.abstract_physical_space().abstract_space
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_physical_space_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_space_mut_traits!($type);

        impl $crate::model::core::AsAbstractSpaceMut for $type {
            fn abstract_space_mut(&mut self) -> &mut $crate::model::core::AbstractSpace {
                use $crate::model::core::AsAbstractPhysicalSpaceMut;
                &mut self.abstract_physical_space_mut().abstract_space
            }
        }
    };
}

impl_abstract_physical_space_traits!(AbstractPhysicalSpace);
impl_abstract_physical_space_mut_traits!(AbstractPhysicalSpace);
