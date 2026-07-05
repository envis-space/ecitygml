use crate::model::common::{FeatureType, HasFeatureType, LevelOfDetail};
use crate::model::core::refs::{
    CityObjectKindRef, PhysicalSpaceKindRef, SpaceBoundaryKindRef, SpaceKindRef,
};
use crate::model::core::{
    AbstractCityObject, AbstractOccupiedSpace, AbstractSpace, AbstractThematicSurface,
    AsAbstractCityObject, AsAbstractFeature, AsAbstractOccupiedSpace, AsAbstractSpace,
    AsAbstractThematicSurface, CityModel, ImplicitGeometry,
};
use crate::model::relief::AsAbstractReliefComponent;
use crate::model::relief::refs::ReliefComponentKindRef;
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
            .iter_city_objects()
            .map(CityObjectGeometryEntry::from_city_object_ref)
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

    pub fn from_city_object_ref(city_object: CityObjectKindRef<'_>) -> Self {
        let class = city_object.feature_type();
        match city_object {
            CityObjectKindRef::SpaceKind(SpaceKindRef::PhysicalSpaceKind(
                PhysicalSpaceKindRef::OccupiedSpaceKind(x),
            )) => Self::from_abstract_occupied_space(class, x.abstract_occupied_space()),
            CityObjectKindRef::SpaceKind(x) => Self::from_abstract_space(class, x.abstract_space()),
            CityObjectKindRef::SpaceBoundaryKind(SpaceBoundaryKindRef::ThematicSurfaceKind(x)) => {
                Self::from_abstract_thematic_surface(class, x.abstract_thematic_surface())
            }
            CityObjectKindRef::SpaceBoundaryKind(SpaceBoundaryKindRef::ReliefFeature(x)) => {
                Self::new(x.abstract_city_object().clone(), class)
            }
            CityObjectKindRef::SpaceBoundaryKind(SpaceBoundaryKindRef::ReliefComponentKind(
                ReliefComponentKindRef::TinRelief(x),
            )) => {
                let mut entry = Self::new(x.abstract_city_object().clone(), class);
                if let Some(geom) = x.tin().as_ref().and_then(|t| t.object.as_ref()) {
                    entry.triangulated_surface.insert(x.lod(), geom.clone());
                }
                entry
            }
        }
    }

    pub fn from_abstract_space(class: FeatureType, space: &AbstractSpace) -> Self {
        let solids: HashMap<LevelOfDetail, Solid> = space
            .solids_by_lod()
            .into_iter()
            .flat_map(|(lod, x)| x.object.as_ref().map(|x| (lod, x.clone())))
            .collect();

        let multi_surfaces: HashMap<LevelOfDetail, MultiSurface> = space
            .multi_surfaces_by_lod()
            .into_iter()
            .flat_map(|(lod, x)| x.object.as_ref().map(|x| (lod, x.clone())))
            .collect();

        let multi_curves: HashMap<LevelOfDetail, MultiCurve> = space
            .multi_curves_by_lod()
            .into_iter()
            .flat_map(|(lod, x)| x.object.as_ref().map(|x| (lod, x.clone())))
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
            .flat_map(|(lod, x)| x.object.as_ref().map(|x| (lod, x.clone())))
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
