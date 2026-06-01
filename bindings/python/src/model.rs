use crate::city_objects::{
    PyBuilding, PyCityFurniture, PyReliefFeature, PyRoad, PySolitaryVegetationObject, PyTinRelief,
};
use ecitygml_rs::model::common::{FeatureRef, TopLevelFeatureRef};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

// ---------------------------------------------------------------------------
// GmlReader
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "GmlReader")]
pub struct PyGmlReader {
    inner: Option<ecitygml_rs::io::GmlReader<std::fs::File>>,
    rebuild_object_bounds: bool,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyGmlReader {
    #[new]
    pub fn from_path(path: &str) -> PyResult<Self> {
        Ok(Self {
            inner: Some(ecitygml_rs::io::GmlReader::from_path(path).map_err(|e| {
                pyo3::exceptions::PyIOError::new_err(format!("Failed to read GML file: {}", e))
            })?),
            rebuild_object_bounds: false,
        })
    }

    pub fn with_rebuild_object_bounds(&mut self, rebuild: bool) -> PyResult<Self> {
        let inner = self.inner.take();
        Ok(Self {
            inner,
            rebuild_object_bounds: rebuild,
        })
    }

    pub fn finish(&mut self) -> PyResult<PyCityModel> {
        let mut reader = self.inner.take().ok_or_else(|| {
            pyo3::exceptions::PyRuntimeError::new_err("GmlReader already consumed by finish()")
        })?;

        if self.rebuild_object_bounds {
            reader = reader.with_rebuild_object_bounds(true);
        }

        let city_model_inner: ecitygml_rs::model::core::CityModel =
            reader.finish().map_err(|e| {
                pyo3::exceptions::PyIOError::new_err(format!("Failed to finish GML parsing: {}", e))
            })?;

        Ok(PyCityModel {
            inner: city_model_inner,
        })
    }

    pub fn __repr__(&self) -> &'static str {
        "GmlReader"
    }
}

// ---------------------------------------------------------------------------
// CityModel
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "CityModel")]
pub struct PyCityModel {
    pub inner: ecitygml_rs::model::core::CityModel,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyCityModel {
    pub fn city_objects_len(&self) -> usize {
        self.inner.city_object_members_len()
    }

    pub fn buildings(&self) -> Vec<PyBuilding> {
        self.inner
            .iter_top_level_features()
            .filter_map(|x| match x {
                TopLevelFeatureRef::Building(b) => Some(PyBuilding::from(b)),
                _ => None,
            })
            .collect()
    }

    pub fn roads(&self) -> Vec<PyRoad> {
        self.inner
            .iter_top_level_features()
            .filter_map(|x| match x {
                TopLevelFeatureRef::Road(r) => Some(PyRoad::from(r)),
                _ => None,
            })
            .collect()
    }

    pub fn solitary_vegetation_objects(&self) -> Vec<PySolitaryVegetationObject> {
        self.inner
            .iter_top_level_features()
            .filter_map(|x| match x {
                TopLevelFeatureRef::SolitaryVegetationObject(v) => {
                    Some(PySolitaryVegetationObject::from(v))
                }
                _ => None,
            })
            .collect()
    }

    pub fn city_furniture_objects(&self) -> Vec<PyCityFurniture> {
        self.inner
            .iter_top_level_features()
            .filter_map(|x| match x {
                TopLevelFeatureRef::CityFurniture(cf) => Some(PyCityFurniture::from(cf)),
                _ => None,
            })
            .collect()
    }

    pub fn relief_features(&self) -> Vec<PyReliefFeature> {
        self.inner
            .iter_top_level_features()
            .filter_map(|x| match x {
                TopLevelFeatureRef::ReliefFeature(rf) => Some(PyReliefFeature::from(rf)),
                _ => None,
            })
            .collect()
    }

    pub fn tin_reliefs(&self) -> Vec<PyTinRelief> {
        self.inner
            .iter_features()
            .filter_map(|x| match x {
                FeatureRef::TinRelief(tr) => Some(PyTinRelief::from(tr)),
                _ => None,
            })
            .collect()
    }

    pub fn recompute_child_bounding_shapes(&mut self) {
        self.inner.recompute_child_bounding_shapes();
    }

    pub fn __repr__(&self) -> String {
        format!("CityModel({} city objects)", self.inner.city_object_members_len())
    }
}
