use crate::model::common::{ForEachFeatureMut, IterFeatures, LevelOfDetail};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::core::{
    AbstractPhysicalSpace, AsAbstractSpace, AsAbstractSpaceMut, ImplicitGeometryProperty,
};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractOccupiedSpace {
    pub(crate) abstract_physical_space: AbstractPhysicalSpace,
    lod1_implicit_representation: Option<ImplicitGeometryProperty>,
    lod2_implicit_representation: Option<ImplicitGeometryProperty>,
    lod3_implicit_representation: Option<ImplicitGeometryProperty>,
}

impl AbstractOccupiedSpace {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_physical_space(AbstractPhysicalSpace::new(id))
    }

    pub fn from_abstract_physical_space(abstract_physical_space: AbstractPhysicalSpace) -> Self {
        Self {
            abstract_physical_space,
            lod1_implicit_representation: None,
            lod2_implicit_representation: None,
            lod3_implicit_representation: None,
        }
    }
}

pub trait AsAbstractOccupiedSpace: AsAbstractSpace {
    fn abstract_occupied_space(&self) -> &AbstractOccupiedSpace;

    fn lod1_implicit_representation(&self) -> Option<&ImplicitGeometryProperty> {
        self.abstract_occupied_space()
            .lod1_implicit_representation
            .as_ref()
    }

    fn lod2_implicit_representation(&self) -> Option<&ImplicitGeometryProperty> {
        self.abstract_occupied_space()
            .lod2_implicit_representation
            .as_ref()
    }

    fn lod3_implicit_representation(&self) -> Option<&ImplicitGeometryProperty> {
        self.abstract_occupied_space()
            .lod3_implicit_representation
            .as_ref()
    }

    fn implicit_representations_by_lod(&self) -> HashMap<LevelOfDetail, &ImplicitGeometryProperty> {
        let mut map = HashMap::new();
        if let Some(x) = self.lod1_implicit_representation() {
            map.insert(LevelOfDetail::One, x);
        }
        if let Some(x) = self.lod2_implicit_representation() {
            map.insert(LevelOfDetail::Two, x);
        }
        if let Some(x) = self.lod3_implicit_representation() {
            map.insert(LevelOfDetail::Three, x);
        }
        map
    }
}

pub trait AsAbstractOccupiedSpaceMut: AsAbstractSpaceMut + AsAbstractOccupiedSpace {
    fn abstract_occupied_space_mut(&mut self) -> &mut AbstractOccupiedSpace;

    fn set_lod1_implicit_representation(&mut self, value: Option<ImplicitGeometryProperty>) {
        self.abstract_occupied_space_mut()
            .lod1_implicit_representation = value;
    }

    fn set_lod2_implicit_representation(&mut self, value: Option<ImplicitGeometryProperty>) {
        self.abstract_occupied_space_mut()
            .lod2_implicit_representation = value;
    }

    fn set_lod3_implicit_representation(&mut self, value: Option<ImplicitGeometryProperty>) {
        self.abstract_occupied_space_mut()
            .lod3_implicit_representation = value;
    }

    fn lod1_implicit_representation_mut(&mut self) -> Option<&mut ImplicitGeometryProperty> {
        self.abstract_occupied_space_mut()
            .lod1_implicit_representation
            .as_mut()
    }

    fn lod2_implicit_representation_mut(&mut self) -> Option<&mut ImplicitGeometryProperty> {
        self.abstract_occupied_space_mut()
            .lod2_implicit_representation
            .as_mut()
    }

    fn lod3_implicit_representation_mut(&mut self) -> Option<&mut ImplicitGeometryProperty> {
        self.abstract_occupied_space_mut()
            .lod3_implicit_representation
            .as_mut()
    }
}

impl AsAbstractOccupiedSpace for AbstractOccupiedSpace {
    fn abstract_occupied_space(&self) -> &AbstractOccupiedSpace {
        self
    }
}

impl AsAbstractOccupiedSpaceMut for AbstractOccupiedSpace {
    fn abstract_occupied_space_mut(&mut self) -> &mut AbstractOccupiedSpace {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_occupied_space_traits {
    ($type:ty) => {
        $crate::impl_abstract_physical_space_traits!($type);

        impl $crate::model::core::AsAbstractPhysicalSpace for $type {
            fn abstract_physical_space(&self) -> &$crate::model::core::AbstractPhysicalSpace {
                &<$type as $crate::model::core::AsAbstractOccupiedSpace>::abstract_occupied_space(
                    self,
                )
                .abstract_physical_space
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_occupied_space_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_physical_space_mut_traits!($type);

        impl $crate::model::core::AsAbstractPhysicalSpaceMut for $type {
            fn abstract_physical_space_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractPhysicalSpace {
                &mut <$type as $crate::model::core::AsAbstractOccupiedSpaceMut>::abstract_occupied_space_mut(self).abstract_physical_space
            }
        }
    };
}

impl_abstract_occupied_space_traits!(AbstractOccupiedSpace);
impl_abstract_occupied_space_mut_traits!(AbstractOccupiedSpace);

impl IterFeatures for AbstractOccupiedSpace {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        self.abstract_physical_space.iter_features()
    }
}

impl ForEachFeatureMut for AbstractOccupiedSpace {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_physical_space.for_each_feature_mut(f);
    }
}

impl ComputeEnvelope for AbstractOccupiedSpace {
    fn compute_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = vec![
            self.abstract_physical_space.compute_envelope(),
            self.lod1_implicit_representation()
                .and_then(|x| x.object())
                .and_then(|x| x.compute_envelope()),
            self.lod2_implicit_representation()
                .and_then(|x| x.object())
                .and_then(|x| x.compute_envelope()),
            self.lod3_implicit_representation()
                .and_then(|x| x.object())
                .and_then(|x| x.compute_envelope()),
        ]
        .into_iter()
        .flatten()
        .collect();

        Envelope::from_envelopes(&envelopes)
    }
}

impl ApplyTransform for AbstractOccupiedSpace {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_physical_space.apply_transform(m);

        if let Some(g) = &mut self
            .lod1_implicit_representation
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            g.apply_transform(m);
        }
        if let Some(g) = &mut self
            .lod2_implicit_representation
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            g.apply_transform(m);
        }
        if let Some(g) = &mut self
            .lod3_implicit_representation
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            g.apply_transform(m);
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_physical_space.apply_isometry(isometry);

        if let Some(g) = &mut self
            .lod1_implicit_representation
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            g.apply_isometry(isometry);
        }
        if let Some(g) = &mut self
            .lod2_implicit_representation
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            g.apply_isometry(isometry);
        }
        if let Some(g) = &mut self
            .lod3_implicit_representation
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            g.apply_isometry(isometry);
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_physical_space.apply_translation(vector);

        if let Some(g) = &mut self
            .lod1_implicit_representation
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            g.apply_translation(vector);
        }
        if let Some(g) = &mut self
            .lod2_implicit_representation
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            g.apply_translation(vector);
        }
        if let Some(g) = &mut self
            .lod3_implicit_representation
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            g.apply_translation(vector);
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_physical_space.apply_rotation(rotation);

        if let Some(g) = &mut self
            .lod1_implicit_representation
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            g.apply_rotation(rotation);
        }
        if let Some(g) = &mut self
            .lod2_implicit_representation
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            g.apply_rotation(rotation);
        }
        if let Some(g) = &mut self
            .lod3_implicit_representation
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            g.apply_rotation(rotation);
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_physical_space.apply_scale(scale);

        if let Some(g) = &mut self
            .lod1_implicit_representation
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            g.apply_scale(scale);
        }
        if let Some(g) = &mut self
            .lod2_implicit_representation
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            g.apply_scale(scale);
        }
        if let Some(g) = &mut self
            .lod3_implicit_representation
            .as_mut()
            .and_then(|p| p.object_mut())
        {
            g.apply_scale(scale);
        }
    }
}
