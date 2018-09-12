extern crate pyo3;

use pyo3::prelude::*;
use pyo3::Python;
use std::io;
use std::io::Read;
use std::io::Write;

/// Wraps a "[binary file]" [`PyObject`].
///
/// The main purpose of this wrapper is to implement the [std::io::Read] trait
/// so that [csv::Reader] can read directly from a Python "[binary file]" object.
///
///  [binary file]: https://docs.python.org/3/glossary.html#term-binary-file
#[derive(Debug)]
pub struct PyFile {
    file_like: PyObject,
}

impl PyFile {
    /// Create a new [PyFile] from a "[binary file]" [PyObject]
    ///
    /// # Arguments
    ///
    /// * `file_like` - [binary file] PyObject, will be quack-tested by getting
    /// the `read` Python attribute from it.
    pub fn from_object(file_like: PyObject) -> PyResult<PyFile> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        info!("Creating from file_like_ref {:?}", file_like.as_ref(py));

        // TODO: Use "readable()"
        // https://docs.python.org/3/library/io.html#io.IOBase.readable

        match file_like.getattr(py, "read") {
            Ok(_) => Ok(PyFile { file_like }),
            Err(error) => Err(exc::TypeError::py_err(format!(
                "Expected a file-like object, got {:?} (original error: {:?})",
                file_like.as_ref(py),
                error.to_object(py).as_ref(py)
            ))),
        }
    }

    /// Reads bytes from the the [binary file] [PyObject] [PyFile::file_like]
    ///
    /// The method acquires the GIL, then calls `getattr(file_like,
    ///
    /// # Arguments
    ///
    /// - `size` - Maximum number of bytes to read.
    #[inline]
    pub fn read_bytes(&self, size: usize) -> PyResult<Box<Vec<u8>>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        // Get the fd.read() callable
        let read_func: PyObject = self.file_like.getattr(py, "read")?;
        //        debug!("read_func is {:?}", read_func.as_ref(py));

        // Call fd.read(size)
        let call_result = read_func.call1(py, (size,))?;

        // Extract the PyBytes into a Box<Vec<u8>>
        match call_result.extract(py) {
            Ok(r) => Ok(Box::new(r)),
            //
            Err(error) => if py.is_instance::<PyString, _>(call_result.as_ref(py))? {
                return Err(exc::TypeError::py_err(format!(
                    "The file {:?} is not open in binary mode. (Cause: {:?})",
                    self.file_like.as_ref(py),
                    error.to_object(py).as_ref(py),
                )));
            } else {
                return Err(error);
            },
        }
    }

    /// Writes bytes to the [binary file] [PyObject] [PyFile::file_like]
    pub fn write_bytes(&mut self, buf: &[u8]) -> PyResult<usize> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        debug!("buf: {:?}", buf);
        let write_func = self.file_like.getattr(py, "write")?;
        let bytes = PyBytes::new(py, buf);
        debug!("bytes: {:?}", bytes.as_ref(py));
        let call_result = write_func.call1(py, (bytes,))?;

        // Return the number of bytes written
        Ok(call_result.extract(py)?)
    }
}

impl Write for PyFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(self.write_bytes(buf)?)
    }

    fn flush(&mut self) -> io::Result<()> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        debug!("flushing {:?}", self.file_like.as_ref(py));
        match self.file_like.getattr(py, "flush")?.call0(py) {
            Err(error) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Could not flush {:?} (Cause: {:?})",
                    self.file_like.as_ref(py),
                    error.to_object(py).as_ref(py),
                ),
            )),
            Ok(_) => Ok(()),
        }
    }
}

impl Read for PyFile {
    /// Reads bytes from the [`PyFile.file_like`] [`PyObject`] via
    /// [`PyFile.read_bytes`].
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        debug!("buf.len(): {:?}", buf.len());
        match self.read_bytes(buf.len()) {
            // Write the bytes into "buf"
            // Need to borrow as mutable here, not sure why
            Ok(read_buf) => buf.as_mut().write(&read_buf[..]),
            Err(error) => {
                let gil = Python::acquire_gil();
                let py = gil.python();
                error!(
                    "Could not read from {:?}: {:?}",
                    self.file_like.as_ref(py),
                    error.to_object(py).as_ref(py)
                );
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "Could not read from {:?}: {:?}",
                        self.file_like.as_ref(py),
                        error.to_object(py).as_ref(py),
                    ),
                ))
            }
        }
    }
}

impl Drop for PyFile {
    fn drop(&mut self) {
        debug!("Dropping {:?}", self);
    }
}

impl<'source> FromPyObject<'source> for PyFile {
    fn extract(ob: &'source PyObjectRef) -> Result<Self, PyErr> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        Ok(PyFile::from_object(ob.to_object(py))?)
    }
}

#[cfg(test)]
mod tests {
    use super::PyFile;
    use pyo3::prelude::*;
    use pyo3::{PyDict, PyResult, Python};
    use std::io::Read;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn read_file() {
        let contents = "Hello World!";
        let mut tmpfile = NamedTempFile::new().unwrap();
        write!(tmpfile, "{}", contents);

        let gil = Python::acquire_gil();
        let py = gil.python();

        let locals = PyDict::new(py);
        let path = tmpfile.path();
        locals
            .set_item("path", path.to_str())
            .unwrap_or_else(|err| {
                panic!("Could not set local: {:?}", err);
            });

        let file_like = py
            .eval("open(path, 'rb')", None, Some(&locals))
            .unwrap_or_else(|err| {
                panic!("Error: {:?}", err);
            });

        let mut rdr = PyFile::new(file_like);
        let mut buffer = String::new();
        rdr.read_to_string(&mut buffer).unwrap_or_else(|err| {
            panic!("Could not read to string: {}", err);
        });
        assert_eq!(contents, buffer);
    }
}
