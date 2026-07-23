use crate::error::Error;
use ecitygml::model::common::{HasFeatureType, IterFeatures};
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
    city_model.recompute_child_bounding_shapes();
    info!("Refreshed bounded_by in {:.3?}", now.elapsed());

    for current_city_object in city_model.iter_features() {
        let envelope_str = egml::model::feature::AsAbstractFeature::bounded_by(
            current_city_object.abstract_feature(),
        )
        .and_then(|x| x.envelope())
        .map_or_else(String::new, |e| format!(", {}", e));

        info!(
            "   ID: {}, class: {}, envelope: {}",
            current_city_object.feature_id(),
            current_city_object.feature_type(),
            envelope_str,
        );
    }

    Ok(())
}
