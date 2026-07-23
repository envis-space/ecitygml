//! SIG3D code lists, version 2.0.
//!
//! Source: <https://www.sig3d.org/codelists/standard/> (browse by thematic
//! module, version 2.0). Submodules here mirror that thematic grouping —
//! e.g. [`building`] holds `class`/`function`/`usage`/`roofType` code lists
//! for `Building`/`BuildingPart`, [`transportation`] holds the traffic and
//! auxiliary traffic area code lists, and so on. Each leaf type links to
//! the exact code list XML file it implements.

pub mod appearance;
pub mod bridge;
pub mod building;
pub mod city_furniture;
pub mod city_object_group;
pub mod core;
pub mod land_use;
pub mod transportation;
pub mod tunnel;
pub mod vegetation;
pub mod water_body;
