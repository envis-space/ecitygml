use ecitygml_rs;
use ecitygml_rs::model::core::AsAbstractFeature;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

#[pyclass]
struct GmlReader {
    inner: Option<ecitygml_rs::io::GmlReader<std::fs::File>>,
}

#[pymethods]
impl GmlReader {
    #[new]
    pub fn from_path(path: &str) -> PyResult<Self> {
        Ok(Self {
            inner: Some(ecitygml_rs::io::GmlReader::from_path(path).map_err(|e| {
                pyo3::exceptions::PyIOError::new_err(format!("Failed to read GML file: {}", e))
            })?),
        })
    }

    pub fn finish(&mut self) -> Result<CityModel, PyErr> {
        let reader = self.inner.take().ok_or_else(|| {
            pyo3::exceptions::PyRuntimeError::new_err("GmlReader already consumed by finish()")
        })?;

        let city_model_inner: ecitygml_rs::model::core::CityModel =
            reader.finish().map_err(|e| {
                pyo3::exceptions::PyIOError::new_err(format!("Failed to finish GML parsing: {}", e))
            })?;

        let city_model = CityModel {
            inner: city_model_inner,
        };

        Ok(city_model)
    }
}

#[pyclass]
struct CityModel {
    inner: ecitygml_rs::model::core::CityModel,
}

#[pymethods]
impl CityModel {
    pub fn city_objects_len(&self) -> usize {
        self.inner.city_objects_len()
    }

    pub fn city_objects(&self) -> Vec<CityObjectKind> {
        self.inner
            .city_objects()
            .iter()
            .map(|x| CityObjectKind { inner: x.clone() })
            .collect()
    }

    pub fn refresh_bounded_by_recursive(&mut self) {
        self.inner.refresh_bounded_by_recursive()
    }
}

#[pyclass]
struct CityObjectKind {
    inner: ecitygml_rs::model::core::CityObjectKind,
}

#[pymethods]
impl CityObjectKind {
    pub fn id(&self) -> String {
        self.inner.id().to_string()
    }

    pub fn city_object_class(&self) -> String {
        self.inner.city_object_class().to_string()
    }

    pub fn bounded_by(&self) -> Option<Envelope> {
        self.inner.bounded_by().map(|e| Envelope {
            lower_corner: vec![
                e.lower_corner().x(),
                e.lower_corner().y(),
                e.lower_corner().z(),
            ],
            upper_corner: vec![
                e.upper_corner().x(),
                e.upper_corner().y(),
                e.upper_corner().z(),
            ],
        })
    }
}

#[pyclass]
struct Envelope {
    lower_corner: Vec<f64>,
    upper_corner: Vec<f64>,
}

#[pymethods]
impl Envelope {
    #[getter]
    fn lower_corner(&self) -> (f64, f64, f64) {
        (
            self.lower_corner[0],
            self.lower_corner[1],
            self.lower_corner[2],
        )
    }

    #[getter]
    fn upper_corner(&self) -> (f64, f64, f64) {
        (
            self.upper_corner[0],
            self.upper_corner[1],
            self.upper_corner[2],
        )
    }
}

/// An example module implemented in Rust using PyO3.
#[pymodule]
fn ecitygml(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GmlReader>()?;
    m.add_class::<CityModel>()?;
    m.add_class::<CityObjectKind>()?;
    m.add_class::<Envelope>()?;
    // m.add_wrapped(wrap_pymodule!(submodule::submodule))?;

    let sys = PyModule::import(py, "sys")?;
    let sys_modules: Bound<'_, PyDict> = sys.getattr("modules")?.cast_into()?;
    // sys_modules.set_item("ecitygml.submodule", m.getattr("submodule")?)?;

    Ok(())
}
