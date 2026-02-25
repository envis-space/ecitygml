use ecitygml::io;
use ecitygml::model::core;
use ecitygml::operations::{CityModelGeometryIndex, CityObjectGeometry};
use egml::model::base::Id;
use egml::model::geometry::Envelope;
use std::fs::File;
use std::io::Read;
use std::panic;
use std::path::PathBuf;

pub struct CCityModel {
    inner: Option<core::CityModel>,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn city_model_destroy(handle: *mut CCityModel) -> CErrorCode {
    if handle.is_null() {
        return CErrorCode::NULL_POINTER;
    }
    unsafe {
        drop(Box::from_raw(handle));
    }
    CErrorCode::OK
}

pub struct CCityModelGeometryIndex {
    inner: Option<CityModelGeometryIndex>,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn city_model_geometry_index_create(
    city_model: *mut CCityModel,
    out: *mut *mut CCityModelGeometryIndex,
) -> CErrorCode {
    unsafe {
        if city_model.is_null() || out.is_null() {
            return CErrorCode::NULL_POINTER;
        }

        let city_model = unsafe { &mut *city_model };

        match city_model.inner.take() {
            Some(city_model) => {
                *out = Box::into_raw(Box::new(CCityModelGeometryIndex {
                    inner: Some(CityModelGeometryIndex::from_city_model(city_model)),
                }));
                CErrorCode::OK
            }
            None => CErrorCode::INTERNAL_ERROR,
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn city_model_geometry_index_destroy(
    handle: *mut CCityModelGeometryIndex,
) -> CErrorCode {
    if handle.is_null() {
        return CErrorCode::NULL_POINTER;
    }
    unsafe {
        drop(Box::from_raw(handle));
    }
    CErrorCode::OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn city_model_geometry_index_objects_len(
    handle: *mut CCityModelGeometryIndex,
    out: *mut usize,
) -> CErrorCode {
    if handle.is_null() || out.is_null() {
        return CErrorCode::NULL_POINTER;
    }

    let city_model_geometry_index = unsafe { &*handle };

    match &city_model_geometry_index.inner {
        Some(city_model_geometry_index) => {
            unsafe {
                *out = city_model_geometry_index.objects_len();
            }
            CErrorCode::OK
        }
        None => CErrorCode::INTERNAL_ERROR,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn city_model_geometry_index_get_object_ids(
    handle: *mut CCityModelGeometryIndex,
    out_ptr: *mut *mut *mut libc::c_char,
    out_len: *mut usize,
) -> CErrorCode {
    if handle.is_null() || out_ptr.is_null() || out_len.is_null() {
        return CErrorCode::NULL_POINTER;
    }

    let city_model_geometry_index = unsafe { &*handle };

    match &city_model_geometry_index.inner {
        Some(index) => {
            let ids = index.object_ids();
            let len = ids.len();

            // Allocate array of C string pointers
            let mut c_strings: Vec<*mut libc::c_char> = ids
                .into_iter()
                .map(|s| std::ffi::CString::new(s).unwrap().into_raw())
                .collect();

            c_strings.shrink_to_fit();
            let ptr = c_strings.as_mut_ptr();

            unsafe {
                *out_ptr = ptr;
                *out_len = len;
            }

            std::mem::forget(c_strings);
            CErrorCode::OK
        }
        None => CErrorCode::INTERNAL_ERROR,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn city_model_geometry_index_free_object_ids(
    ptr: *mut *mut libc::c_char,
    len: usize,
) -> CErrorCode {
    if ptr.is_null() {
        return CErrorCode::NULL_POINTER;
    }

    unsafe {
        let strings = Vec::from_raw_parts(ptr, len, len);
        for c_str in strings {
            if !c_str.is_null() {
                drop(std::ffi::CString::from_raw(c_str));
            }
        }
    }

    CErrorCode::OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn city_model_geometry_index_get(
    handle: *mut CCityModelGeometryIndex,
    id: *const libc::c_char,
    out: *mut *mut CCityObjectGeometry,
) -> CErrorCode {
    unsafe {
        if handle.is_null() || out.is_null() {
            return CErrorCode::NULL_POINTER;
        }

        let city_model_geometry_index = unsafe { &*handle };
        let id_str = unsafe { std::ffi::CStr::from_ptr(id).to_str() }.unwrap();
        let id: Id = Id::try_from(id_str).unwrap();

        match &city_model_geometry_index.inner {
            Some(city_model_geometry_index) => {
                match city_model_geometry_index.get(&id) {
                    None => {
                        *out = std::ptr::null_mut();
                    }
                    Some(g) => {
                        *out = Box::into_raw(Box::new(CCityObjectGeometry {
                            inner: Some(g.clone()),
                        }));
                    }
                }
                CErrorCode::OK
            }
            None => {
                println!("Error: GeometryCollector is null");
                CErrorCode::INTERNAL_ERROR
            }
        }
    }
}

pub struct CCityObjectGeometry {
    inner: Option<CityObjectGeometry>,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn city_object_geometry_envelope(
    handle: *mut CCityObjectGeometry,
    out: *mut *mut CEnvelope,
) -> CErrorCode {
    unsafe {
        if handle.is_null() {
            return CErrorCode::NULL_POINTER;
        }

        let handle = unsafe { &*handle };

        match &handle.inner {
            Some(city_object_geometry) => {
                let envelope: Option<CEnvelope> = city_object_geometry
                    .envelope()
                    .cloned()
                    .map(CEnvelope::from);
                match envelope {
                    None => {
                        *out = std::ptr::null_mut();
                    }
                    Some(x) => {
                        *out = Box::into_raw(Box::new(x));
                    }
                }

                CErrorCode::OK
            }
            None => CErrorCode::INTERNAL_ERROR,
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn city_object_geometry_destroy(
    handle: *mut CCityObjectGeometry,
) -> CErrorCode {
    if handle.is_null() {
        return CErrorCode::NULL_POINTER;
    }
    unsafe {
        drop(Box::from_raw(handle));
    }
    CErrorCode::OK
}

pub struct CGmlReader {
    inner: Option<io::GmlReader<File>>,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gml_reader_create(
    file_path: *const libc::c_char,
    out: *mut *mut CGmlReader,
) -> CErrorCode {
    unsafe {
        if out.is_null() {
            return CErrorCode::NULL_POINTER;
        }

        let result = panic::catch_unwind(|| {
            let path = unsafe { std::ffi::CStr::from_ptr(file_path).to_str() }.unwrap();
            io::GmlReader::from_path(PathBuf::from(path)).unwrap()
        });

        match result {
            Ok(reader) => {
                *out = Box::into_raw(Box::new(CGmlReader {
                    inner: Some(reader),
                }));
                CErrorCode::OK
            }
            Err(_) => CErrorCode::INTERNAL_ERROR,
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gml_reader_destroy(handle: *mut CGmlReader) -> CErrorCode {
    if handle.is_null() {
        return CErrorCode::NULL_POINTER;
    }
    unsafe {
        drop(Box::from_raw(handle));
    }
    CErrorCode::OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gml_reader_finish(
    handle: *mut CGmlReader,
    out: *mut *mut CCityModel,
) -> CErrorCode {
    if handle.is_null() {
        return CErrorCode::NULL_POINTER;
    }
    let reader = unsafe { &mut *handle };

    let inner_reader = match reader.inner.take() {
        Some(r) => r,
        None => return CErrorCode::INTERNAL_ERROR,
    };

    match inner_reader.finish() {
        Ok(city_model) => {
            let model = CCityModel {
                inner: Some(city_model),
            };
            unsafe {
                *out = Box::into_raw(Box::new(model));
            }
        }
        Err(_) => {
            return CErrorCode::INTERNAL_ERROR;
        }
    }

    CErrorCode::OK
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CErrorCode {
    OK = 0,
    NULL_POINTER = 1,
    INTERNAL_ERROR = 255,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn city_model_objects_len(
    handle: *mut CCityModel,
    out: *mut usize,
) -> CErrorCode {
    if handle.is_null() || out.is_null() {
        return CErrorCode::NULL_POINTER;
    }

    let model = unsafe { &*handle };

    match &model.inner {
        Some(city_model) => {
            unsafe {
                *out = city_model.city_objects_len();
            }
            CErrorCode::OK
        }
        None => CErrorCode::INTERNAL_ERROR,
    }
}

pub struct CEnvelope {
    inner: Option<Envelope>,
}

impl From<Envelope> for CEnvelope {
    fn from(envelope: Envelope) -> Self {
        CEnvelope {
            inner: Some(envelope),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn envelope_size_x(handle: *mut CEnvelope, out: *mut f64) -> CErrorCode {
    if handle.is_null() || out.is_null() {
        return CErrorCode::NULL_POINTER;
    }

    let envelope = unsafe { &*handle };

    match &envelope.inner {
        Some(x) => {
            unsafe {
                *out = x.size_x();
            }
            CErrorCode::OK
        }
        None => CErrorCode::INTERNAL_ERROR,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn envelope_size_y(handle: *mut CEnvelope, out: *mut f64) -> CErrorCode {
    if handle.is_null() || out.is_null() {
        return CErrorCode::NULL_POINTER;
    }

    let envelope = unsafe { &*handle };

    match &envelope.inner {
        Some(x) => {
            unsafe {
                *out = x.size_y();
            }
            CErrorCode::OK
        }
        None => CErrorCode::INTERNAL_ERROR,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn envelope_size_z(handle: *mut CEnvelope, out: *mut f64) -> CErrorCode {
    if handle.is_null() || out.is_null() {
        return CErrorCode::NULL_POINTER;
    }

    let envelope = unsafe { &*handle };

    match &envelope.inner {
        Some(x) => {
            unsafe {
                *out = x.size_z();
            }
            CErrorCode::OK
        }
        None => CErrorCode::INTERNAL_ERROR,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn envelope_destroy(handle: *mut CEnvelope) -> CErrorCode {
    if handle.is_null() {
        return CErrorCode::NULL_POINTER;
    }
    unsafe {
        drop(Box::from_raw(handle));
    }
    CErrorCode::OK
}
