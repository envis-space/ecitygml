use crate::error::Error;
use ecitygml::io::CitygmlFormat;
use ecitygml::model::common::{CityObjectClass, LevelOfDetail};
use ecitygml::operations::{CityModelGeometryIndex, CityObjectGeometry};
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
    let city_model = ecitygml::io::GmlReader::from_path(file_path.as_ref())?.finish()?;
    let time_elapsed = now.elapsed();
    info!("Read model in {:.3?}", time_elapsed);

    let city_model_geometry_index = CityModelGeometryIndex::from_city_model(city_model);
    info!(
        "Number of city objects: {} (read speed: {:.3?} objects/s)",
        city_model_geometry_index.objects_len(),
        city_model_geometry_index.objects_len() as f64 / time_elapsed.as_secs_f64()
    );

    for current_city_object_class in CityObjectClass::iter() {
        print_object_class_statistics(&city_model_geometry_index, current_city_object_class)?;
    }
    println!();

    Ok(())
}

fn print_object_class_statistics(
    city_model_geometry_index: &CityModelGeometryIndex,
    current_city_object_class: CityObjectClass,
) -> Result<(), Error> {
    let filtered_objects: Vec<&CityObjectGeometry> = city_model_geometry_index
        .objects
        .values()
        .filter(|x| x.class == current_city_object_class)
        .collect();

    if filtered_objects.is_empty() {
        return Ok(());
    }

    info!(
        "   Class: {} (obj count: {})",
        current_city_object_class,
        filtered_objects.len()
    );

    for current_level_of_detail in LevelOfDetail::iter() {
        let count = filtered_objects
            .iter()
            .filter(|x| x.solids.contains_key(&current_level_of_detail))
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
            .filter(|x| x.multi_surfaces.contains_key(&current_level_of_detail))
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
            .filter(|x| x.multi_curves.contains_key(&current_level_of_detail))
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
            .filter(|x| x.implicit_geometries.contains_key(&current_level_of_detail))
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
