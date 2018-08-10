#![feature(use_extern_macros, specialization)]

#[macro_use] extern crate log;
extern crate env_logger;
extern crate csv;
extern crate pyo3;

/// Used for testing
extern crate tempfile;

mod record;
mod file_like_reader;

use pyo3::exc;
use pyo3::prelude::*;
use file_like_reader::FileLikeReader;

type RecordsIter = Iterator<Item=csv::Result<csv::StringRecord>>;

#[pyclass(subclass)]
struct CSVReader {
    token: PyToken,
    file_like: &'static PyObjectRef,
    // It would be nice to have a reference to csv::Reader here,
    // but I haven't figured out lifetimes yet.
    iter: Box<RecordsIter>,
}


fn records_iterator(
//    path: String,
    reader: FileLikeReader<'static>,
    delimiter: u8,
    terminator: u8,
) -> csv::Result<Box<RecordsIter>> {
    let rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        .terminator(csv::Terminator::Any(terminator))
//        .from_path(path)?;
        .from_reader(reader);

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
//        path: String,
        file_like: &'static PyObjectRef,
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

        info!(target: "rustcsv", "file_like: {:?}", file_like);

        let reader = FileLikeReader::new(
            file_like
        );

        let iter = match records_iterator(
            reader,
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
                file_like,
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
                Ok(r) => {
                    let py = self.token.py();
                    let rec: record::Record = r.into();
                    let t = rec.into_tuple(py);
                    return Ok(Some(t.into_object(py)));
                }
                Err(err) => {
                    return Err(PyErr::new::<exc::IOError, _>(
                        format!("Could not read record {:?}", err)
                    ));
                }
            },
            None => {
                info!("Reached end");
                return Ok(None);
            }
        }
    }
}

impl Drop for CSVReader {
    fn drop(&mut self) {
        info!("Dropping CSVReader")
    }
}


// Add bindings to the generated python module
// N.B: names: "_rustcsv" must be the name of the `.so` or `.pyd` file
/// PyO3 + rust-csv
/// An exploration in reading CSV as fast as possible from Python.
#[pymodinit(_rustcsv)]
fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    env_logger::init();
    m.add_class::<CSVReader>()?;
    Ok(())
}
