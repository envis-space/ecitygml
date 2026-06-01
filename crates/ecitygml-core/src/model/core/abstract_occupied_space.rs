use crate::model::common::{FeatureRef, FeatureRefMut, LevelOfDetail};
use crate::model::core::{
    AbstractPhysicalSpace, AsAbstractSpace, AsAbstractSpaceMut, ImplicitGeometry,
};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractOccupiedSpace {
    pub(crate) abstract_physical_space: AbstractPhysicalSpace,
    pub(crate) lod1_implicit_representation: Option<ImplicitGeometry>,
    pub(crate) lod2_implicit_representation: Option<ImplicitGeometry>,
    pub(crate) lod3_implicit_representation: Option<ImplicitGeometry>,
}

impl AbstractOccupiedSpace {
    pub fn new(abstract_physical_space: AbstractPhysicalSpace) -> Self {
        Self {
            abstract_physical_space,
            lod1_implicit_representation: None,
            lod2_implicit_representation: None,
            lod3_implicit_representation: None,
        }
    }

    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureRef<'a>> + 'a {
        self.abstract_physical_space.iter_features()
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureRefMut<'_>)>(&mut self, f: &mut F) {
        self.abstract_physical_space.for_each_feature_mut(f);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = vec![
            self.abstract_physical_space.compute_envelope(),
            self.lod1_implicit_representation()
                .and_then(|x| x.compute_envelope()),
            self.lod2_implicit_representation()
                .and_then(|x| x.compute_envelope()),
            self.lod3_implicit_representation()
                .and_then(|x| x.compute_envelope()),
        ]
        .into_iter()
        .flatten()
        .collect();

        Envelope::from_envelopes(&envelopes)
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_physical_space.apply_transform(m);

        if let Some(g) = &mut self
            .abstract_occupied_space_mut()
            .lod1_implicit_representation
        {
            g.apply_transform(m);
        }
        if let Some(g) = &mut self
            .abstract_occupied_space_mut()
            .lod2_implicit_representation
        {
            g.apply_transform(m);
        }
        if let Some(g) = &mut self
            .abstract_occupied_space_mut()
            .lod3_implicit_representation
        {
            g.apply_transform(m);
        }
    }
}

pub trait AsAbstractOccupiedSpace: AsAbstractSpace {
    fn abstract_occupied_space(&self) -> &AbstractOccupiedSpace;

    fn lod1_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        self.abstract_occupied_space()
            .lod1_implicit_representation
            .as_ref()
    }

    fn lod2_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        self.abstract_occupied_space()
            .lod2_implicit_representation
            .as_ref()
    }

    fn lod3_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        self.abstract_occupied_space()
            .lod3_implicit_representation
            .as_ref()
    }

    fn implicit_representations_by_lod(&self) -> HashMap<LevelOfDetail, &ImplicitGeometry> {
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

    fn set_lod1_implicit_representation(&mut self, value: Option<ImplicitGeometry>) {
        self.abstract_occupied_space_mut()
            .lod1_implicit_representation = value;
    }

    fn set_lod2_implicit_representation(&mut self, value: Option<ImplicitGeometry>) {
        self.abstract_occupied_space_mut()
            .lod2_implicit_representation = value;
    }

    fn set_lod3_implicit_representation(&mut self, value: Option<ImplicitGeometry>) {
        self.abstract_occupied_space_mut()
            .lod3_implicit_representation = value;
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
                use $crate::model::core::AsAbstractOccupiedSpace;
                &self.abstract_occupied_space().abstract_physical_space
            }
        }

        impl $crate::model::core::AsAbstractPhysicalSpaceMut for $type {
            fn abstract_physical_space_mut(
                &mut self,
            ) -> &mut $crate::model::core::AbstractPhysicalSpace {
                use $crate::model::core::AsAbstractOccupiedSpaceMut;
                &mut self.abstract_occupied_space_mut().abstract_physical_space
            }
        }
    };
}

impl_abstract_occupied_space_traits!(AbstractOccupiedSpace);
