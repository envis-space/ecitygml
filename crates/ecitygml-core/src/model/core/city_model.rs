use crate::model::building::Building;
use crate::model::city_furniture::CityFurniture;
use crate::model::core::{
    AbstractFeature, AsAbstractFeature, AsAbstractFeatureMut, CityObjectKind, CityObjectRef,
};
use crate::model::transportation::Road;
use crate::model::vegetation::SolitaryVegetationObject;
use crate::operations::{Visitable, Visitor};
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CityModel {
    pub abstract_feature: AbstractFeature,
    pub city_objects: Vec<CityObjectKind>,
}

impl CityModel {
    pub fn new(abstract_feature: AbstractFeature, city_objects: Vec<CityObjectKind>) -> Self {
        Self {
            abstract_feature,
            city_objects,
        }
    }

    pub fn from_city_models(city_models: Vec<Self>) -> Self {
        let abstract_feature = AbstractFeature::new(Id::generate_uuid_v4());

        let combined_city_objects: Vec<CityObjectKind> = city_models
            .into_iter()
            .flat_map(|x| x.city_objects)
            .collect();

        CityModel::new(abstract_feature, combined_city_objects)
    }

    pub fn is_empty(&self) -> bool {
        self.city_objects.is_empty()
    }

    pub fn city_objects(&self) -> &Vec<CityObjectKind> {
        &self.city_objects
    }

    pub fn city_objects_len(&self) -> usize {
        self.city_objects.len()
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        self.city_objects.iter().flat_map(|x| x.iter_city_object())
    }

    pub fn refresh_bounded_by_recursive(&mut self) {
        self.city_objects
            .par_iter_mut()
            .for_each(|x| x.refresh_bounded_by_recursive());

        let envelopes: Vec<&Envelope> = self
            .city_objects
            .iter()
            .filter_map(|x| x.bounded_by())
            .collect();

        self.set_bounded_by(Envelope::from_envelopes(&envelopes));
    }

    pub fn apply_transform_recursive(&mut self, m: &Isometry3<f64>) {
        self.city_objects
            .par_iter_mut()
            .for_each(|x| x.apply_transform_recursive(m));
    }

    pub fn filter_city_objects_by_envelope(&mut self, filter_envelope: &Envelope) {
        self.city_objects.retain(|obj| {
            obj.bounded_by()
                .is_some_and(|env| filter_envelope.contains_envelope_partially(env))
        });
    }

    pub fn roads(&self) -> Vec<&Road> {
        self.city_objects
            .iter()
            .filter_map(|x| match x {
                CityObjectKind::Road(x) => Some(x),
                _ => None,
            })
            .collect()
    }

    pub fn buildings(&self) -> Vec<&Building> {
        self.city_objects
            .iter()
            .filter_map(|x| match x {
                CityObjectKind::Building(x) => Some(x),
                _ => None,
            })
            .collect()
    }

    pub fn solitary_vegetation_objects(&self) -> Vec<&SolitaryVegetationObject> {
        self.city_objects
            .iter()
            .filter_map(|x| match x {
                CityObjectKind::SolitaryVegetationObject(x) => Some(x),
                _ => None,
            })
            .collect()
    }

    pub fn city_furniture_objects(&self) -> Vec<&CityFurniture> {
        self.city_objects
            .iter()
            .filter_map(|x| match x {
                CityObjectKind::CityFurniture(x) => Some(x),
                _ => None,
            })
            .collect()
    }
}

impl AsAbstractFeature for CityModel {
    fn abstract_feature(&self) -> &AbstractFeature {
        &self.abstract_feature
    }
}

impl AsAbstractFeatureMut for CityModel {
    fn abstract_feature_mut(&mut self) -> &mut AbstractFeature {
        &mut self.abstract_feature
    }
}

impl Visitable for CityModel {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_city_model(self);

        for feature in &self.city_objects {
            match feature {
                CityObjectKind::Building(x) => {
                    x.accept(visitor);
                }
                CityObjectKind::BuildingConstructiveElement(x) => {
                    x.accept(visitor);
                }
                CityObjectKind::CityFurniture(x) => {
                    x.accept(visitor);
                }
                CityObjectKind::ReliefFeature(x) => {
                    x.accept(visitor);
                }
                CityObjectKind::Road(x) => {
                    x.accept(visitor);
                }
                CityObjectKind::SolitaryVegetationObject(x) => {
                    x.accept(visitor);
                }
                CityObjectKind::TinRelief(x) => {
                    x.accept(visitor);
                }
                CityObjectKind::TrafficSpace(x) => {
                    x.accept(visitor);
                }
                CityObjectKind::AuxiliaryTrafficSpace(x) => {
                    x.accept(visitor);
                }
                CityObjectKind::TrafficArea(x) => {
                    x.accept(visitor);
                }
                CityObjectKind::AuxiliaryTrafficArea(x) => {
                    x.accept(visitor);
                }
                CityObjectKind::Section(x) => {
                    x.accept(visitor);
                }
                CityObjectKind::Intersection(x) => {
                    x.accept(visitor);
                }
                CityObjectKind::DoorSurface(x) => {
                    x.accept(visitor);
                }
                CityObjectKind::GroundSurface(x) => {
                    x.accept(visitor);
                }
                CityObjectKind::RoofSurface(x) => {
                    x.accept(visitor);
                }
                CityObjectKind::WallSurface(x) => {
                    x.accept(visitor);
                }
                CityObjectKind::WindowSurface(x) => {
                    x.accept(visitor);
                }
            }
        }
    }
}
