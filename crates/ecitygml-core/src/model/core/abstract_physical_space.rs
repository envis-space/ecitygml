use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractPointCloudProperty, AbstractSpace, AsAbstractSpace, AsAbstractSpaceMut,
};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use egml::model::geometry::aggregates::MultiCurveProperty;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractPhysicalSpace {
    pub(crate) abstract_space: AbstractSpace,
    lod1_terrain_intersection_curve: Option<MultiCurveProperty>,
    lod2_terrain_intersection_curve: Option<MultiCurveProperty>,
    lod3_terrain_intersection_curve: Option<MultiCurveProperty>,
    point_cloud: Option<AbstractPointCloudProperty>,
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

    fn point_cloud(&self) -> Option<&AbstractPointCloudProperty> {
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

    fn set_point_cloud(&mut self, value: Option<AbstractPointCloudProperty>) {
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

    fn point_cloud_mut(&mut self) -> Option<&mut AbstractPointCloudProperty> {
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
                &<$type as $crate::model::core::AsAbstractPhysicalSpace>::abstract_physical_space(
                    self,
                )
                .abstract_space
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
                &mut <$type as $crate::model::core::AsAbstractPhysicalSpaceMut>::abstract_physical_space_mut(self).abstract_space
            }
        }
    };
}

impl_abstract_physical_space_traits!(AbstractPhysicalSpace);
impl_abstract_physical_space_mut_traits!(AbstractPhysicalSpace);

impl IterFeatures for AbstractPhysicalSpace {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            self.abstract_space.iter_features(), /* TODO .chain(
                                                     self.point_cloud
                                                         .iter()
                                                         .filter_map(|x| x.object.as_ref())
                                                         .flat_map(|x| x.iter_features()),
                                                 )*/
        )
    }
}

impl ForEachFeatureMut for AbstractPhysicalSpace {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_space.for_each_feature_mut(f);
        if let Some(prop) = &mut self.point_cloud
            && let Some(x) = prop.object_mut()
        {
            x.for_each_feature_mut(f);
        }
    }
}

impl ComputeEnvelope for AbstractPhysicalSpace {
    fn compute_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = vec![
            self.abstract_space.compute_envelope(),
            self.lod1_terrain_intersection_curve()
                .and_then(|x| x.object())
                .and_then(|x| x.compute_envelope()),
            self.lod2_terrain_intersection_curve()
                .and_then(|x| x.object())
                .and_then(|x| x.compute_envelope()),
            self.lod3_terrain_intersection_curve()
                .and_then(|x| x.object())
                .and_then(|x| x.compute_envelope()),
            self.point_cloud()
                .and_then(|x| x.object())
                .and_then(|x| x.compute_envelope()),
        ]
        .into_iter()
        .flatten()
        .collect();

        Envelope::from_envelopes(&envelopes)
    }
}

impl ApplyTransform for AbstractPhysicalSpace {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_space.apply_transform(m);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_space.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_space.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_space.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_space.apply_scale(scale);
    }
}
