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
use std::slice;
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
    // ----------------- Duckling API -----------------------------------
    pub fn wparseText(
        text: *const c_char,
        reference_time: *mut HaskellValue,
        locale: *mut HaskellValue,
        dimensions: *mut HaskellValue,
        with_latent: u8,
    ) -> *const c_char;
    pub fn wparseDimensions(n: i32, dimensions: *const *const c_char) -> *mut HaskellValue;
    pub fn wparseLocale(
        locale: *const c_char,
        default_locale: *mut HaskellValue,
    ) -> *mut HaskellValue;
    pub fn wmakeDefaultLocale(lang: *mut HaskellValue) -> *mut HaskellValue;
    pub fn wparseLang(lang: *const c_char) -> *mut HaskellValue;
    pub fn wparseRefTime(
        tzdb: *mut HaskellValue,
        tzStr: *const c_char,
        timestamp: i64,
    ) -> *mut HaskellValue;
    pub fn wcurrentReftime(tzdb: *mut HaskellValue, strPtr: *const c_char) -> *mut HaskellValue;
    pub fn wloadTimeZoneSeries(path: *const c_char) -> *mut HaskellValue;
    // ----------------- Duckling API -----------------------------------
    // Dimension list functions
    pub fn dimensionListCreate(
        ptrs: *const *mut HaskellValue,
        numElements: i32,
    ) -> *mut HaskellValue;
    pub fn dimensionListLength(dims: *mut HaskellValue) -> i32;
    pub fn dimensionListPtrs(dims: *mut HaskellValue) -> *mut *mut HaskellValue;
    pub fn dimensionListDestroy(dims: *mut HaskellValue);
    // Dimension functions
    pub fn dimensionDestroy(dim: *mut HaskellValue);
    // Time zone database functions
    pub fn tzdbDestroy(db: *mut HaskellValue);
    // Time reference wrapper functions
    pub fn duckTimeDestroy(time: *mut HaskellValue);
    // Language wrapper functions
    pub fn langDestroy(lang: *mut HaskellValue);
    // Locale wrapper functions
    pub fn localeDestroy(locale: *mut HaskellValue);
    // Haskell runtime start/stop
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
#[derive(Debug, Clone)]
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

/// Handle to a language code stored by Duckling
#[pyclass(name=Language)]
#[derive(Debug, Clone)]
pub struct LanguageWrapper {
    ptr: *mut HaskellValue,
}

#[pyproto]
impl PyGCProtocol for LanguageWrapper {
    fn __traverse__(&self, _visit: PyVisit) -> Result<(), PyTraverseError> {
        Ok(())
    }

    fn __clear__(&mut self) {
        unsafe { langDestroy(self.ptr) }
    }
}

/// Handle to a locale code stored by Duckling
#[pyclass(name=Locale)]
#[derive(Debug, Clone)]
pub struct LocaleWrapper {
    ptr: *mut HaskellValue,
}

#[pyproto]
impl PyGCProtocol for LocaleWrapper {
    fn __traverse__(&self, _visit: PyVisit) -> Result<(), PyTraverseError> {
        Ok(())
    }

    fn __clear__(&mut self) {
        unsafe { localeDestroy(self.ptr) }
    }
}

/// Handle to a parsing dimension identifier
#[pyclass(name=Dimension)]
#[derive(Debug, Clone)]
pub struct DimensionWrapper {
    ptr: *mut HaskellValue,
}

#[pyproto]
impl PyGCProtocol for DimensionWrapper {
    fn __traverse__(&self, _visit: PyVisit) -> Result<(), PyTraverseError> {
        Ok(())
    }

    fn __clear__(&mut self) {
        unsafe { dimensionDestroy(self.ptr) }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Context {
    pub reference_time: DucklingTimeWrapper,
    pub locale: LocaleWrapper,
}

#[pymethods]
impl Context {
    #[new]
    fn new(reference_time: DucklingTimeWrapper, locale: LocaleWrapper) -> Self {
        Context {
            reference_time: reference_time,
            locale: locale,
        }
    }
}

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
/// tz_info: TimeZoneDatabase
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

/// Get current reference time, given a Olson time zone
///
/// Parameters
/// ----------
/// tz_db: TimeZoneDatabase
///     Opaque handle to a map of time zone data information in Haskell
/// tz: str
///     Time zone name according to IANA
///
/// Returns
/// -------
/// ref_time: DucklingTime
///     Opaque handle to a time reference in Haskell
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

/// Parse a reference timestamp on a given Olson time zone
///
/// Parameters
/// ----------
/// tz_db: TimeZoneDatabase
///     Opaque handle to a map of time zone data information in Haskell
/// tz: str
///     Time zone name according to IANA
/// timestamp: int
///     UNIX integer timestamp
///
/// Returns
/// -------
/// ref_time: DucklingTime
///     Opaque handle to a time reference in Haskell
#[pyfunction]
fn parse_ref_time(
    tz_db: TimeZoneDatabaseWrapper,
    tz: &str,
    timestamp: i64,
) -> PyResult<DucklingTimeWrapper> {
    let tz_c_str = CString::new(tz).expect("CString::new failed");
    let haskell_tz = unsafe { wparseRefTime(tz_db.ptr, tz_c_str.as_ptr(), timestamp) };
    let result = DucklingTimeWrapper { ptr: haskell_tz };
    Ok(result)
}

/// Parse an ISO-639-1 language code
///
/// Parameters
/// ----------
/// lang: str
///     ISO-639-1 code of the language to parse
///
/// Returns
/// -------
/// Language:
///     Opaque handle to a Haskell reference of the language. If the language
///     does not exist, or if it is not supported by Duckling,
///     it defaults to English (EN).
#[pyfunction]
fn parse_lang(lang: &str) -> PyResult<LanguageWrapper> {
    let lang_c_str = CString::new(lang).expect("CString::new failed");
    let haskell_lang = unsafe { wparseLang(lang_c_str.as_ptr()) };
    let result = LanguageWrapper { ptr: haskell_lang };
    Ok(result)
}

/// Retrieve the default locale for a given language
///
/// Parameters
/// ----------
/// lang: Language
///     Opaque handle to a Duckling language
///
/// Returns
/// -------
/// Locale:
///     Opaque handle to the default language locale
#[pyfunction]
fn default_locale_lang(lang: LanguageWrapper) -> PyResult<LocaleWrapper> {
    let haskell_locale = unsafe { wmakeDefaultLocale(lang.ptr) };
    let result = LocaleWrapper {
        ptr: haskell_locale,
    };
    Ok(result)
}

/// Parse an ISO3166 alpha2 country code into a locale
///
/// Parameters
/// ----------
/// locale: str
///     Locale identifier to parse, it can be either a country code or a language
///     with its country separated by underscore.
/// default_locale: Locale
///     Default locale to fallback on on case that the given code is not valid.
///
/// Returns
/// -------
/// Locale:
///     Opaque handle to the default language locale
#[pyfunction]
fn parse_locale(locale: &str, default_locale: LocaleWrapper) -> PyResult<LocaleWrapper> {
    let locale_c_str = CString::new(locale).expect("CString::new failed");
    let haskell_locale = unsafe { wparseLocale(locale_c_str.as_ptr(), default_locale.ptr) };
    let result = LocaleWrapper {
        ptr: haskell_locale,
    };
    Ok(result)
}

/// Parse a list of dimensions to use during parsing
///
/// Parameters
/// ----------
/// dims: List[str]
///     A list containing valid parsing dimensions to use with Duckling. See
///     :class:`DucklingDimensions` to see a list of valid dimensions to use.
///
/// Returns
/// -------
/// wrapped_dims: List[DimensionWrapper]
///     A list of opaque handlers that describe the given dimensions in Duckling.
#[pyfunction]
fn parse_dimensions(dims: Vec<String>) -> PyResult<Vec<DimensionWrapper>> {
    let n_elems = dims.len() as i32;

    // This is required in order to preserve ownership of the pointers
    let cstr_dims: Vec<CString> = dims
        .iter()
        .map(|s| CString::new(s.as_str()).expect("CString::new failed"))
        .collect();

    let c_dims: Vec<*const c_char> = cstr_dims.iter().map(|s| s.as_ptr()).collect();

    let haskell_list = unsafe { wparseDimensions(n_elems, c_dims.as_ptr()) };
    let haskell_length = unsafe { dimensionListLength(haskell_list) };
    let haskell_ptrs = unsafe { dimensionListPtrs(haskell_list) };
    let ptr_slice = unsafe { slice::from_raw_parts(haskell_ptrs, haskell_length as usize) };
    let mut result_vec: Vec<DimensionWrapper> = Vec::new();
    for ptr in ptr_slice {
        let wrapper = DimensionWrapper { ptr: *ptr };
        result_vec.push(wrapper);
    }
    Ok(result_vec)
}

/// Parse a text into a structured format
///
/// Parameters
/// ----------
/// text: str
///     Text to parse.
/// context: Context
///     Reference time and locale information
/// dimensions: List[Dimension]
///     List of dimensions to parse
/// with_latent: bool
///     When set, includes less certain parses, e.g. "7" as an hour of the day
///
/// Returns
/// -------
/// result: str
///     JSON-valid string that contains the parsed information.
#[pyfunction]
fn parse_text(
    text: &str,
    context: Context,
    dimensions: Vec<DimensionWrapper>,
    with_latent: bool,
) -> PyResult<String> {
    let c_text = CString::new(text).expect("CString::new failed");
    let reference_time = context.reference_time;
    let locale = context.locale;
    let n_elems = dimensions.len() as i32;
    let c_dims: Vec<*mut HaskellValue> = dimensions.iter().map(|d| d.ptr).collect();
    let dim_list = unsafe { dimensionListCreate(c_dims.as_ptr(), n_elems) };
    let haskell_entities = unsafe {
        wparseText(
            c_text.as_ptr(),
            reference_time.ptr,
            locale.ptr,
            dim_list,
            with_latent as u8,
        )
    };
    let string_result = unsafe {
        CStr::from_ptr(haskell_entities)
            .to_string_lossy()
            .to_owned()
            .to_string()
    };
    Ok(string_result)
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn duckling(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", VERSION)?;
    m.add_wrapped(wrap_pyfunction!(load_time_zones))?;
    m.add_wrapped(wrap_pyfunction!(get_current_ref_time))?;
    m.add_wrapped(wrap_pyfunction!(parse_ref_time))?;
    m.add_wrapped(wrap_pyfunction!(parse_lang))?;
    m.add_wrapped(wrap_pyfunction!(default_locale_lang))?;
    m.add_wrapped(wrap_pyfunction!(parse_locale))?;
    m.add_wrapped(wrap_pyfunction!(parse_dimensions))?;
    m.add_wrapped(wrap_pyfunction!(parse_text))?;
    m.add_wrapped(wrap_pyfunction!(init))?;
    m.add_wrapped(wrap_pyfunction!(stop))?;
    m.add_class::<TimeZoneDatabaseWrapper>()?;
    m.add_class::<Context>()?;
    m.add_class::<DucklingTimeWrapper>()?;
    Ok(())
}
