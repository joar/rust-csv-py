extern crate csv;
extern crate pyo3;

use pyo3::prelude::*;
use std::convert;

pub struct Record {
    r: csv::StringRecord,
}

impl convert::From<csv::StringRecord> for Record {
    fn from(record: csv::StringRecord) -> Self {
        Record { r: record }
    }
}

impl IntoPyTuple for Record {
    fn into_tuple(self, py: Python) -> Py<PyTuple> {
        let items: Vec<&str> = self.r.iter().collect();
        PyTuple::new(py, items.as_slice())
    }
}
