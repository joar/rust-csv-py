#![feature(use_extern_macros, specialization, extern_prelude)]

extern crate csv;
extern crate env_logger;
#[macro_use] extern crate log;
#[macro_use] extern crate pyo3;
/// Used for testing
extern crate tempfile;

use file_like_reader::FileLikeReader;
use pyo3::exc;
use pyo3::prelude::*;

mod record;
mod file_like_reader;
mod instance;

type RecordsIter = Iterator<Item=csv::Result<csv::StringRecord>>;

#[pyclass(subclass)]
struct CSVReader {
    token: PyToken,
    // It would be nice to have a reference to csv::Reader here,
    // but I haven't figured out lifetimes yet.
    iter: Box<RecordsIter>,
}


fn records_iterator(
    readable: FileLikeReader<'static>,
    delimiter: u8,
    terminator: u8,
) -> csv::Result<Box<RecordsIter>> {
    let rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        .terminator(csv::Terminator::Any(terminator))
        .from_reader(readable);

    // XXX: I'm not sure that this doesn't read all the records into memory.
    // If that is the case it would explain why I don't need to confront
    // lifetimes in my struct.
    let iter: Box<RecordsIter> = Box::new(rdr.into_records());
    return Ok(iter);
}

fn get_optional_single_byte(bytes: Option<&PyBytes>, default: u8) -> PyResult<u8> {
    match bytes {
        Some(b) => get_single_byte(b),
        None => Ok(default)
    }
}

fn get_single_byte(bytes: &PyBytes) -> PyResult<u8> {
    let data: &[u8] = bytes.data();
    if data.len() > 1 {
        error!("data is too long: {:?}", data);
        return Err(PyErr::new::<exc::ValueError, _>(
            (format!("Expected a single byte, got {:?}", data), )
        ));
    }
    if data.len() < 1 {
        error!("data is too short: {:?}", data);
        return Err(PyErr::new::<exc::ValueError, _>(
            (format!("Expected a single byte, got {:?}", data), )
        ));
    }
    return Ok(data[0]);
}

impl CSVReader {
    fn new<'fd>(
        token: PyToken,
        file_like: &'static PyObjectRef,
        delimiter: u8,
        terminator: u8,
    ) -> PyResult<CSVReader> {
        info!("file_like: {:?}", file_like);

        let gil = Python::acquire_gil();
        let py = gil.python();


        let reader = FileLikeReader::new(
            file_like
        );

        let iter = match records_iterator(
            reader,
            delimiter,
            terminator,
        ) {
            Ok(it) => it,
            Err(err) => {
                return Err(PyErr::new::<exc::IOError, _>(
                    format!("Could not parse CSV: {:?}", err)
                ));
            }
        };

        Ok(
            CSVReader {
                token,
                iter,
            }
        )
    }
}

#[pymethods]
impl CSVReader {
    #[new]
    fn __new__(
        obj: &PyRawObject,
        file_like: &'static PyObjectRef,
        delimiter: Option<&PyBytes>,
        terminator: Option<&PyBytes>,
    ) -> PyResult<()> {
        // I've hung these parameter extractions here to DRY.
        let delimiter_arg = get_optional_single_byte(delimiter, b',')?;
        let terminator_arg = get_optional_single_byte(terminator, b'\n')?;

        obj.init(|token| {
            CSVReader::new(
                token,
                file_like,
                delimiter_arg,
                terminator_arg
            ).unwrap()
        })
    }
}

#[pyproto]
impl PyIterProtocol for CSVReader {
    fn __iter__(&mut self) -> PyResult<PyObject> {
        debug!("__iter__");
        Ok(self.into())
    }

    fn __next__(&mut self) -> PyResult<Option<PyObject>> {
        debug!("__next__");
        match self.iter.next() {
            Some(res) => match res {
                Ok(r) => {
                    let gil = Python::acquire_gil();
                    let py = gil.python();
                    let rec: record::Record = r.into();
                    let t = rec.into_tuple(py);
                    Ok(Some(t.into_object(py)))
                }
                Err(err) => Err(
                    PyErr::new::<exc::IOError, _>(
                        format!("Could not read row: {:?}", err)
                    )
                )
            }
            None => {
                info!("Reached end");
                Ok(None)
            }
        }
    }
}

impl Drop for CSVReader {
    fn drop(&mut self) {
        info!("Dropping CSVReader")
    }
}


use instance::InstanceWrapper;

#[pyfunction]
pub fn wrap_and_hello(
    instance: &'static PyObjectRef,
) -> PyResult<String> {
    let wrapper = InstanceWrapper {
        instance
    };
    wrapper.say_hello_to_instance()
}

// Add bindings to the generated python module
// N.B: names: "_rustcsv" must be the name of the `.so` or `.pyd` file
/// PyO3 + rust-csv
/// An exploration in reading CSV as fast as possible from Python.
#[pymodinit]
fn _rustcsv(_py: Python, m: &PyModule) -> PyResult<()> {
    env_logger::init();
    m.add_class::<CSVReader>()?;
    m.add_class::<instance::PyInstanceWrapper>()?;
    m.add_function(wrap_function!(wrap_and_hello))?;
    Ok(())
}
