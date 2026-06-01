use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;

mod city_objects;
mod enums;
mod geometry;
mod model;

use city_objects::{
    PyAuxiliaryTrafficArea, PyAuxiliaryTrafficSpace, PyBuilding, PyCityFurniture, PyDoorSurface,
    PyGroundSurface, PyIntersection, PyReliefFeature, PyRoad, PyRoofSurface, PySection,
    PySolitaryVegetationObject, PyTinRelief, PyTrafficArea, PyTrafficSpace, PyWallSurface,
    PyWindowSurface,
};
use enums::{PyFeatureType, PyLevelOfDetail};
use geometry::{
    PyDirectPosition, PyEnvelope, PyLinearRing, PyMultiCurve, PyMultiSurface, PyPolygon, PySolid,
    PyTriangle, PyTriangulatedSurface,
};
use model::{PyCityModel, PyGmlReader};

#[pymodule]
#[pyo3(name = "_ecitygml")]
fn ecitygml(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Geometry
    m.add_class::<PyDirectPosition>()?;
    m.add_class::<PyEnvelope>()?;
    m.add_class::<PyLinearRing>()?;
    m.add_class::<PyPolygon>()?;
    m.add_class::<PyTriangle>()?;
    m.add_class::<PyTriangulatedSurface>()?;
    m.add_class::<PySolid>()?;
    m.add_class::<PyMultiSurface>()?;
    m.add_class::<PyMultiCurve>()?;

    // Enums
    m.add_class::<PyLevelOfDetail>()?;
    m.add_class::<PyFeatureType>()?;

    // Model
    m.add_class::<PyGmlReader>()?;
    m.add_class::<PyCityModel>()?;

    // City objects
    m.add_class::<PyBuilding>()?;
    m.add_class::<PyWallSurface>()?;
    m.add_class::<PyRoofSurface>()?;
    m.add_class::<PyGroundSurface>()?;
    m.add_class::<PyDoorSurface>()?;
    m.add_class::<PyWindowSurface>()?;
    m.add_class::<PyRoad>()?;
    m.add_class::<PySection>()?;
    m.add_class::<PyIntersection>()?;
    m.add_class::<PyTrafficSpace>()?;
    m.add_class::<PyTrafficArea>()?;
    m.add_class::<PyAuxiliaryTrafficSpace>()?;
    m.add_class::<PyAuxiliaryTrafficArea>()?;
    m.add_class::<PySolitaryVegetationObject>()?;
    m.add_class::<PyCityFurniture>()?;
    m.add_class::<PyReliefFeature>()?;
    m.add_class::<PyTinRelief>()?;

    Ok(())
}

define_stub_info_gatherer!(stub_info);
