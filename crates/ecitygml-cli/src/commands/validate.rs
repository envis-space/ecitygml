use crate::error::Error;
use ecitygml::io::GmlReader;
use std::fs;
use std::path::Path;
use tracing::info;

pub fn run(
    file_path: impl AsRef<Path>,
    output_directory_path: impl AsRef<Path>,
) -> Result<(), Error> {
    info!("Start validation");

    let reader = GmlReader::from_path(file_path)?;
    let report = reader.validate()?;

    fs::create_dir_all(&output_directory_path)?;
    let path = output_directory_path
        .as_ref()
        .to_owned()
        .join("report.yaml");
    report.write(path);

    /*if report.contains_gml_id_duplicates() {
        warn!("GML ID duplicates identified: ");
        for (id, count) in report.get_gml_id_duplicates() {
            warn!("id: {id} count: {count}");
        }
    } else {
        info!("No GML duplicates found.");
    }

    let dead_object_relations = report.find_dead_object_relations();
    info!(
        "Dead object relations: {}/{}",
        dead_object_relations.len(),
        report.city_object_relations.len()
    );

    if dead_object_relations.is_empty() {
        info!("No dead object relations found.");
    } else {
        warn!("Dead object relations identified: ");
        for r in dead_object_relations {
            warn!("id: {}", r.related_to.xlink_href);
        }
    }*/

    info!("");

    /*for r in report.gml_id_count_per_element_type {
        info!(
            "name: {}, with id {}, without id {}",
            r.0, r.1.with_gml_id_count, r.1.without_gml_id_count
        );
    }*/

    Ok(())
}
