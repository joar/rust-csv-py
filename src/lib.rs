#![feature(use_extern_macros, specialization)]


extern crate csv;
extern crate pyo3;

use pyo3::exc;
use pyo3::prelude::*;


type RecordsIter = Iterator<Item=csv::Result<csv::StringRecord>>;

#[pyclass(subclass)]
struct CSVReader {
    token: PyToken,
    // It would be nice to have a reference to csv::Reader here,
    // but I haven't figured out lifetimes yet.
    iter: Box<RecordsIter>,
}


fn records_iterator(
    path: String,
    delimiter: u8,
    terminator: u8,
) -> csv::Result<Box<RecordsIter>> {
    let rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        .terminator(csv::Terminator::Any(terminator))
        .from_path(path)?;

    // XXX: I'm not sure that this doesn't read all the records into memory.
    // If that is the case it would explain why I don't need to confront
    // lifetimes in my struct.
    let iter: Box<RecordsIter> = Box::new(rdr.into_records());
    return Ok(iter);
}

fn get_single_byte(bytes: &PyBytes) -> PyResult<u8> {
    let data: &[u8] = bytes.data();
    if data.len() > 1 {
        return Err(PyErr::new::<exc::ValueError, _>(
            format!("Expected a single byte, got {:?}", data)
        ));
    }
    if data.len() < 1 {
        return Err(PyErr::new::<exc::ValueError, _>(
            format!("Expected a single byte, got {:?}", data)
        ));
    }
    return Ok(data[0]);
}

#[pymethods]
impl CSVReader {
    #[new]
    fn __new__(
        obj: &PyRawObject,
        path: String,
        delimiter: Option<&PyBytes>,
        terminator: Option<&PyBytes>,
    ) -> PyResult<()> {
        // I've hung these parameter extractions here to DRY.
        let delimiter_arg = match delimiter {
            Some(bytes) => {
                get_single_byte(bytes)?
            }
            None => { b',' }
        };
        let terminator_arg = match terminator {
            Some(bytes) => {
                get_single_byte(bytes)?
            }
            None => { b'\n' }
        };

        let iter = match records_iterator(
            path,
            delimiter_arg,
            terminator_arg,
        ) {
            Ok(it) => it,
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
                    let items: Vec<&str> = record.iter().collect();
                    let tuple = PyTuple::new(py, items.as_slice());
                    let output = tuple.into();
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
