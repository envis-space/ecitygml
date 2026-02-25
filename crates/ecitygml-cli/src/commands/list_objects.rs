use crate::error::Error;
use ecitygml::model::core::AsAbstractFeature;
use std::path::Path;
use std::time::Instant;
use tracing::info;

pub fn run(file_path: impl AsRef<Path>) -> Result<(), Error> {
    let metadata = std::fs::metadata(file_path.as_ref())?;
    info!(
        "Start reading model of size {} located at {}",
        humansize::format_size(metadata.len(), humansize::DECIMAL),
        file_path.as_ref().display(),
    );
    let now = Instant::now();
    let mut city_model = ecitygml::io::GmlReader::from_path(file_path.as_ref())?.finish()?;
    let time_elapsed = now.elapsed();
    info!("Read model in {:.3?}", time_elapsed);

    let now = Instant::now();
    city_model.refresh_bounded_by_recursive();
    info!("Refreshed bounded_by in {:.3?}", now.elapsed());

    for current_city_object in city_model.iter_city_object() {
        let envelope_str = current_city_object
            .bounded_by()
            .map_or_else(String::new, |e| format!(", {}", e));

        info!(
            "   ID: {}, class: {}{}",
            current_city_object.id(),
            current_city_object.city_object_class(),
            envelope_str,
        );
    }

    Ok(())
}
