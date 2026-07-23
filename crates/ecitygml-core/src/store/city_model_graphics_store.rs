use crate::Error;
use crate::arena::CityModelArena;
use crate::model::common::LevelOfDetail;
use crate::model::core::CityModel;
use crate::model::core::refs::{
    AbstractCityObjectKindRef, AbstractPhysicalSpaceKindRef, AbstractSpaceBoundaryKindRef,
    AbstractSpaceKindRef,
};
use crate::model::core::{AsAbstractOccupiedSpace, AsAbstractSpace, AsAbstractThematicSurface};
use crate::model::relief::refs::AbstractReliefComponentKindRef;
use crate::store::ImplicitGeometryStore;
use egml::model::common::{Triangulate, Triangulation};
use egml::model::geometry::aggregates::{MultiSurface, MultiSurfaceProperty};
use egml::model::geometry::primitives::{Solid, SolidProperty};
use std::collections::HashMap;

/// Level of detail preference used whenever multiple representations of the same object are
/// available and only one can be shown at a time.
const LOD_PRIORITY: [LevelOfDetail; 4] = [
    LevelOfDetail::Three,
    LevelOfDetail::Two,
    LevelOfDetail::One,
    LevelOfDetail::Zero,
];

#[derive(Debug, Clone, Default)]
pub struct CityModelGraphicsStore {
    pub city_model_arena: CityModelArena,
    pub implicit_geometry_store: ImplicitGeometryStore,
}

impl CityModelGraphicsStore {
    pub fn from_city_model(city_model: CityModel) -> Result<Self, Error> {
        let implicit_geometry_store = ImplicitGeometryStore::from_city_model(&city_model);
        let city_model_arena = CityModelArena::from_city_model(city_model)?;

        Ok(Self {
            city_model_arena,
            implicit_geometry_store,
        })
    }

    /// Picks the best available geometry for a city object across LODs and triangulates it:
    /// a precomputed triangulated surface (e.g. `TinRelief`) is preferred, then solids, then
    /// multi-surfaces, always favoring higher LODs first. Callers decide what to do with any
    /// skipped members (e.g. logging), since that policy differs by caller.
    pub fn best_triangulated_surface(
        city_object: AbstractCityObjectKindRef<'_>,
    ) -> Option<Triangulation> {
        if let AbstractCityObjectKindRef::AbstractSpaceBoundaryKind(
            AbstractSpaceBoundaryKindRef::AbstractReliefComponentKind(
                AbstractReliefComponentKindRef::TinRelief(x),
            ),
        ) = city_object
        {
            return x
                .tin()
                .and_then(|tin| tin.object())
                .cloned()
                .map(|surface| Triangulation::new(surface, Vec::new()));
        }

        match city_object {
            AbstractCityObjectKindRef::AbstractSpaceKind(
                AbstractSpaceKindRef::AbstractPhysicalSpaceKind(
                    AbstractPhysicalSpaceKindRef::AbstractOccupiedSpaceKind(x),
                ),
            ) => {
                let space = x.abstract_occupied_space();
                pick_best_surface(space.solids_by_lod(), space.multi_surfaces_by_lod())
            }
            AbstractCityObjectKindRef::AbstractSpaceKind(x) => {
                let space = x.abstract_space();
                pick_best_surface(space.solids_by_lod(), space.multi_surfaces_by_lod())
            }
            AbstractCityObjectKindRef::AbstractSpaceBoundaryKind(
                AbstractSpaceBoundaryKindRef::AbstractThematicSurfaceKind(x),
            ) => pick_best_surface(
                HashMap::new(),
                x.abstract_thematic_surface().multi_surfaces_by_lod(),
            ),
            AbstractCityObjectKindRef::AbstractSpaceBoundaryKind(_) => None,
        }
    }
}

fn pick_best_surface(
    solids: HashMap<LevelOfDetail, &SolidProperty>,
    multi_surfaces: HashMap<LevelOfDetail, &MultiSurfaceProperty>,
) -> Option<Triangulation> {
    for lod in LOD_PRIORITY {
        if let Some(triangulation) = solids
            .get(&lod)
            .and_then(|p| p.object())
            .and_then(triangulate_solid)
        {
            return Some(triangulation);
        }
    }
    for lod in LOD_PRIORITY {
        if let Some(triangulation) = multi_surfaces
            .get(&lod)
            .and_then(|p| p.object())
            .and_then(triangulate_multi_surface)
        {
            return Some(triangulation);
        }
    }

    None
}

fn triangulate_solid(solid: &Solid) -> Option<Triangulation> {
    let shell = solid.exterior().and_then(|sp| sp.object())?;
    shell.triangulate().ok()
}

fn triangulate_multi_surface(ms: &MultiSurface) -> Option<Triangulation> {
    ms.triangulate().ok()
}
