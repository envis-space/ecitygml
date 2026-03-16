use egml::model::geometry::DirectPosition as RustDirectPosition;
use egml::model::geometry::Envelope as RustEnvelope;
use egml::model::geometry::aggregates::{MultiCurve as RustMultiCurve, MultiSurface as RustMultiSurface};
use egml::model::geometry::primitives::{
    LinearRing as RustLinearRing, Polygon as RustPolygon, RingPropertyKind, Solid as RustSolid,
    Triangle as RustTriangle, TriangulatedSurface as RustTriangulatedSurface, SurfaceKind,
    CurveKind,
};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

// ---------------------------------------------------------------------------
// DirectPosition
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "DirectPosition", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyDirectPosition {
    pub inner: RustDirectPosition,
}

impl From<RustDirectPosition> for PyDirectPosition {
    fn from(inner: RustDirectPosition) -> Self {
        Self { inner }
    }
}

impl From<&RustDirectPosition> for PyDirectPosition {
    fn from(inner: &RustDirectPosition) -> Self {
        Self { inner: *inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyDirectPosition {
    #[getter]
    pub fn x(&self) -> f64 {
        self.inner.x()
    }

    #[getter]
    pub fn y(&self) -> f64 {
        self.inner.y()
    }

    #[getter]
    pub fn z(&self) -> f64 {
        self.inner.z()
    }

    #[getter]
    pub fn coords(&self) -> (f64, f64, f64) {
        let c = self.inner.coords();
        (c[0], c[1], c[2])
    }

    pub fn __repr__(&self) -> String {
        format!(
            "DirectPosition(x={}, y={}, z={})",
            self.inner.x(),
            self.inner.y(),
            self.inner.z()
        )
    }
}

// ---------------------------------------------------------------------------
// Envelope
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "Envelope", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyEnvelope {
    pub inner: RustEnvelope,
}

impl From<RustEnvelope> for PyEnvelope {
    fn from(inner: RustEnvelope) -> Self {
        Self { inner }
    }
}

impl From<&RustEnvelope> for PyEnvelope {
    fn from(inner: &RustEnvelope) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyEnvelope {
    #[getter]
    pub fn lower_corner(&self) -> (f64, f64, f64) {
        let lc = self.inner.lower_corner();
        (lc.x(), lc.y(), lc.z())
    }

    #[getter]
    pub fn upper_corner(&self) -> (f64, f64, f64) {
        let uc = self.inner.upper_corner();
        (uc.x(), uc.y(), uc.z())
    }

    #[getter]
    pub fn size_x(&self) -> f64 {
        self.inner.size_x()
    }

    #[getter]
    pub fn size_y(&self) -> f64 {
        self.inner.size_y()
    }

    #[getter]
    pub fn size_z(&self) -> f64 {
        self.inner.size_z()
    }

    #[getter]
    pub fn volume(&self) -> f64 {
        self.inner.volume()
    }

    pub fn contains(&self, pos: &PyDirectPosition) -> bool {
        self.inner.contains(&pos.inner)
    }

    pub fn __repr__(&self) -> String {
        let lc = self.inner.lower_corner();
        let uc = self.inner.upper_corner();
        format!(
            "Envelope([{}, {}, {}] -> [{}, {}, {}])",
            lc.x(), lc.y(), lc.z(),
            uc.x(), uc.y(), uc.z()
        )
    }
}

// ---------------------------------------------------------------------------
// LinearRing
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "LinearRing", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyLinearRing {
    pub inner: RustLinearRing,
}

impl From<RustLinearRing> for PyLinearRing {
    fn from(inner: RustLinearRing) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyLinearRing {
    #[getter]
    pub fn points(&self) -> Vec<PyDirectPosition> {
        self.inner.points().iter().map(|p| PyDirectPosition::from(p)).collect()
    }

    pub fn __repr__(&self) -> String {
        format!("LinearRing({} points)", self.inner.points().len())
    }
}

// Helper to convert a RingPropertyKind reference into a PyLinearRing
pub fn ring_property_to_py(ring: &RingPropertyKind) -> PyLinearRing {
    match ring {
        RingPropertyKind::LinearRing(lr) => PyLinearRing { inner: lr.clone() },
        RingPropertyKind::RingKind(_) => {
            // RingKind is not common — build a fallback with the available points
            // We need at least 3 points; if data is valid (from GML) it should be fine
            use egml::model::geometry::primitives::AbstractRing;
            let pts = ring.points().to_vec();
            let lr = RustLinearRing::new(AbstractRing::default(), pts)
                .expect("ring from valid GML has >= 3 non-duplicate points");
            PyLinearRing { inner: lr }
        }
    }
}

// ---------------------------------------------------------------------------
// Polygon
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "Polygon", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyPolygon {
    pub inner: RustPolygon,
}

impl From<RustPolygon> for PyPolygon {
    fn from(inner: RustPolygon) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyPolygon {
    #[getter]
    pub fn exterior(&self) -> Option<PyLinearRing> {
        self.inner.exterior.as_ref().map(ring_property_to_py)
    }

    #[getter]
    pub fn interior(&self) -> Vec<PyLinearRing> {
        self.inner.interior.iter().map(ring_property_to_py).collect()
    }

    pub fn compute_envelope(&self) -> Option<PyEnvelope> {
        if self.inner.exterior.is_some() {
            Some(PyEnvelope::from(self.inner.compute_envelope()))
        } else {
            None
        }
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Polygon(exterior={}, interior={})",
            self.inner.exterior.is_some(),
            self.inner.interior.len()
        )
    }
}

// ---------------------------------------------------------------------------
// Triangle
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "Triangle", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyTriangle {
    pub inner: RustTriangle,
}

impl From<RustTriangle> for PyTriangle {
    fn from(inner: RustTriangle) -> Self {
        Self { inner }
    }
}

impl From<&RustTriangle> for PyTriangle {
    fn from(inner: &RustTriangle) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyTriangle {
    #[getter]
    pub fn a(&self) -> PyDirectPosition {
        PyDirectPosition::from(self.inner.a)
    }

    #[getter]
    pub fn b(&self) -> PyDirectPosition {
        PyDirectPosition::from(self.inner.b)
    }

    #[getter]
    pub fn c(&self) -> PyDirectPosition {
        PyDirectPosition::from(self.inner.c)
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Triangle(a={}, b={}, c={})",
            PyDirectPosition::from(self.inner.a).__repr__(),
            PyDirectPosition::from(self.inner.b).__repr__(),
            PyDirectPosition::from(self.inner.c).__repr__()
        )
    }
}

// ---------------------------------------------------------------------------
// TriangulatedSurface
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "TriangulatedSurface", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyTriangulatedSurface {
    pub inner: RustTriangulatedSurface,
}

impl From<RustTriangulatedSurface> for PyTriangulatedSurface {
    fn from(inner: RustTriangulatedSurface) -> Self {
        Self { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyTriangulatedSurface {
    #[getter]
    pub fn triangles(&self) -> Vec<PyTriangle> {
        self.inner.triangles().iter().map(|t| PyTriangle::from(*t)).collect()
    }

    pub fn compute_envelope(&self) -> PyEnvelope {
        PyEnvelope::from(self.inner.compute_envelope())
    }

    pub fn __repr__(&self) -> String {
        format!("TriangulatedSurface({} triangles)", self.inner.triangles().len())
    }
}

// ---------------------------------------------------------------------------
// Solid
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "Solid", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PySolid {
    pub inner: RustSolid,
}

impl From<RustSolid> for PySolid {
    fn from(inner: RustSolid) -> Self {
        Self { inner }
    }
}

impl From<&RustSolid> for PySolid {
    fn from(inner: &RustSolid) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PySolid {
    pub fn polygons(&self) -> Vec<PyPolygon> {
        self.inner
            .members()
            .iter()
            .filter_map(|sp| match &sp.content {
                SurfaceKind::Polygon(p) => Some(PyPolygon::from(p.clone())),
                _ => None,
            })
            .collect()
    }

    pub fn triangulate(&self) -> PyResult<PyTriangulatedSurface> {
        self.inner
            .triangulate()
            .map(PyTriangulatedSurface::from)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    pub fn compute_envelope(&self) -> PyEnvelope {
        PyEnvelope::from(self.inner.compute_envelope())
    }

    pub fn __repr__(&self) -> String {
        format!("Solid({} surfaces)", self.inner.members().len())
    }
}

// ---------------------------------------------------------------------------
// MultiSurface
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "MultiSurface", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyMultiSurface {
    pub inner: RustMultiSurface,
}

impl From<RustMultiSurface> for PyMultiSurface {
    fn from(inner: RustMultiSurface) -> Self {
        Self { inner }
    }
}

impl From<&RustMultiSurface> for PyMultiSurface {
    fn from(inner: &RustMultiSurface) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyMultiSurface {
    pub fn polygons(&self) -> Vec<PyPolygon> {
        self.inner
            .surface_member()
            .iter()
            .filter_map(|sk| match sk {
                SurfaceKind::Polygon(p) => Some(PyPolygon::from(p.clone())),
                _ => None,
            })
            .collect()
    }

    pub fn triangulate(&self) -> PyResult<PyTriangulatedSurface> {
        self.inner
            .triangulate()
            .map(PyTriangulatedSurface::from)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    pub fn compute_envelope(&self) -> PyEnvelope {
        PyEnvelope::from(self.inner.compute_envelope())
    }

    pub fn __repr__(&self) -> String {
        format!("MultiSurface({} members)", self.inner.surface_member().len())
    }
}

// ---------------------------------------------------------------------------
// MultiCurve
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "MultiCurve", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyMultiCurve {
    pub inner: RustMultiCurve,
}

impl From<RustMultiCurve> for PyMultiCurve {
    fn from(inner: RustMultiCurve) -> Self {
        Self { inner }
    }
}

impl From<&RustMultiCurve> for PyMultiCurve {
    fn from(inner: &RustMultiCurve) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyMultiCurve {
    pub fn curves(&self) -> Vec<Vec<PyDirectPosition>> {
        self.inner
            .curve_member()
            .iter()
            .map(|ck| match ck {
                CurveKind::LineString(ls) => {
                    ls.points().iter().map(PyDirectPosition::from).collect()
                }
            })
            .collect()
    }

    pub fn compute_envelope(&self) -> PyEnvelope {
        PyEnvelope::from(self.inner.compute_envelope())
    }

    pub fn __repr__(&self) -> String {
        format!("MultiCurve({} curves)", self.inner.curve_member().len())
    }
}
