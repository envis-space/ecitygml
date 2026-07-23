//! External code list definitions referenced by CityGML properties.
//!
//! A code list constrains a `gml:CodeType`-based property (e.g. `class`,
//! `function`, `usage`) to a fixed, externally defined vocabulary identified
//! by a `codeSpace` URI. This module provides typed Rust representations of
//! such code lists so that code values can be interpreted and produced
//! without hand-parsing the `codeSpace`/value pair.

mod error;

#[doc(inline)]
pub use crate::error::Error;
