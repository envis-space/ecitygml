use crate::model::common::{CityObjectClass, LevelOfDetail};
use crate::model::core::{
    AbstractCityObject, AbstractOccupiedSpace, AbstractSpace, AbstractThematicSurface,
    AsAbstractCityObject, AsAbstractFeature, AsAbstractOccupiedSpace, AsAbstractSpace,
    CityObjectRef, ImplicitGeometry,
};
use crate::model::core::{AsAbstractThematicSurface, CityModel};
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use egml::model::geometry::aggregates::{MultiCurve, MultiSurface};
use egml::model::geometry::primitives::Solid;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CityModelGeometryIndex {
    pub objects: HashMap<Id, CityObjectGeometry>,
}

impl CityModelGeometryIndex {
    pub fn from_city_model(mut city_model: CityModel) -> Self {
        city_model.refresh_bounded_by_recursive();

        let city_objects: HashMap<Id, CityObjectGeometry> = city_model
            .iter_city_object()
            .map(|x| (x.id().clone(), CityObjectGeometry::from_city_object(x)))
            .collect();

        Self {
            objects: city_objects,
        }
    }

    pub fn objects_len(&self) -> usize {
        self.objects.len()
    }

    pub fn object_ids(&self) -> Vec<String> {
        self.objects.keys().map(|x| x.to_string()).collect()
    }

    pub fn get(&self, id: &Id) -> Option<&CityObjectGeometry> {
        self.objects.get(id)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CityObjectGeometry {
    pub base: AbstractCityObject,
    pub class: CityObjectClass,
    pub implicit_geometries: HashMap<LevelOfDetail, ImplicitGeometry>,
    pub multi_surfaces: HashMap<LevelOfDetail, MultiSurface>,
    pub multi_curves: HashMap<LevelOfDetail, MultiCurve>,
    pub solids: HashMap<LevelOfDetail, Solid>,
}

impl CityObjectGeometry {
    pub fn new(abstract_space: AbstractCityObject, class: CityObjectClass) -> Self {
        Self {
            base: abstract_space,
            class,
            implicit_geometries: HashMap::new(),
            multi_surfaces: HashMap::new(),
            multi_curves: HashMap::new(),
            solids: HashMap::new(),
        }
    }

    pub fn from_city_object(city_object: CityObjectRef) -> Self {
        let city_object_class = city_object.city_object_class();

        match city_object {
            CityObjectRef::AuxiliaryTrafficArea(x) => Self::from_abstract_thematic_surface(
                city_object_class,
                x.abstract_thematic_surface(),
            ),
            CityObjectRef::AuxiliaryTrafficSpace(x) => {
                Self::from_abstract_space(city_object_class, x.abstract_space())
            }
            CityObjectRef::Building(x) => {
                Self::from_abstract_space(city_object_class, x.abstract_space())
            }
            CityObjectRef::BuildingConstructiveElement(x) => {
                Self::from_abstract_occupied_space(city_object_class, x.abstract_occupied_space())
            }
            CityObjectRef::CityFurniture(x) => {
                Self::from_abstract_occupied_space(city_object_class, x.abstract_occupied_space())
            }
            CityObjectRef::DoorSurface(x) => Self::from_abstract_thematic_surface(
                city_object_class,
                x.abstract_thematic_surface(),
            ),
            CityObjectRef::GroundSurface(x) => Self::from_abstract_thematic_surface(
                city_object_class,
                x.abstract_thematic_surface(),
            ),
            CityObjectRef::Intersection(x) => {
                Self::from_abstract_space(city_object_class, x.abstract_space())
            }
            CityObjectRef::ReliefFeature(x) => {
                CityObjectGeometry::new(x.abstract_city_object().clone(), city_object_class)
            }
            CityObjectRef::Road(x) => {
                Self::from_abstract_space(city_object_class, x.abstract_space())
            }
            CityObjectRef::RoofSurface(x) => Self::from_abstract_thematic_surface(
                city_object_class,
                x.abstract_thematic_surface(),
            ),
            CityObjectRef::Section(x) => {
                Self::from_abstract_space(city_object_class, x.abstract_space())
            }
            CityObjectRef::SolitaryVegetationObject(x) => {
                Self::from_abstract_occupied_space(city_object_class, x.abstract_occupied_space())
            }
            CityObjectRef::TinRelief(x) => {
                CityObjectGeometry::new(x.abstract_city_object().clone(), city_object_class)
            }
            CityObjectRef::TrafficArea(x) => Self::from_abstract_thematic_surface(
                city_object_class,
                x.abstract_thematic_surface(),
            ),
            CityObjectRef::TrafficSpace(x) => {
                Self::from_abstract_space(city_object_class, x.abstract_space())
            }
            CityObjectRef::WallSurface(x) => Self::from_abstract_thematic_surface(
                city_object_class,
                x.abstract_thematic_surface(),
            ),
            CityObjectRef::WindowSurface(x) => Self::from_abstract_thematic_surface(
                city_object_class,
                x.abstract_thematic_surface(),
            ),
        }
    }

    pub fn from_abstract_space(class: CityObjectClass, space: &AbstractSpace) -> Self {
        let solids: HashMap<LevelOfDetail, Solid> = space
            .solids_by_lod()
            .into_iter()
            .map(|(lod, x)| (lod, x.clone()))
            .collect();

        let multi_surfaces: HashMap<LevelOfDetail, MultiSurface> = space
            .multi_surfaces_by_lod()
            .into_iter()
            .map(|(lod, x)| (lod, x.clone()))
            .collect();

        let multi_curves: HashMap<LevelOfDetail, MultiCurve> = space
            .multi_curves_by_lod()
            .into_iter()
            .map(|(lod, x)| (lod, x.clone()))
            .collect();

        Self {
            base: space.abstract_city_object().clone(),
            class,
            implicit_geometries: HashMap::new(),
            multi_surfaces,
            multi_curves,
            solids,
        }
    }

    pub fn from_abstract_occupied_space(
        class: CityObjectClass,
        occupied_space: &AbstractOccupiedSpace,
    ) -> Self {
        let mut city_object_geometry =
            CityObjectGeometry::from_abstract_space(class, &occupied_space.abstract_space);

        let implicit_representations: HashMap<LevelOfDetail, ImplicitGeometry> = occupied_space
            .implicit_representations_by_lod()
            .into_iter()
            .map(|(lod, x)| (lod, x.clone()))
            .collect();

        city_object_geometry.implicit_geometries = implicit_representations;
        city_object_geometry
    }

    pub fn from_abstract_thematic_surface(
        class: CityObjectClass,
        thematic_surface: &AbstractThematicSurface,
    ) -> Self {
        let multi_surfaces: HashMap<LevelOfDetail, MultiSurface> = thematic_surface
            .multi_surfaces_by_lod()
            .into_iter()
            .map(|(lod, x)| (lod, x.clone()))
            .collect();

        Self {
            base: thematic_surface.abstract_city_object().clone(),
            class,
            implicit_geometries: HashMap::new(),
            multi_surfaces,
            solids: HashMap::new(),
            multi_curves: HashMap::new(),
        }
    }

    pub fn id(&self) -> &Id {
        self.base.id()
    }

    pub fn envelope(&self) -> Option<&Envelope> {
        self.base.bounded_by()
    }
}
