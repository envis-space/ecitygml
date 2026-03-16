use crate::enums::PyCityObjectClass;
use crate::geometry::{PyDirectPosition, PyEnvelope, PyMultiCurve, PyMultiSurface, PySolid, PyTriangulatedSurface};
use ecitygml_rs::model::building::Building as RustBuilding;
use ecitygml_rs::model::city_furniture::CityFurniture as RustCityFurniture;
use ecitygml_rs::model::construction::{
    DoorSurface as RustDoorSurface, GroundSurface as RustGroundSurface,
    RoofSurface as RustRoofSurface, WallSurface as RustWallSurface,
    WindowSurface as RustWindowSurface,
};
use ecitygml_rs::model::core::{AsAbstractFeature, AsAbstractOccupiedSpace, AsAbstractSpace, AsAbstractThematicSurface};
use ecitygml_rs::model::building::{AsAbstractBuilding};
use ecitygml_rs::model::relief::{ReliefComponentKind, ReliefFeature as RustReliefFeature, TinRelief as RustTinRelief};
use ecitygml_rs::model::transportation::{
    AuxiliaryTrafficArea as RustAuxiliaryTrafficArea,
    AuxiliaryTrafficSpace as RustAuxiliaryTrafficSpace,
    Intersection as RustIntersection, Road as RustRoad, Section as RustSection,
    TrafficArea as RustTrafficArea, TrafficSpace as RustTrafficSpace,
};
use ecitygml_rs::model::vegetation::SolitaryVegetationObject as RustSolitaryVegetationObject;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

// ---------------------------------------------------------------------------
// Helper macros
// ---------------------------------------------------------------------------

macro_rules! py_id {
    ($self:ident) => {
        $self.inner.id().to_string()
    };
}

macro_rules! py_bounded_by {
    ($self:ident) => {
        $self.inner.bounded_by().map(PyEnvelope::from)
    };
}

macro_rules! py_city_object_class {
    ($variant:expr) => {
        PyCityObjectClass::from($variant)
    };
}

macro_rules! py_lod1_solid {
    ($self:ident) => {
        $self.inner.lod1_solid().map(PySolid::from)
    };
}

macro_rules! py_lod2_solid {
    ($self:ident) => {
        $self.inner.lod2_solid().map(PySolid::from)
    };
}

macro_rules! py_lod3_solid {
    ($self:ident) => {
        $self.inner.lod3_solid().map(PySolid::from)
    };
}

macro_rules! py_lod0_multi_surface {
    ($self:ident) => {
        $self.inner.lod0_multi_surface().map(PyMultiSurface::from)
    };
}

macro_rules! py_lod2_multi_surface {
    ($self:ident) => {
        $self.inner.lod2_multi_surface().map(PyMultiSurface::from)
    };
}

macro_rules! py_lod3_multi_surface {
    ($self:ident) => {
        $self.inner.lod3_multi_surface().map(PyMultiSurface::from)
    };
}

macro_rules! py_lod0_multi_curve {
    ($self:ident) => {
        $self.inner.lod0_multi_curve().map(PyMultiCurve::from)
    };
}

macro_rules! py_lod2_multi_curve {
    ($self:ident) => {
        $self.inner.lod2_multi_curve().map(PyMultiCurve::from)
    };
}

// ---------------------------------------------------------------------------
// Thematic surface common methods (lod0/2/3 multi_surface via trait)
// ---------------------------------------------------------------------------

macro_rules! py_ts_lod0_multi_surface {
    ($self:ident) => {
        $self.inner.lod0_multi_surface().map(PyMultiSurface::from)
    };
}

macro_rules! py_ts_lod2_multi_surface {
    ($self:ident) => {
        $self.inner.lod2_multi_surface().map(PyMultiSurface::from)
    };
}

macro_rules! py_ts_lod3_multi_surface {
    ($self:ident) => {
        $self.inner.lod3_multi_surface().map(PyMultiSurface::from)
    };
}

// ---------------------------------------------------------------------------
// DoorSurface
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "DoorSurface", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyDoorSurface {
    pub inner: RustDoorSurface,
}

impl From<RustDoorSurface> for PyDoorSurface {
    fn from(inner: RustDoorSurface) -> Self {
        Self { inner }
    }
}

impl From<&RustDoorSurface> for PyDoorSurface {
    fn from(inner: &RustDoorSurface) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyDoorSurface {
    #[getter]
    pub fn id(&self) -> String { py_id!(self) }

    #[getter]
    pub fn bounded_by(&self) -> Option<PyEnvelope> { py_bounded_by!(self) }

    #[getter]
    pub fn city_object_class(&self) -> PyCityObjectClass {
        py_city_object_class!(ecitygml_rs::model::common::CityObjectClass::DoorSurface)
    }

    #[getter]
    pub fn lod0_multi_surface(&self) -> Option<PyMultiSurface> { py_ts_lod0_multi_surface!(self) }

    #[getter]
    pub fn lod2_multi_surface(&self) -> Option<PyMultiSurface> { py_ts_lod2_multi_surface!(self) }

    #[getter]
    pub fn lod3_multi_surface(&self) -> Option<PyMultiSurface> { py_ts_lod3_multi_surface!(self) }

    pub fn __repr__(&self) -> String {
        format!("DoorSurface(id='{}')", py_id!(self))
    }
}

// ---------------------------------------------------------------------------
// WindowSurface
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "WindowSurface", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyWindowSurface {
    pub inner: RustWindowSurface,
}

impl From<RustWindowSurface> for PyWindowSurface {
    fn from(inner: RustWindowSurface) -> Self {
        Self { inner }
    }
}

impl From<&RustWindowSurface> for PyWindowSurface {
    fn from(inner: &RustWindowSurface) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyWindowSurface {
    #[getter]
    pub fn id(&self) -> String { py_id!(self) }

    #[getter]
    pub fn bounded_by(&self) -> Option<PyEnvelope> { py_bounded_by!(self) }

    #[getter]
    pub fn city_object_class(&self) -> PyCityObjectClass {
        py_city_object_class!(ecitygml_rs::model::common::CityObjectClass::WindowSurface)
    }

    #[getter]
    pub fn lod0_multi_surface(&self) -> Option<PyMultiSurface> { py_ts_lod0_multi_surface!(self) }

    #[getter]
    pub fn lod2_multi_surface(&self) -> Option<PyMultiSurface> { py_ts_lod2_multi_surface!(self) }

    #[getter]
    pub fn lod3_multi_surface(&self) -> Option<PyMultiSurface> { py_ts_lod3_multi_surface!(self) }

    pub fn __repr__(&self) -> String {
        format!("WindowSurface(id='{}')", py_id!(self))
    }
}

// ---------------------------------------------------------------------------
// WallSurface
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "WallSurface", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyWallSurface {
    pub inner: RustWallSurface,
}

impl From<RustWallSurface> for PyWallSurface {
    fn from(inner: RustWallSurface) -> Self {
        Self { inner }
    }
}

impl From<&RustWallSurface> for PyWallSurface {
    fn from(inner: &RustWallSurface) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyWallSurface {
    #[getter]
    pub fn id(&self) -> String { py_id!(self) }

    #[getter]
    pub fn bounded_by(&self) -> Option<PyEnvelope> { py_bounded_by!(self) }

    #[getter]
    pub fn city_object_class(&self) -> PyCityObjectClass {
        py_city_object_class!(ecitygml_rs::model::common::CityObjectClass::WallSurface)
    }

    #[getter]
    pub fn lod0_multi_surface(&self) -> Option<PyMultiSurface> { py_ts_lod0_multi_surface!(self) }

    #[getter]
    pub fn lod2_multi_surface(&self) -> Option<PyMultiSurface> { py_ts_lod2_multi_surface!(self) }

    #[getter]
    pub fn lod3_multi_surface(&self) -> Option<PyMultiSurface> { py_ts_lod3_multi_surface!(self) }

    #[getter]
    pub fn door_surface(&self) -> Vec<PyDoorSurface> {
        self.inner.door_surface.iter().map(PyDoorSurface::from).collect()
    }

    #[getter]
    pub fn window_surface(&self) -> Vec<PyWindowSurface> {
        self.inner.window_surface.iter().map(PyWindowSurface::from).collect()
    }

    pub fn __repr__(&self) -> String {
        format!("WallSurface(id='{}')", py_id!(self))
    }
}

// ---------------------------------------------------------------------------
// RoofSurface
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "RoofSurface", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyRoofSurface {
    pub inner: RustRoofSurface,
}

impl From<RustRoofSurface> for PyRoofSurface {
    fn from(inner: RustRoofSurface) -> Self {
        Self { inner }
    }
}

impl From<&RustRoofSurface> for PyRoofSurface {
    fn from(inner: &RustRoofSurface) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyRoofSurface {
    #[getter]
    pub fn id(&self) -> String { py_id!(self) }

    #[getter]
    pub fn bounded_by(&self) -> Option<PyEnvelope> { py_bounded_by!(self) }

    #[getter]
    pub fn city_object_class(&self) -> PyCityObjectClass {
        py_city_object_class!(ecitygml_rs::model::common::CityObjectClass::RoofSurface)
    }

    #[getter]
    pub fn lod0_multi_surface(&self) -> Option<PyMultiSurface> { py_ts_lod0_multi_surface!(self) }

    #[getter]
    pub fn lod2_multi_surface(&self) -> Option<PyMultiSurface> { py_ts_lod2_multi_surface!(self) }

    #[getter]
    pub fn lod3_multi_surface(&self) -> Option<PyMultiSurface> { py_ts_lod3_multi_surface!(self) }

    pub fn __repr__(&self) -> String {
        format!("RoofSurface(id='{}')", py_id!(self))
    }
}

// ---------------------------------------------------------------------------
// GroundSurface
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "GroundSurface", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyGroundSurface {
    pub inner: RustGroundSurface,
}

impl From<RustGroundSurface> for PyGroundSurface {
    fn from(inner: RustGroundSurface) -> Self {
        Self { inner }
    }
}

impl From<&RustGroundSurface> for PyGroundSurface {
    fn from(inner: &RustGroundSurface) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyGroundSurface {
    #[getter]
    pub fn id(&self) -> String { py_id!(self) }

    #[getter]
    pub fn bounded_by(&self) -> Option<PyEnvelope> { py_bounded_by!(self) }

    #[getter]
    pub fn city_object_class(&self) -> PyCityObjectClass {
        py_city_object_class!(ecitygml_rs::model::common::CityObjectClass::GroundSurface)
    }

    #[getter]
    pub fn lod0_multi_surface(&self) -> Option<PyMultiSurface> { py_ts_lod0_multi_surface!(self) }

    #[getter]
    pub fn lod2_multi_surface(&self) -> Option<PyMultiSurface> { py_ts_lod2_multi_surface!(self) }

    #[getter]
    pub fn lod3_multi_surface(&self) -> Option<PyMultiSurface> { py_ts_lod3_multi_surface!(self) }

    pub fn __repr__(&self) -> String {
        format!("GroundSurface(id='{}')", py_id!(self))
    }
}

// ---------------------------------------------------------------------------
// Building
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "Building", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyBuilding {
    pub inner: RustBuilding,
}

impl From<RustBuilding> for PyBuilding {
    fn from(inner: RustBuilding) -> Self {
        Self { inner }
    }
}

impl From<&RustBuilding> for PyBuilding {
    fn from(inner: &RustBuilding) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyBuilding {
    #[getter]
    pub fn id(&self) -> String { py_id!(self) }

    #[getter]
    pub fn bounded_by(&self) -> Option<PyEnvelope> { py_bounded_by!(self) }

    #[getter]
    pub fn city_object_class(&self) -> PyCityObjectClass {
        py_city_object_class!(ecitygml_rs::model::common::CityObjectClass::Building)
    }

    #[getter]
    pub fn storeys_above_ground(&self) -> Option<i64> {
        self.inner.storeys_above_ground()
    }

    #[getter]
    pub fn storeys_below_ground(&self) -> Option<i64> {
        self.inner.storeys_below_ground()
    }

    #[getter]
    pub fn lod1_solid(&self) -> Option<PySolid> { py_lod1_solid!(self) }

    #[getter]
    pub fn lod2_solid(&self) -> Option<PySolid> { py_lod2_solid!(self) }

    #[getter]
    pub fn lod3_solid(&self) -> Option<PySolid> { py_lod3_solid!(self) }

    #[getter]
    pub fn lod0_multi_surface(&self) -> Option<PyMultiSurface> { py_lod0_multi_surface!(self) }

    #[getter]
    pub fn lod2_multi_surface(&self) -> Option<PyMultiSurface> { py_lod2_multi_surface!(self) }

    #[getter]
    pub fn lod3_multi_surface(&self) -> Option<PyMultiSurface> { py_lod3_multi_surface!(self) }

    #[getter]
    pub fn wall_surface(&self) -> Vec<PyWallSurface> {
        self.inner.wall_surface.iter().map(PyWallSurface::from).collect()
    }

    #[getter]
    pub fn roof_surface(&self) -> Vec<PyRoofSurface> {
        self.inner.roof_surface.iter().map(PyRoofSurface::from).collect()
    }

    #[getter]
    pub fn ground_surface(&self) -> Vec<PyGroundSurface> {
        self.inner.ground_surface.iter().map(PyGroundSurface::from).collect()
    }

    pub fn __repr__(&self) -> String {
        format!("Building(id='{}')", py_id!(self))
    }
}

// ---------------------------------------------------------------------------
// TrafficArea
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "TrafficArea", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyTrafficArea {
    pub inner: RustTrafficArea,
}

impl From<RustTrafficArea> for PyTrafficArea {
    fn from(inner: RustTrafficArea) -> Self {
        Self { inner }
    }
}

impl From<&RustTrafficArea> for PyTrafficArea {
    fn from(inner: &RustTrafficArea) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyTrafficArea {
    #[getter]
    pub fn id(&self) -> String { py_id!(self) }

    #[getter]
    pub fn bounded_by(&self) -> Option<PyEnvelope> { py_bounded_by!(self) }

    #[getter]
    pub fn city_object_class(&self) -> PyCityObjectClass {
        py_city_object_class!(ecitygml_rs::model::common::CityObjectClass::TrafficArea)
    }

    #[getter]
    pub fn lod2_multi_surface(&self) -> Option<PyMultiSurface> { py_ts_lod2_multi_surface!(self) }

    pub fn __repr__(&self) -> String {
        format!("TrafficArea(id='{}')", py_id!(self))
    }
}

// ---------------------------------------------------------------------------
// AuxiliaryTrafficArea
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "AuxiliaryTrafficArea", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyAuxiliaryTrafficArea {
    pub inner: RustAuxiliaryTrafficArea,
}

impl From<RustAuxiliaryTrafficArea> for PyAuxiliaryTrafficArea {
    fn from(inner: RustAuxiliaryTrafficArea) -> Self {
        Self { inner }
    }
}

impl From<&RustAuxiliaryTrafficArea> for PyAuxiliaryTrafficArea {
    fn from(inner: &RustAuxiliaryTrafficArea) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyAuxiliaryTrafficArea {
    #[getter]
    pub fn id(&self) -> String { py_id!(self) }

    #[getter]
    pub fn bounded_by(&self) -> Option<PyEnvelope> { py_bounded_by!(self) }

    #[getter]
    pub fn city_object_class(&self) -> PyCityObjectClass {
        py_city_object_class!(ecitygml_rs::model::common::CityObjectClass::AuxiliaryTrafficArea)
    }

    #[getter]
    pub fn lod2_multi_surface(&self) -> Option<PyMultiSurface> { py_ts_lod2_multi_surface!(self) }

    pub fn __repr__(&self) -> String {
        format!("AuxiliaryTrafficArea(id='{}')", py_id!(self))
    }
}

// ---------------------------------------------------------------------------
// TrafficSpace
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "TrafficSpace", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyTrafficSpace {
    pub inner: RustTrafficSpace,
}

impl From<RustTrafficSpace> for PyTrafficSpace {
    fn from(inner: RustTrafficSpace) -> Self {
        Self { inner }
    }
}

impl From<&RustTrafficSpace> for PyTrafficSpace {
    fn from(inner: &RustTrafficSpace) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyTrafficSpace {
    #[getter]
    pub fn id(&self) -> String { py_id!(self) }

    #[getter]
    pub fn bounded_by(&self) -> Option<PyEnvelope> { py_bounded_by!(self) }

    #[getter]
    pub fn city_object_class(&self) -> PyCityObjectClass {
        py_city_object_class!(ecitygml_rs::model::common::CityObjectClass::TrafficSpace)
    }

    #[getter]
    pub fn lod2_multi_surface(&self) -> Option<PyMultiSurface> { py_lod2_multi_surface!(self) }

    #[getter]
    pub fn lod2_multi_curve(&self) -> Option<PyMultiCurve> { py_lod2_multi_curve!(self) }

    #[getter]
    pub fn traffic_area(&self) -> Vec<PyTrafficArea> {
        self.inner.traffic_area.iter().map(PyTrafficArea::from).collect()
    }

    pub fn __repr__(&self) -> String {
        format!("TrafficSpace(id='{}')", py_id!(self))
    }
}

// ---------------------------------------------------------------------------
// AuxiliaryTrafficSpace
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "AuxiliaryTrafficSpace", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyAuxiliaryTrafficSpace {
    pub inner: RustAuxiliaryTrafficSpace,
}

impl From<RustAuxiliaryTrafficSpace> for PyAuxiliaryTrafficSpace {
    fn from(inner: RustAuxiliaryTrafficSpace) -> Self {
        Self { inner }
    }
}

impl From<&RustAuxiliaryTrafficSpace> for PyAuxiliaryTrafficSpace {
    fn from(inner: &RustAuxiliaryTrafficSpace) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyAuxiliaryTrafficSpace {
    #[getter]
    pub fn id(&self) -> String { py_id!(self) }

    #[getter]
    pub fn bounded_by(&self) -> Option<PyEnvelope> { py_bounded_by!(self) }

    #[getter]
    pub fn city_object_class(&self) -> PyCityObjectClass {
        py_city_object_class!(ecitygml_rs::model::common::CityObjectClass::AuxiliaryTrafficSpace)
    }

    #[getter]
    pub fn lod2_multi_surface(&self) -> Option<PyMultiSurface> { py_lod2_multi_surface!(self) }

    #[getter]
    pub fn lod2_multi_curve(&self) -> Option<PyMultiCurve> { py_lod2_multi_curve!(self) }

    #[getter]
    pub fn auxiliary_traffic_area(&self) -> Vec<PyAuxiliaryTrafficArea> {
        self.inner.auxiliary_traffic_area.iter().map(PyAuxiliaryTrafficArea::from).collect()
    }

    pub fn __repr__(&self) -> String {
        format!("AuxiliaryTrafficSpace(id='{}')", py_id!(self))
    }
}

// ---------------------------------------------------------------------------
// Section
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "Section", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PySection {
    pub inner: RustSection,
}

impl From<RustSection> for PySection {
    fn from(inner: RustSection) -> Self {
        Self { inner }
    }
}

impl From<&RustSection> for PySection {
    fn from(inner: &RustSection) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PySection {
    #[getter]
    pub fn id(&self) -> String { py_id!(self) }

    #[getter]
    pub fn bounded_by(&self) -> Option<PyEnvelope> { py_bounded_by!(self) }

    #[getter]
    pub fn city_object_class(&self) -> PyCityObjectClass {
        py_city_object_class!(ecitygml_rs::model::common::CityObjectClass::Section)
    }

    #[getter]
    pub fn lod0_multi_curve(&self) -> Option<PyMultiCurve> { py_lod0_multi_curve!(self) }

    #[getter]
    pub fn lod0_multi_surface(&self) -> Option<PyMultiSurface> { py_lod0_multi_surface!(self) }

    #[getter]
    pub fn lod1_solid(&self) -> Option<PySolid> { py_lod1_solid!(self) }

    #[getter]
    pub fn traffic_space(&self) -> Vec<PyTrafficSpace> {
        self.inner.traffic_space.iter().map(PyTrafficSpace::from).collect()
    }

    #[getter]
    pub fn auxiliary_traffic_space(&self) -> Vec<PyAuxiliaryTrafficSpace> {
        self.inner.auxiliary_traffic_space.iter().map(PyAuxiliaryTrafficSpace::from).collect()
    }

    pub fn __repr__(&self) -> String {
        format!("Section(id='{}')", py_id!(self))
    }
}

// ---------------------------------------------------------------------------
// Intersection
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "Intersection", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyIntersection {
    pub inner: RustIntersection,
}

impl From<RustIntersection> for PyIntersection {
    fn from(inner: RustIntersection) -> Self {
        Self { inner }
    }
}

impl From<&RustIntersection> for PyIntersection {
    fn from(inner: &RustIntersection) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyIntersection {
    #[getter]
    pub fn id(&self) -> String { py_id!(self) }

    #[getter]
    pub fn bounded_by(&self) -> Option<PyEnvelope> { py_bounded_by!(self) }

    #[getter]
    pub fn city_object_class(&self) -> PyCityObjectClass {
        py_city_object_class!(ecitygml_rs::model::common::CityObjectClass::Intersection)
    }

    #[getter]
    pub fn lod0_multi_curve(&self) -> Option<PyMultiCurve> { py_lod0_multi_curve!(self) }

    #[getter]
    pub fn lod0_multi_surface(&self) -> Option<PyMultiSurface> { py_lod0_multi_surface!(self) }

    #[getter]
    pub fn lod1_solid(&self) -> Option<PySolid> { py_lod1_solid!(self) }

    #[getter]
    pub fn traffic_space(&self) -> Vec<PyTrafficSpace> {
        self.inner.traffic_space.iter().map(PyTrafficSpace::from).collect()
    }

    #[getter]
    pub fn auxiliary_traffic_space(&self) -> Vec<PyAuxiliaryTrafficSpace> {
        self.inner.auxiliary_traffic_space.iter().map(PyAuxiliaryTrafficSpace::from).collect()
    }

    pub fn __repr__(&self) -> String {
        format!("Intersection(id='{}')", py_id!(self))
    }
}

// ---------------------------------------------------------------------------
// Road
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "Road", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyRoad {
    pub inner: RustRoad,
}

impl From<RustRoad> for PyRoad {
    fn from(inner: RustRoad) -> Self {
        Self { inner }
    }
}

impl From<&RustRoad> for PyRoad {
    fn from(inner: &RustRoad) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyRoad {
    #[getter]
    pub fn id(&self) -> String { py_id!(self) }

    #[getter]
    pub fn bounded_by(&self) -> Option<PyEnvelope> { py_bounded_by!(self) }

    #[getter]
    pub fn city_object_class(&self) -> PyCityObjectClass {
        py_city_object_class!(ecitygml_rs::model::common::CityObjectClass::Road)
    }

    #[getter]
    pub fn section(&self) -> Vec<PySection> {
        self.inner.section.iter().map(PySection::from).collect()
    }

    #[getter]
    pub fn intersection(&self) -> Vec<PyIntersection> {
        self.inner.intersection.iter().map(PyIntersection::from).collect()
    }

    pub fn __repr__(&self) -> String {
        format!("Road(id='{}')", py_id!(self))
    }
}

// ---------------------------------------------------------------------------
// TinRelief
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "TinRelief", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyTinRelief {
    pub inner: RustTinRelief,
}

impl From<RustTinRelief> for PyTinRelief {
    fn from(inner: RustTinRelief) -> Self {
        Self { inner }
    }
}

impl From<&RustTinRelief> for PyTinRelief {
    fn from(inner: &RustTinRelief) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyTinRelief {
    #[getter]
    pub fn id(&self) -> String { py_id!(self) }

    #[getter]
    pub fn bounded_by(&self) -> Option<PyEnvelope> { py_bounded_by!(self) }

    #[getter]
    pub fn city_object_class(&self) -> PyCityObjectClass {
        py_city_object_class!(ecitygml_rs::model::common::CityObjectClass::TinRelief)
    }

    #[getter]
    pub fn tin(&self) -> PyTriangulatedSurface {
        PyTriangulatedSurface::from(self.inner.tin().clone())
    }

    pub fn __repr__(&self) -> String {
        format!("TinRelief(id='{}')", py_id!(self))
    }
}

// ---------------------------------------------------------------------------
// ReliefFeature
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "ReliefFeature", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyReliefFeature {
    pub inner: RustReliefFeature,
}

impl From<RustReliefFeature> for PyReliefFeature {
    fn from(inner: RustReliefFeature) -> Self {
        Self { inner }
    }
}

impl From<&RustReliefFeature> for PyReliefFeature {
    fn from(inner: &RustReliefFeature) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyReliefFeature {
    #[getter]
    pub fn id(&self) -> String { py_id!(self) }

    #[getter]
    pub fn bounded_by(&self) -> Option<PyEnvelope> { py_bounded_by!(self) }

    #[getter]
    pub fn city_object_class(&self) -> PyCityObjectClass {
        py_city_object_class!(ecitygml_rs::model::common::CityObjectClass::ReliefFeature)
    }

    #[getter]
    pub fn relief_component(&self) -> Vec<PyTinRelief> {
        self.inner
            .relief_component()
            .iter()
            .filter_map(|c| match c {
                ReliefComponentKind::TinRelief(t) => Some(PyTinRelief::from(t)),
            })
            .collect()
    }

    pub fn __repr__(&self) -> String {
        format!("ReliefFeature(id='{}')", py_id!(self))
    }
}

// ---------------------------------------------------------------------------
// SolitaryVegetationObject
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "SolitaryVegetationObject", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PySolitaryVegetationObject {
    pub inner: RustSolitaryVegetationObject,
}

impl From<RustSolitaryVegetationObject> for PySolitaryVegetationObject {
    fn from(inner: RustSolitaryVegetationObject) -> Self {
        Self { inner }
    }
}

impl From<&RustSolitaryVegetationObject> for PySolitaryVegetationObject {
    fn from(inner: &RustSolitaryVegetationObject) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PySolitaryVegetationObject {
    #[getter]
    pub fn id(&self) -> String { py_id!(self) }

    #[getter]
    pub fn bounded_by(&self) -> Option<PyEnvelope> { py_bounded_by!(self) }

    #[getter]
    pub fn city_object_class(&self) -> PyCityObjectClass {
        py_city_object_class!(ecitygml_rs::model::common::CityObjectClass::SolitaryVegetationObject)
    }

    #[getter]
    pub fn lod1_solid(&self) -> Option<PySolid> { py_lod1_solid!(self) }

    #[getter]
    pub fn lod2_solid(&self) -> Option<PySolid> { py_lod2_solid!(self) }

    #[getter]
    pub fn lod0_multi_surface(&self) -> Option<PyMultiSurface> { py_lod0_multi_surface!(self) }

    #[getter]
    pub fn lod1_implicit_representation(&self) -> Option<PyDirectPosition> {
        self.inner
            .lod1_implicit_representation()
            .map(|ig| PyDirectPosition::from(ig.reference_point.pos()))
    }

    pub fn __repr__(&self) -> String {
        format!("SolitaryVegetationObject(id='{}')", py_id!(self))
    }
}

// ---------------------------------------------------------------------------
// CityFurniture
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "CityFurniture", frozen, skip_from_py_object)]
#[derive(Clone)]
pub struct PyCityFurniture {
    pub inner: RustCityFurniture,
}

impl From<RustCityFurniture> for PyCityFurniture {
    fn from(inner: RustCityFurniture) -> Self {
        Self { inner }
    }
}

impl From<&RustCityFurniture> for PyCityFurniture {
    fn from(inner: &RustCityFurniture) -> Self {
        Self { inner: inner.clone() }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyCityFurniture {
    #[getter]
    pub fn id(&self) -> String { py_id!(self) }

    #[getter]
    pub fn bounded_by(&self) -> Option<PyEnvelope> { py_bounded_by!(self) }

    #[getter]
    pub fn city_object_class(&self) -> PyCityObjectClass {
        py_city_object_class!(ecitygml_rs::model::common::CityObjectClass::CityFurniture)
    }

    #[getter]
    pub fn lod1_solid(&self) -> Option<PySolid> { py_lod1_solid!(self) }

    #[getter]
    pub fn lod2_solid(&self) -> Option<PySolid> { py_lod2_solid!(self) }

    #[getter]
    pub fn lod0_multi_surface(&self) -> Option<PyMultiSurface> { py_lod0_multi_surface!(self) }

    pub fn __repr__(&self) -> String {
        format!("CityFurniture(id='{}')", py_id!(self))
    }
}
