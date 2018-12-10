extern crate csv;
extern crate pyo3;

use pyo3::types::PyTuple;
use pyo3::IntoPyObject;
use pyo3::IntoPyTuple;
use pyo3::Py;
use pyo3::PyObject;
use pyo3::Python;
use std::convert;

pub struct Record {
    r: csv::StringRecord,
}

impl convert::From<csv::StringRecord> for Record {
    fn from(record: csv::StringRecord) -> Self {
        Record { r: record }
    }
}

impl IntoPyObject for Record {
    fn into_object(self, py: Python) -> PyObject {
        self.into_tuple(py).into()
    }
}

impl IntoPyTuple for Record {
    fn into_tuple(self, py: Python) -> Py<PyTuple> {
        let items: Vec<&str> = self.r.iter().collect();
        PyTuple::new(py, items)
    }
}
