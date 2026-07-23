//! Code lists published by SIG 3D.
//!
//! [SIG 3D](https://www.sig3d.org/en/home) ("Special Interest Group 3D",
//! part of Geoinfo.NRW / GDI-DE) is the working group that maintains the
//! German CityGML profile and publishes standardized code lists for CityGML
//! thematic properties such as `class`, `function`, `usage`, `roofType`,
//! and material/surface codes. The full set of published code lists is
//! browsable at <https://www.sig3d.org/codelists/standard/>.
//!
//! Each code list is versioned; only [`v2`] (SIG3D code lists version 2.0,
//! aligned with CityGML 3.0) is implemented here. Every enum variant
//! documents the specific code list XML file it corresponds to.

pub mod v2;
