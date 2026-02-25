use crate::model::common::LevelOfDetail;
use crate::model::core::{
    AbstractSpace, AsAbstractCityObject, AsAbstractFeature, AsAbstractSpace, AsAbstractSpaceMut,
    ImplicitGeometry,
};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractOccupiedSpace {
    pub(crate) abstract_space: AbstractSpace,
    pub lod1_implicit_representation: Option<ImplicitGeometry>,
    pub lod2_implicit_representation: Option<ImplicitGeometry>,
    pub lod3_implicit_representation: Option<ImplicitGeometry>,
}

impl AbstractOccupiedSpace {
    pub fn new(abstract_space: AbstractSpace) -> Self {
        Self {
            abstract_space,
            lod1_implicit_representation: None,
            lod2_implicit_representation: None,
            lod3_implicit_representation: None,
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
            map.insert(LevelOfDetail::Zero, x);
        }
        if let Some(x) = self.lod2_implicit_representation() {
            map.insert(LevelOfDetail::One, x);
        }
        if let Some(x) = self.lod3_implicit_representation() {
            map.insert(LevelOfDetail::Two, x);
        }
        map
    }

    fn compute_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = vec![
            self.abstract_space().compute_envelope(),
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

        let refs: Vec<&Envelope> = envelopes.iter().collect();
        Envelope::from_envelopes(&refs)
    }
}

pub trait AsAbstractOccupiedSpaceMut: AsAbstractSpaceMut + AsAbstractOccupiedSpace {
    fn abstract_occupied_space_mut(&mut self) -> &mut AbstractOccupiedSpace;

    fn refresh_bounded_by(&mut self) {
        let envelope = AsAbstractOccupiedSpace::compute_envelope(self);
        self.set_bounded_by(envelope);
    }

    fn apply_transform(&mut self, m: &Isometry3<f64>) {
        AsAbstractSpaceMut::apply_transform(self, m);

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
        $crate::impl_abstract_space_traits!($type);

        impl $crate::model::core::AsAbstractSpace for $type {
            fn abstract_space(&self) -> &$crate::model::core::AbstractSpace {
                use $crate::model::core::AsAbstractOccupiedSpace;
                &self.abstract_occupied_space().abstract_space
            }
        }

        impl $crate::model::core::AsAbstractSpaceMut for $type {
            fn abstract_space_mut(&mut self) -> &mut $crate::model::core::AbstractSpace {
                use $crate::model::core::AsAbstractOccupiedSpaceMut;
                &mut self.abstract_occupied_space_mut().abstract_space
            }
        }
    };
}

impl_abstract_occupied_space_traits!(AbstractOccupiedSpace);
