// ----------------------------------------------------------------------------
// Copyright (c) Treble.ai
//
// Licensed under the terms of the MIT License
// (see LICENSE.txt for details)
// ----------------------------------------------------------------------------

/// Python bindings for Haskell's Duckling library written in Rust
// PyO3 imports
// use pyo3::class::PyMappingProtocol;
use pyo3::create_exception;
use pyo3::exceptions;
use pyo3::gc::{PyGCProtocol, PyVisit};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::PyTraverseError;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;

// Package version
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// Haskell runtime status
static START_ONCE: Once = Once::new();
static STOP_ONCE: Once = Once::new();
static STOPPED: AtomicBool = AtomicBool::new(false);

pub enum HaskellValue {}

extern "C" {
    pub fn wcurrentReftime(tzdb: *mut HaskellValue, strPtr: *const c_char)
        -> *mut HaskellValue;
    pub fn wloadTimeZoneSeries(path: *const c_char) -> *mut HaskellValue;
    pub fn stringCreate(s: *const c_char) -> *mut HaskellValue;
    pub fn stringDestroy(s: *mut HaskellValue);
    pub fn stringGet(s: *mut HaskellValue) -> *const c_char;
    pub fn tzdbDestroy(db: *mut HaskellValue);
    pub fn duckTimeDestroy(time: *mut HaskellValue);
    pub fn hs_init(argc: c_int, argv: *const *const c_char);
    pub fn hs_exit();
}

create_exception!(pyduckling, RuntimeStoppedError, exceptions::Exception);

/// Initialize the Haskell runtime. This function is safe to call more than once, and
/// will do nothing on subsequent calls.
///
/// The runtime will automatically be shutdown at program exit, or you can stop it
/// earlier with `stop`.
#[pyfunction]
fn init() -> PyResult<()> {
    START_ONCE.call_once(|| {
        start_hs();
        unsafe {
            ::libc::atexit(stop_hs);
        }
    });
    Ok(())
}

/// Stop the Haskell runtime before the program exits. This function may only be called
/// once during a program's execution.
///
/// It is safe, but not useful, to call this before the runtime has started.
///
/// Raises
/// ------
/// RuntimeStoppedError:
///     If the runtime was already stopped.
#[pyfunction]
pub fn stop() -> PyResult<()> {
    if STOPPED.swap(true, Ordering::SeqCst) {
        let err = "Haskell: The GHC runtime may only be stopped once. See \
      https://downloads.haskell.org/%7Eghc/latest/docs/html/users_guide\
      /ffi-chap.html#id1";
      let exc = RuntimeStoppedError::py_err(err.to_string());
      return Err(exc);
    }
    stop_hs();
    Ok(())
}


fn start_hs() {
    let mut argv = Vec::<*const c_char>::with_capacity(1);
    argv.push(ptr::null_mut());
    unsafe {
        hs_init(0 as c_int, argv.as_ptr());
    }
}

extern "C" fn stop_hs() {
    STOP_ONCE.call_once(|| unsafe { hs_exit() });
}

/// Wraps a string into a C pointer valid from Haskell.
///
/// This class contains a reference to a string.
#[pyclass(name=DucklingString)]
// #[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrappedString {
    ptr: *mut HaskellValue,
}

#[pymethods]
impl WrappedString {
    #[new]
    pub fn new(this_str: &str) -> Self {
        let c_str = CString::new(this_str).expect("CString::new failed");
        WrappedString {
            ptr: unsafe { stringCreate(c_str.as_ptr()) },
        }
    }

    /// Retrieve the string wrapped
    #[getter]
    pub fn value(&self) -> PyResult<&str> {
        let c_str = unsafe { stringGet(self.ptr) };
        let rust_c_str = unsafe {
            CStr::from_ptr(c_str)
                .to_str()
                .expect("Error encoding string to UTF-8")
        };
        Ok(rust_c_str)
    }
}

#[pyproto]
impl PyGCProtocol for WrappedString {
    fn __traverse__(&self, _visit: PyVisit) -> Result<(), PyTraverseError> {
        Ok(())
    }

    fn __clear__(&mut self) {
        unsafe { stringDestroy(self.ptr) }
    }
}

impl Drop for WrappedString {
    fn drop(&mut self) {
        unsafe {
            stringDestroy(self.ptr);
        }
    }
}

/// Handle to the time zone database stored by Duckling
#[pyclass(name=TimeZoneDatabase)]
#[derive(Debug, Clone)]
pub struct TimeZoneDatabaseWrapper {
    ptr: *mut HaskellValue,
}

#[pyproto]
impl PyGCProtocol for TimeZoneDatabaseWrapper {
    fn __traverse__(&self, _visit: PyVisit) -> Result<(), PyTraverseError> {
        Ok(())
    }

    fn __clear__(&mut self) {
        unsafe { tzdbDestroy(self.ptr) }
    }
}

// impl Drop for TimeZoneDatabaseWrapper {
//     fn drop(&mut self) {
//         println!("Calling GC");
//         unsafe {
//             tzdbDestroy(self.ptr);
//         }
//     }
// }

/// Handle to the time zone database stored by Duckling
#[pyclass(name=DucklingTime)]
pub struct DucklingTimeWrapper {
    ptr: *mut HaskellValue,
}

#[pyproto]
impl PyGCProtocol for DucklingTimeWrapper {
    fn __traverse__(&self, _visit: PyVisit) -> Result<(), PyTraverseError> {
        Ok(())
    }

    fn __clear__(&mut self) {
        unsafe { duckTimeDestroy(self.ptr) }
    }
}

// impl Drop for DucklingTimeWrapper {
//     fn drop(&mut self) {
//         unsafe {
//             duckTimeDestroy(self.ptr);
//         }
//     }
// }

/// Load time zone information from local Olson files.
///
/// Parameters
/// ----------
/// path: str
///     Path to the olson data definitions. Many linux distros have
///     Olson data in "/usr/share/zoneinfo/".
///
/// Returns
/// -------
/// tz_info:
///     Opaque handle to a map of time zone data information in Haskell.
#[pyfunction]
fn load_time_zones(path: &str) -> PyResult<TimeZoneDatabaseWrapper> {
    println!("{}", path);
    // let c_str = WrappedString::new(path);
    let c_str = CString::new(path).expect("CString::new failed");
    let haskell_ptr = unsafe { wloadTimeZoneSeries(c_str.as_ptr()) };
    let result = TimeZoneDatabaseWrapper { ptr: haskell_ptr };
    Ok(result)
}

#[pyfunction]
fn get_current_ref_time(tz_db: TimeZoneDatabaseWrapper, tz: &str) -> PyResult<DucklingTimeWrapper> {
    // let c_str = WrappedString::new(tz);
    println!("Timezone: {}", tz);
    let tz_c_str = CString::new(tz).expect("CString::new failed");
    let haskell_tz = unsafe { wcurrentReftime(tz_db.ptr, tz_c_str.as_ptr()) };
    println!("Haskell call successful!");
    let result = DucklingTimeWrapper { ptr: haskell_tz };
    Ok(result)
}


/// This module is a python module implemented in Rust.
#[pymodule]
fn duckling(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", VERSION)?;
    m.add_wrapped(wrap_pyfunction!(load_time_zones))?;
    m.add_wrapped(wrap_pyfunction!(get_current_ref_time))?;
    m.add_wrapped(wrap_pyfunction!(init))?;
    m.add_wrapped(wrap_pyfunction!(stop))?;
    m.add_class::<WrappedString>()?;
    m.add_class::<TimeZoneDatabaseWrapper>()?;
    m.add_class::<DucklingTimeWrapper>()?;
    Ok(())
}
