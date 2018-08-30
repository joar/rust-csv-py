#![feature(specialization, extern_prelude)]

extern crate csv;
extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate pyo3;
/// Used for testing
extern crate tempfile;

pub mod py_file;
pub mod reader;
pub mod record;
pub mod util;
pub mod writer;

use pyo3::prelude::*;

// Add bindings to the generated python module
// N.B: names: "_rustcsv" must be the name of the `.so` or `.pyd` file
/// PyO3 + rust-csv
/// An exploration in reading CSV as fast as possible from Python.
#[pymodinit]
pub fn _rustcsv(_py: Python, m: &PyModule) -> PyResult<()> {
    env_logger::init();
    m.add_class::<reader::CSVReader>()?;
    m.add_class::<writer::CSVWriter>()?;
    Ok(())
}
