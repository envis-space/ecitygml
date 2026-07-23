use crate::error::Error;
use ecitygml::arena::CityModelArena;
use ecitygml::io::CitygmlFormat;
use ecitygml::model::common::{FeatureType, HasFeatureType, LevelOfDetail};
use ecitygml::model::core::refs::{
    AbstractCityObjectKindRef, AbstractPhysicalSpaceKindRef, AbstractSpaceBoundaryKindRef,
    AbstractSpaceKindRef,
};
use ecitygml::model::core::{
    AsAbstractFeature, AsAbstractOccupiedSpace, AsAbstractSpace, AsAbstractThematicSurface,
};
use ecitygml::store::ImplicitGeometryStore;
use egml::model::base::Id;
use std::collections::HashSet;
use std::path::Path;
use std::time::Instant;
use strum::IntoEnumIterator;
use tracing::info;
use walkdir::WalkDir;

pub fn run(path: impl AsRef<Path>) -> Result<(), Error> {
    info!("Creating statistics for: {}", path.as_ref().display());

    if path.as_ref().is_file() {
        print_city_model_statistics(path)?;
    } else if path.as_ref().is_dir() {
        for entry in WalkDir::new(path)
            .sort_by(|a, b| a.file_name().cmp(b.file_name()))
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file() && e.path().extension().is_some())
            .filter(|e| CitygmlFormat::from_path(e.path()).is_some())
        {
            print_city_model_statistics(entry.into_path())?;
        }
    }

    Ok(())
}

fn print_city_model_statistics(file_path: impl AsRef<Path>) -> Result<(), Error> {
    let metadata = std::fs::metadata(file_path.as_ref())?;
    info!(
        "Start reading model of size {} located at {}",
        humansize::format_size(metadata.len(), humansize::DECIMAL),
        file_path.as_ref().display(),
    );
    let now = Instant::now();
    let city_model = ecitygml::io::GmlReader::from_path(file_path.as_ref())?
        // .with_rebuild_object_bounds(false)
        .finish()?;
    let time_elapsed = now.elapsed();
    info!("Read model in {:.3?}", time_elapsed);

    let implicit_geometry_store = ImplicitGeometryStore::from_city_model(&city_model);
    let city_model_arena = CityModelArena::from_city_model(city_model)?;
    let city_object_count = city_model_arena.iter_city_objects().count();
    info!(
        "Number of city objects: {} (read speed: {:.3?} objects/s)",
        city_object_count,
        city_object_count as f64 / time_elapsed.as_secs_f64()
    );

    for current_city_object_class in FeatureType::iter() {
        print_object_class_statistics(
            &city_model_arena,
            &implicit_geometry_store,
            current_city_object_class,
        )?;
    }
    println!();

    Ok(())
}

/// Per-object LOD presence, gathered directly from the arena without materializing full
/// geometry (no cloning of solids/surfaces/curves) — just which LODs are present, which is
/// all these statistics need.
struct ObjectGeometryLods {
    id: Id,
    solid_lods: HashSet<LevelOfDetail>,
    multi_surface_lods: HashSet<LevelOfDetail>,
    multi_curve_lods: HashSet<LevelOfDetail>,
}

fn object_geometry_lods(city_object: AbstractCityObjectKindRef<'_>) -> ObjectGeometryLods {
    let id = city_object.feature_id().clone();

    let (solid_lods, multi_surface_lods, multi_curve_lods) = match city_object {
        AbstractCityObjectKindRef::AbstractSpaceKind(
            AbstractSpaceKindRef::AbstractPhysicalSpaceKind(
                AbstractPhysicalSpaceKindRef::AbstractOccupiedSpaceKind(x),
            ),
        ) => {
            let space = x.abstract_occupied_space();
            (
                space.solids_by_lod().into_keys().collect(),
                space.multi_surfaces_by_lod().into_keys().collect(),
                space.multi_curves_by_lod().into_keys().collect(),
            )
        }
        AbstractCityObjectKindRef::AbstractSpaceKind(x) => {
            let space = x.abstract_space();
            (
                space.solids_by_lod().into_keys().collect(),
                space.multi_surfaces_by_lod().into_keys().collect(),
                space.multi_curves_by_lod().into_keys().collect(),
            )
        }
        AbstractCityObjectKindRef::AbstractSpaceBoundaryKind(
            AbstractSpaceBoundaryKindRef::AbstractThematicSurfaceKind(x),
        ) => {
            let surface = x.abstract_thematic_surface();
            (
                HashSet::new(),
                surface.multi_surfaces_by_lod().into_keys().collect(),
                HashSet::new(),
            )
        }
        AbstractCityObjectKindRef::AbstractSpaceBoundaryKind(_) => {
            (HashSet::new(), HashSet::new(), HashSet::new())
        }
    };

    ObjectGeometryLods {
        id,
        solid_lods,
        multi_surface_lods,
        multi_curve_lods,
    }
}

fn print_object_class_statistics(
    city_model_arena: &CityModelArena,
    implicit_geometry_store: &ImplicitGeometryStore,
    feature_type: FeatureType,
) -> Result<(), Error> {
    let filtered_objects: Vec<ObjectGeometryLods> = city_model_arena
        .iter_city_objects()
        .filter(|x| x.feature_type() == feature_type)
        .map(object_geometry_lods)
        .collect();

    if filtered_objects.is_empty() {
        return Ok(());
    }

    info!(
        "   Class: {} (obj count: {})",
        feature_type,
        filtered_objects.len()
    );

    for current_level_of_detail in LevelOfDetail::iter() {
        let count = filtered_objects
            .iter()
            .filter(|x| x.solid_lods.contains(&current_level_of_detail))
            .count();
        if count > 0 {
            info!(
                "       lod{}Solids: {}",
                current_level_of_detail.as_index(),
                count
            );
        }
    }

    for current_level_of_detail in LevelOfDetail::iter() {
        let count = filtered_objects
            .iter()
            .filter(|x| x.multi_surface_lods.contains(&current_level_of_detail))
            .count();
        if count > 0 {
            info!(
                "       lod{}MultiSurface: {}",
                current_level_of_detail.as_index(),
                count
            );
        }
    }

    for current_level_of_detail in LevelOfDetail::iter() {
        let count = filtered_objects
            .iter()
            .filter(|x| x.multi_curve_lods.contains(&current_level_of_detail))
            .count();
        if count > 0 {
            info!(
                "       lod{}MultiCurve: {}",
                current_level_of_detail.as_index(),
                count
            );
        }
    }

    for current_level_of_detail in LevelOfDetail::iter() {
        let count = filtered_objects
            .iter()
            .filter(|x| implicit_geometry_store.has_placement_at(&x.id, current_level_of_detail))
            .count();
        if count > 0 {
            info!(
                "       lod{}ImplicitGeometry: {}",
                current_level_of_detail.as_index(),
                count
            );
        }
    }

    Ok(())
}
