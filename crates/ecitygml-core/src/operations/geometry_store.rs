use crate::model::common::{FeatureRef, FeatureType, LevelOfDetail};
use crate::model::core::{
    AbstractCityObject, AbstractOccupiedSpace, AbstractSpace, AbstractThematicSurface,
    AsAbstractCityObject, AsAbstractFeature, AsAbstractOccupiedSpace, AsAbstractSpace,
    ImplicitGeometry,
};
use crate::model::core::{AsAbstractThematicSurface, CityModel};
use crate::model::relief::AsAbstractReliefComponent;
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use egml::model::geometry::aggregates::{MultiCurve, MultiSurface};
use egml::model::geometry::primitives::{Solid, TriangulatedSurface};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CityModelGeometryStore {
    pub objects: Vec<CityObjectGeometryEntry>,
    pub id_to_index: HashMap<Id, usize>,
}

impl CityModelGeometryStore {
    pub fn from_city_model(city_model: CityModel) -> Self {
        // city_model.refresh_bounded_by_recursive();

        let objects: Vec<CityObjectGeometryEntry> = city_model
            .iter_features()
            .map(CityObjectGeometryEntry::from_city_object)
            .collect();

        let id_to_index: HashMap<Id, usize> = objects
            .iter()
            .enumerate()
            .map(|(i, x)| (x.id().clone(), i))
            .collect();

        Self {
            objects,
            id_to_index,
        }
    }

    pub fn objects_len(&self) -> usize {
        self.objects.len()
    }

    pub fn object_ids(&self) -> Vec<String> {
        self.id_to_index.keys().map(|x| x.to_string()).collect()
    }

    pub fn get_by_id(&self, id: &Id) -> Option<&CityObjectGeometryEntry> {
        self.id_to_index.get(id).map(|&i| &self.objects[i])
    }

    pub fn get_by_index(&self, index: usize) -> Option<&CityObjectGeometryEntry> {
        self.objects.get(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &CityObjectGeometryEntry> {
        self.objects.iter()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CityObjectGeometryEntry {
    pub base: AbstractCityObject,
    pub class: FeatureType,
    pub multi_curves: HashMap<LevelOfDetail, MultiCurve>,
    pub multi_surfaces: HashMap<LevelOfDetail, MultiSurface>,
    pub solids: HashMap<LevelOfDetail, Solid>,
    pub implicit_geometries: HashMap<LevelOfDetail, ImplicitGeometry>,
    pub triangulated_surface: HashMap<LevelOfDetail, TriangulatedSurface>,
}

impl CityObjectGeometryEntry {
    pub fn new(abstract_space: AbstractCityObject, class: FeatureType) -> Self {
        Self {
            base: abstract_space,
            class,
            multi_curves: HashMap::new(),
            multi_surfaces: HashMap::new(),
            solids: HashMap::new(),
            implicit_geometries: HashMap::new(),
            triangulated_surface: HashMap::new(),
        }
    }

    pub fn from_city_object(feature_ref: FeatureRef) -> Self {
        let class = feature_ref.feature_type();
        match feature_ref {
            FeatureRef::AuxiliaryTrafficArea(x) => {
                Self::from_abstract_thematic_surface(class, x.abstract_thematic_surface())
            }
            FeatureRef::AuxiliaryTrafficSpace(x) => {
                Self::from_abstract_space(class, x.abstract_space())
            }
            FeatureRef::Building(x) => Self::from_abstract_space(class, x.abstract_space()),
            FeatureRef::BuildingConstructiveElement(x) => {
                Self::from_abstract_occupied_space(class, x.abstract_occupied_space())
            }
            FeatureRef::BuildingInstallation(x) => {
                Self::from_abstract_occupied_space(class, x.abstract_occupied_space())
            }
            FeatureRef::BuildingRoom(x) => Self::from_abstract_space(class, x.abstract_space()),
            FeatureRef::BuildingUnit(x) => Self::from_abstract_space(class, x.abstract_space()),
            FeatureRef::CeilingSurface(x) => {
                Self::from_abstract_thematic_surface(class, x.abstract_thematic_surface())
            }
            FeatureRef::CityFurniture(x) => {
                Self::from_abstract_occupied_space(class, x.abstract_occupied_space())
            }
            FeatureRef::ClosureSurface(x) => {
                Self::from_abstract_thematic_surface(class, x.abstract_thematic_surface())
            }
            FeatureRef::Door(x) => {
                Self::from_abstract_occupied_space(class, x.abstract_occupied_space())
            }
            FeatureRef::DoorSurface(x) => {
                Self::from_abstract_thematic_surface(class, x.abstract_thematic_surface())
            }
            FeatureRef::FloorSurface(x) => {
                Self::from_abstract_thematic_surface(class, x.abstract_thematic_surface())
            }
            FeatureRef::GroundSurface(x) => {
                Self::from_abstract_thematic_surface(class, x.abstract_thematic_surface())
            }
            FeatureRef::InteriorWallSurface(x) => {
                Self::from_abstract_thematic_surface(class, x.abstract_thematic_surface())
            }
            FeatureRef::Intersection(x) => Self::from_abstract_space(class, x.abstract_space()),
            FeatureRef::Marking(x) => {
                Self::from_abstract_thematic_surface(class, x.abstract_thematic_surface())
            }
            FeatureRef::OuterCeilingSurface(x) => {
                Self::from_abstract_thematic_surface(class, x.abstract_thematic_surface())
            }
            FeatureRef::OuterFloorSurface(x) => {
                Self::from_abstract_thematic_surface(class, x.abstract_thematic_surface())
            }
            FeatureRef::ReliefFeature(x) => Self::new(x.abstract_city_object().clone(), class),
            FeatureRef::Road(x) => Self::from_abstract_space(class, x.abstract_space()),
            FeatureRef::RoofSurface(x) => {
                Self::from_abstract_thematic_surface(class, x.abstract_thematic_surface())
            }
            FeatureRef::Section(x) => Self::from_abstract_space(class, x.abstract_space()),
            FeatureRef::SolitaryVegetationObject(x) => {
                Self::from_abstract_occupied_space(class, x.abstract_occupied_space())
            }
            FeatureRef::Storey(x) => Self::from_abstract_space(class, x.abstract_space()),
            FeatureRef::TinRelief(x) => {
                let mut entry = Self::new(x.abstract_city_object().clone(), class);
                if let Some(geom) = x.tin() {
                    entry.triangulated_surface.insert(x.lod(), geom.clone());
                }
                entry
            }
            FeatureRef::TrafficArea(x) => {
                Self::from_abstract_thematic_surface(class, x.abstract_thematic_surface())
            }
            FeatureRef::TrafficSpace(x) => Self::from_abstract_space(class, x.abstract_space()),
            FeatureRef::WallSurface(x) => {
                Self::from_abstract_thematic_surface(class, x.abstract_thematic_surface())
            }
            FeatureRef::Window(x) => {
                Self::from_abstract_occupied_space(class, x.abstract_occupied_space())
            }
            FeatureRef::WindowSurface(x) => {
                Self::from_abstract_thematic_surface(class, x.abstract_thematic_surface())
            }
        }
    }

    pub fn from_abstract_space(class: FeatureType, space: &AbstractSpace) -> Self {
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
            multi_curves,
            multi_surfaces,
            solids,
            implicit_geometries: HashMap::new(),
            triangulated_surface: HashMap::new(),
        }
    }

    pub fn from_abstract_occupied_space(
        class: FeatureType,
        occupied_space: &AbstractOccupiedSpace,
    ) -> Self {
        let mut city_object_geometry = CityObjectGeometryEntry::from_abstract_space(
            class,
            &occupied_space.abstract_physical_space.abstract_space,
        );

        let implicit_representations: HashMap<LevelOfDetail, ImplicitGeometry> = occupied_space
            .implicit_representations_by_lod()
            .into_iter()
            .map(|(lod, x)| (lod, x.clone()))
            .collect();

        city_object_geometry.implicit_geometries = implicit_representations;
        city_object_geometry
    }

    pub fn from_abstract_thematic_surface(
        class: FeatureType,
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
            multi_curves: HashMap::new(),
            multi_surfaces,
            solids: HashMap::new(),
            implicit_geometries: HashMap::new(),
            triangulated_surface: HashMap::new(),
        }
    }

    pub fn id(&self) -> &Id {
        self.base.id()
    }

    pub fn envelope(&self) -> Option<&Envelope> {
        self.base.bounded_by()
    }

    pub fn has_geometry(&self) -> bool {
        !self.multi_curves.is_empty()
            || !self.multi_surfaces.is_empty()
            || !self.solids.is_empty()
            || !self.implicit_geometries.is_empty()
    }
}
