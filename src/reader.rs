extern crate pyo3;
use py_file::PyFile;
use pyo3::prelude::*;
use record;
use util::get_optional_single_byte;

type RecordsIter = Iterator<Item = csv::Result<csv::StringRecord>>;

/// Handles [CSVReader]'s reading from either a filesystem path or "`BinaryIO`" [PyObject]
pub enum CSVSource {
    /// A file-system path
    Path(String),
    /// A [PyFile] wrapping a Python file-like "`BinaryIO`" [PyObject].
    Readable(PyFile),
}

/// The `CSVReader` Python class
#[pyclass(subclass)]
pub struct CSVReader {
    token: PyToken,
    // It would be nice to have a reference to csv::Reader here,
    // but I haven't figured out lifetimes yet.
    /// Iterator over the parsed records
    iter: Box<RecordsIter>,
}

/// Builds a [`csv::Reader`] and returns a boxed [`Iterator`] of the
/// records from [`csv::Reader::into_records`].
///
/// # Arguments
///
/// * `source` - [CSVSource] to read the CSV from.
/// * `delimiter` - CSV field separator.
/// * `terminator` - CSV record separator.
pub fn make_records_iterator(
    source: CSVSource,
    delimiter: u8,
    terminator: u8,
) -> csv::Result<Box<RecordsIter>> {
    let mut x = csv::ReaderBuilder::new();
    let builder = x
        .delimiter(delimiter)
        .has_headers(false)
        .terminator(csv::Terminator::Any(terminator));

    use self::CSVSource::{Path, Readable};
    {
        match source {
            Readable(readable) => {
                let rdr = builder.from_reader(readable);
                Ok(Box::new(rdr.into_records()))
            }
            Path(path) => {
                let rdr = builder.from_path(path)?;
                Ok(Box::new(rdr.into_records()))
            }
        }
    }
}

/// Implements the Python type methods for `CSVReader`
#[pymethods]
impl CSVReader {
    /// Creates a new CSVReader instance
    ///
    /// - `path_or_fd` - Either a string path to a file or a [binary file].
    /// - `delimiter` - CSV field separator
    /// - `terminator` - CSV field separator
    ///
    /// Note: The `delimiter` and `terminator` [PyBytes] objects must only
    /// contain a single byte.
    ///
    ///  [binary file]: https://docs.python.org/3/glossary.html#term-binary-file
    #[new]
    pub fn __new__(
        obj: &PyRawObject,
        path_or_fd: &'static PyObjectRef,
        delimiter: Option<&PyBytes>,
        terminator: Option<&PyBytes>,
    ) -> PyResult<()> {
        debug!(
            "__new__: path_or_fd: {:?}, delimiter: {:?}, terminator: {:?}",
            path_or_fd, delimiter, terminator
        );
        let gil = Python::acquire_gil();
        let py = gil.python();

        let delimiter_arg = get_optional_single_byte(delimiter, b',')?;
        let terminator_arg = get_optional_single_byte(terminator, b'\n')?;

        let path_or_fd_obj = path_or_fd.to_object(py);

        let source = if py.is_instance::<PyString, _>(path_or_fd_obj.as_ref(py))? {
            // Treat path_or_fd_obj as a path
            CSVSource::Path(path_or_fd_obj.extract(py)?)
        } else {
            // Treat path_or_fd_obj as a "binary file"
            CSVSource::Readable(PyFile::from_object(path_or_fd_obj)?)
        };

        match make_records_iterator(source, delimiter_arg, terminator_arg) {
            Ok(iter) => obj.init(|token| CSVReader { token, iter }),
            Err(error) => match error.into_kind() {
                csv::ErrorKind::Io(err) => Err(err.into()),
                err => Err(exc::IOError::py_err(format!(
                    "Could not parse CSV: {:?}",
                    err
                ))),
            },
        }
    }
}

import_exception!(rustcsv.error, UnequalLengthsError);
import_exception!(rustcsv.error, UTF8Error);

/// Create a Python rustcsv.error.Position object from a csv::Position
pub fn make_error_position(pos: csv::Position) -> PyResult<PyObject> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let errors_mod = py.import("rustcsv.error")?;
    let position_type = errors_mod.get("Position")?;
    Ok(position_type
        .to_object(py)
        .call1(py, (pos.byte(), pos.line(), pos.record()))?)
}

#[pyproto]
impl PyIterProtocol for CSVReader {
    fn __iter__(&mut self) -> PyResult<PyObject> {
        debug!("__iter__");
        Ok(self.into())
    }

    /// Read the next record from [CSVReader::iter]
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
                Err(error) => match error.into_kind() {
                    csv::ErrorKind::Io(err) => {
                        error!("IO error: {:?}", err);
                        Err(PyErr::from(err))
                    }
                    csv::ErrorKind::Utf8 { pos, err } => {
                        let position = match pos {
                            Some(p) => Some(make_error_position(p.clone())?),
                            None => None,
                        };
                        Err(UTF8Error::py_err((format!("{:?}", err), position)))
                    }
                    csv::ErrorKind::UnequalLengths {
                        pos,
                        expected_len,
                        len,
                    } => {
                        let position = match pos {
                            Some(p) => Some(make_error_position(p.clone())?),
                            None => None,
                        };
                        Err(UnequalLengthsError::py_err((
                            format!(
                                "Unequal lengths: Expected length {:?} got length {:?}",
                                expected_len, len,
                            ),
                            position,
                        )))
                    }
                    not_io_error => Err(exc::ValueError::py_err(format!(
                        "CSV parsing error: {:?}",
                        not_io_error
                    ))),
                },
            },
            None => {
                debug!("Reached end");
                Ok(None)
            }
        }
    }
}

impl Drop for CSVReader {
    fn drop(&mut self) {
        debug!("Dropping CSVReader")
    }
}
