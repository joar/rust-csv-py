#![feature(proc_macro, specialization)]

extern crate csv;
extern crate pyo3;

use pyo3::exc;
use pyo3::prelude::*;
use pyo3::py::class as pyclass;
use pyo3::py::methods as pymethods;
use pyo3::py::modinit as pymodinit;
use pyo3::py::proto as pyproto;


type RecordsIter = Iterator<Item=csv::Result<csv::StringRecord>>;

#[pyclass]
struct CSVReader {
    token: PyToken,
    iter: Box<RecordsIter>,
}


fn records_iterator(
    path: String,
) -> csv::Result<Box<RecordsIter>> {
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b'\x01')
        .flexible(true)
        .has_headers(false)
        .terminator(csv::Terminator::Any(b'\x02'))
        .from_path(path)?;

    let iter: Box<RecordsIter> = Box::new(rdr.into_records());
    return Ok(iter);
}

#[pymethods]
impl CSVReader {
    #[new]
    fn __new__(
        obj: &PyRawObject,
        path: String,
    ) -> PyResult<()> {
        let iter = match records_iterator(
            path,
        ) {
            Ok(rdr) => rdr,
            Err(err) => {
                return Err(PyErr::new::<exc::IOError, _>(
                    format!("Could not parse CSV: {:?}", err)
                ));
            }
        };

        obj.init(|token| {
            CSVReader {
                token,
                iter,
            }
        })
    }
}

fn record_to_list(py: Python, record: csv::StringRecord) -> PyResult<PyObject> {
    let list = PyList::new::<&str>(py, &[]);
    for field in record.iter() {
        list.append(field)?;
    }
    return Ok(list.into());
}

#[pyproto]
impl PyIterProtocol for CSVReader {
    fn __iter__(&mut self) -> PyResult<PyObject> {
        Ok(self.into())
    }

    fn __next__(&mut self) -> PyResult<Option<PyObject>> {
        match self.iter.next() {
            Some(res) => match res {
                Ok(record) => {
                    let gil = Python::acquire_gil();
                    let py = gil.python();
                    let output = record_to_list(py, record)?;
                    return Ok(Some(output));
                }
                Err(err) => {
                    return Err(PyErr::new::<exc::IOError, _>(
                        format!("Could not read record {:?}", err)
                    ));
                }
            },
            None => {
                return Ok(None);
            }
        }
    }
}


// Add bindings to the generated python module
// N.B: names: "_rustcsv" must be the name of the `.so` or `.pyd` file
/// This module is implemented in Rust.
#[pymodinit(_rustcsv)]
fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CSVReader>()?;
    Ok(())
}
