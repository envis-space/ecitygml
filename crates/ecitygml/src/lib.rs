//! `ecitygml` is a library for processing [CityGML](https://www.ogc.org/standards/citygml/) data.
//!
//! ## Example
//!
//! To get started, read a CityGML 3.0 dataset into memory and print all GML IDs of the building.
//!
//! ```no_run
//! use std::path::PathBuf;
//! use ecitygml_io::GmlReader;
//! use crate::ecitygml::model::common::IterFeatures;
//! use crate::ecitygml::model::core::AsAbstractFeature;
//!
//! // read the CityGML dataset
//! let file_path = PathBuf::from("example/city_model.gml");
//! let city_model = GmlReader::from_path(file_path)
//!     .expect("file extension should be correct")
//!     .finish()
//!     .expect("parsing should work");
//!
//! // iterate over all features
//! for current_feature_ref in city_model.iter_features() {
//!     println!("GML ID of the current feature: {}", current_feature_ref.feature_id());
//! }
//! ```
//!

pub use ecitygml_core::{Error, arena, index, model, resolver, store};
pub use ecitygml_io as io;

pub use ecitygml_transform as transform;
