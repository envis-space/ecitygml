use crate::model::common::LevelOfDetail;
use crate::model::core::{
    AbstractCityObject, AsAbstractCityObjectMut, AsAbstractFeature, RelativeToTerrain,
    RelativeToWater, SpaceType,
};
use crate::model::core::{AsAbstractCityObject, AsAbstractFeatureMut};
use egml::model::geometry::Envelope;
use egml::model::geometry::aggregates::{MultiCurve, MultiSurface};
use egml::model::geometry::primitives::Solid;
use nalgebra::Isometry3;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractSpace {
    pub(crate) abstract_city_object: AbstractCityObject,

    pub(crate) space_type: Option<SpaceType>,
    pub(crate) relative_to_terrain: Option<RelativeToTerrain>,
    pub(crate) relative_to_water: Option<RelativeToWater>,

    pub(crate) lod1_solid: Option<Solid>,
    pub(crate) lod2_solid: Option<Solid>,
    pub(crate) lod3_solid: Option<Solid>,

    pub(crate) lod0_multi_surface: Option<MultiSurface>,
    pub(crate) lod2_multi_surface: Option<MultiSurface>,
    pub(crate) lod3_multi_surface: Option<MultiSurface>,

    pub(crate) lod0_multi_curve: Option<MultiCurve>,
    pub(crate) lod2_multi_curve: Option<MultiCurve>,
    pub(crate) lod3_multi_curve: Option<MultiCurve>,
}

impl AbstractSpace {
    pub fn new(abstract_city_object: AbstractCityObject) -> Self {
        Self {
            abstract_city_object,
            space_type: None,
            relative_to_terrain: None,
            relative_to_water: None,
            lod1_solid: None,
            lod2_solid: None,
            lod3_solid: None,
            lod0_multi_surface: None,
            lod2_multi_surface: None,
            lod3_multi_surface: None,
            lod0_multi_curve: None,
            lod2_multi_curve: None,
            lod3_multi_curve: None,
        }
    }
}

pub trait AsAbstractSpace: AsAbstractCityObject {
    fn abstract_space(&self) -> &AbstractSpace;

    fn space_type(&self) -> Option<&SpaceType> {
        self.abstract_space().space_type.as_ref()
    }

    fn relative_to_terrain(&self) -> Option<&RelativeToTerrain> {
        self.abstract_space().relative_to_terrain.as_ref()
    }

    fn relative_to_water(&self) -> Option<&RelativeToWater> {
        self.abstract_space().relative_to_water.as_ref()
    }

    fn lod1_solid(&self) -> Option<&Solid> {
        self.abstract_space().lod1_solid.as_ref()
    }

    fn lod2_solid(&self) -> Option<&Solid> {
        self.abstract_space().lod2_solid.as_ref()
    }

    fn lod3_solid(&self) -> Option<&Solid> {
        self.abstract_space().lod3_solid.as_ref()
    }

    fn solids_by_lod(&self) -> HashMap<LevelOfDetail, &Solid> {
        let mut map = HashMap::new();
        if let Some(x) = self.lod1_solid() {
            map.insert(LevelOfDetail::One, x);
        }
        if let Some(x) = self.lod2_solid() {
            map.insert(LevelOfDetail::Two, x);
        }
        if let Some(x) = self.lod3_solid() {
            map.insert(LevelOfDetail::Three, x);
        }
        map
    }

    fn lod0_multi_surface(&self) -> Option<&MultiSurface> {
        self.abstract_space().lod0_multi_surface.as_ref()
    }

    fn lod2_multi_surface(&self) -> Option<&MultiSurface> {
        self.abstract_space().lod2_multi_surface.as_ref()
    }

    fn lod3_multi_surface(&self) -> Option<&MultiSurface> {
        self.abstract_space().lod3_multi_surface.as_ref()
    }

    fn multi_surfaces_by_lod(&self) -> HashMap<LevelOfDetail, &MultiSurface> {
        let mut map = HashMap::new();
        if let Some(x) = self.lod0_multi_surface() {
            map.insert(LevelOfDetail::Zero, x);
        }
        if let Some(x) = self.lod2_multi_surface() {
            map.insert(LevelOfDetail::Two, x);
        }
        if let Some(x) = self.lod3_multi_surface() {
            map.insert(LevelOfDetail::Three, x);
        }
        map
    }

    fn lod0_multi_curve(&self) -> Option<&MultiCurve> {
        self.abstract_space().lod0_multi_curve.as_ref()
    }

    fn lod2_multi_curve(&self) -> Option<&MultiCurve> {
        self.abstract_space().lod2_multi_curve.as_ref()
    }

    fn lod3_multi_curve(&self) -> Option<&MultiCurve> {
        self.abstract_space().lod3_multi_curve.as_ref()
    }

    fn multi_curves_by_lod(&self) -> HashMap<LevelOfDetail, &MultiCurve> {
        let mut map = HashMap::new();
        if let Some(x) = self.lod0_multi_curve() {
            map.insert(LevelOfDetail::Zero, x);
        }
        if let Some(x) = self.lod2_multi_curve() {
            map.insert(LevelOfDetail::Two, x);
        }
        if let Some(x) = self.lod3_multi_curve() {
            map.insert(LevelOfDetail::Three, x);
        }
        map
    }

    fn compute_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = vec![
            self.lod1_solid().map(|x| x.compute_envelope()),
            self.lod2_solid().map(|x| x.compute_envelope()),
            self.lod3_solid().map(|x| x.compute_envelope()),
            self.lod0_multi_surface().map(|x| x.compute_envelope()),
            self.lod2_multi_surface().map(|x| x.compute_envelope()),
            self.lod3_multi_surface().map(|x| x.compute_envelope()),
            self.lod0_multi_curve().map(|x| x.compute_envelope()),
            self.lod2_multi_curve().map(|x| x.compute_envelope()),
            self.lod3_multi_curve().map(|x| x.compute_envelope()),
        ]
        .into_iter()
        .flatten()
        .collect();

        Envelope::from_envelopes(&envelopes)
    }
}

pub trait AsAbstractSpaceMut: AsAbstractCityObjectMut + AsAbstractSpace {
    fn abstract_space_mut(&mut self) -> &mut AbstractSpace;

    fn set_space_type(&mut self, value: Option<SpaceType>) {
        self.abstract_space_mut().space_type = value;
    }

    fn set_relative_to_terrain(&mut self, value: Option<RelativeToTerrain>) {
        self.abstract_space_mut().relative_to_terrain = value;
    }

    fn set_relative_to_water(&mut self, value: Option<RelativeToWater>) {
        self.abstract_space_mut().relative_to_water = value;
    }

    fn set_lod1_solid(&mut self, value: Option<Solid>) {
        self.abstract_space_mut().lod1_solid = value;
    }

    fn set_lod2_solid(&mut self, value: Option<Solid>) {
        self.abstract_space_mut().lod2_solid = value;
    }

    fn set_lod3_solid(&mut self, value: Option<Solid>) {
        self.abstract_space_mut().lod3_solid = value;
    }

    fn set_lod0_multi_surface(&mut self, value: Option<MultiSurface>) {
        self.abstract_space_mut().lod0_multi_surface = value;
    }

    fn set_lod2_multi_surface(&mut self, value: Option<MultiSurface>) {
        self.abstract_space_mut().lod2_multi_surface = value;
    }

    fn set_lod3_multi_surface(&mut self, value: Option<MultiSurface>) {
        self.abstract_space_mut().lod3_multi_surface = value;
    }

    fn set_lod0_multi_curve(&mut self, value: Option<MultiCurve>) {
        self.abstract_space_mut().lod0_multi_curve = value;
    }

    fn set_lod2_multi_curve(&mut self, value: Option<MultiCurve>) {
        self.abstract_space_mut().lod2_multi_curve = value;
    }

    fn set_lod3_multi_curve(&mut self, value: Option<MultiCurve>) {
        self.abstract_space_mut().lod3_multi_curve = value;
    }

    fn refresh_bounded_by(&mut self) {
        let envelope = self.compute_envelope();
        self.set_bounded_by(envelope);
    }

    fn apply_transform(&mut self, m: &Isometry3<f64>) {
        if let Some(g) = &mut self.abstract_space_mut().lod1_solid {
            g.apply_transform(m);
        }
        if let Some(g) = &mut self.abstract_space_mut().lod2_solid {
            g.apply_transform(m);
        }
        if let Some(g) = &mut self.abstract_space_mut().lod3_solid {
            g.apply_transform(m);
        }

        if let Some(g) = &mut self.abstract_space_mut().lod0_multi_surface {
            g.apply_transform(m);
        }
        if let Some(g) = &mut self.abstract_space_mut().lod2_multi_surface {
            g.apply_transform(m);
        }
        if let Some(g) = &mut self.abstract_space_mut().lod3_multi_surface {
            g.apply_transform(m);
        }

        if let Some(g) = &mut self.abstract_space_mut().lod0_multi_curve {
            g.apply_transform(m);
        }
        if let Some(g) = &mut self.abstract_space_mut().lod2_multi_curve {
            g.apply_transform(m);
        }
        if let Some(g) = &mut self.abstract_space_mut().lod3_multi_curve {
            g.apply_transform(m);
        }
    }
}

impl AsAbstractSpace for AbstractSpace {
    fn abstract_space(&self) -> &AbstractSpace {
        self
    }
}

impl AsAbstractSpaceMut for AbstractSpace {
    fn abstract_space_mut(&mut self) -> &mut AbstractSpace {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_space_traits {
    ($type:ty) => {
        $crate::impl_abstract_city_object_traits!($type);

        impl $crate::model::core::AsAbstractCityObject for $type {
            fn abstract_city_object(&self) -> &$crate::model::core::AbstractCityObject {
                use $crate::model::core::AsAbstractSpace;
                &self.abstract_space().abstract_city_object
            }
        }

        impl $crate::model::core::AsAbstractCityObjectMut for $type {
            fn abstract_city_object_mut(&mut self) -> &mut $crate::model::core::AbstractCityObject {
                use $crate::model::core::AsAbstractSpaceMut;
                &mut self.abstract_space_mut().abstract_city_object
            }
        }
    };
}

impl_abstract_space_traits!(AbstractSpace);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::core::{AbstractFeature, AbstractFeatureWithLifespan};
    use egml::model::base::Id;

    #[test]
    fn trait_implementation_macro_test() {
        let abstract_feature = AbstractFeature::new(Id::generate_uuid_v4());
        let abstract_feature_with_lifespan = AbstractFeatureWithLifespan::new(abstract_feature);
        let abstract_city_object = AbstractCityObject::new(abstract_feature_with_lifespan);
        let space = AbstractSpace::new(abstract_city_object);

        let a = space.bounded_by();
    }
}
