//! `ecitygml` is a library for processing [CityGML](https://www.ogc.org/standards/citygml/) data.
//!
//! Only reading of CityGML version 3.0 datasets is currently supported.
//!
//! ## Example
//!
//! To get started, read a CityGML 3.0 dataset into memory and print all GML IDs of the building.
//!
//! ```no_run
//! use std::path::PathBuf;
//! use ecitygml_io::GmlReader;
//! use crate::ecitygml::model::core::AsAbstractFeature;
//!
//! // read the CityGML dataset
//! let file_path = PathBuf::from("example/city_model.gml");
//! let city_model = GmlReader::from_path(file_path)
//!     .expect("file extension should be correct")
//!     .finish()
//!     .expect("parsing should work");
//!
//! // iterate over all buildings
//! for current_building in city_model.buildings() {
//!     println!("GML ID of the current building: {}", current_building.id());
//! }
//! ```
//!

pub use ecitygml_core::{Error, model, operations};
pub use ecitygml_io as io;

pub use ecitygml_transform as transform;
