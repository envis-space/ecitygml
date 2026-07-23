//! Code lists published by the AdV (Arbeitsgemeinschaft der
//! Vermessungsverwaltungen der Länder der Bundesrepublik Deutschland — the
//! working group of the German federal states' surveying authorities).
//!
//! AdV maintains the code lists used by the official German cadastral
//! (ALKIS) and topographic (ATKIS) CityGML profiles, published at
//! <https://repository.gdi-de.org/schemas/adv/citygml/Codelisten/>. Unlike
//! [`sig3d`](crate::codelists::sig3d), these lists are not versioned by a
//! `vX` path segment, so submodules here are grouped directly by CityGML
//! theme.

pub mod building;
