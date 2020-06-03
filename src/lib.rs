// ----------------------------------------------------------------------------
// Copyright (c) Treble.ai
//
// Licensed under the terms of the MIT License
// (see LICENSE.txt for details)
// ----------------------------------------------------------------------------

/// Python bindings for Haskell's Duckling library written in Rust

// PyO3 imports
use pyo3::class::PyMappingProtocol;
use pyo3::create_exception;
use pyo3::exceptions;
use pyo3::gc::{PyGCProtocol, PyVisit};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::PyTraverseError;

use curryrs::hsrt::{start, stop};
use curryrs::types::I64;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// Package version
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub enum HaskellValue {}

extern "C" {
    pub fn wcurrentReftime(tzdb: *mut HaskellValue, strPtr: *mut HaskellValue)
        -> *mut HaskellValue;
    pub fn wloadTimeZoneSeries(path: *mut HaskellValue) -> *mut HaskellValue;
    pub fn stringCreate(s: *const c_char) -> *mut HaskellValue;
    pub fn stringDestroy(s: *mut HaskellValue);
    pub fn stringGet(s: *mut HaskellValue) -> *const c_char;
    pub fn tzdbDestroy(db: *mut HaskellValue);
    pub fn duckTimeDestroy(time: *mut HaskellValue);
}

#[pyclass(name=String)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

fn main() {
    println!("Hello, world!");
}
