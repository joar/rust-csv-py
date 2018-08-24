extern crate pyo3;

use pyo3::prelude::*;
use pyo3::Python;
use std::io;
use std::io::Read;
use std::io::Write;

/// Wraps a Python file-like readable object.
/// The file-like object must be in binary mode
#[derive(Debug)]
pub struct PyReader {
    file_like: PyObject,
}

impl PyReader {
    pub fn from_ref(file_like: PyObject) -> PyResult<PyReader> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        info!("Creating from file_like_ref {:?}", file_like.as_ref(py));

        match file_like.getattr(py, "read") {
            Ok(_) => Ok(PyReader { file_like }),
            Err(error) => Err(exc::TypeError::new(format!(
                "Expected a file-like object, got {:?} (original error: {:?})",
                file_like.as_ref(py),
                error.to_object(py).as_ref(py)
            ))),
        }
    }

    #[inline]
    fn read_bytes(&self, size: usize) -> PyResult<Box<Vec<u8>>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        // Get the fd.read() callable
        let read_func: PyObject = self.file_like.getattr(py, "read")?;
//        debug!("read_func is {:?}", read_func.as_ref(py));

        // Call fd.read(size)
        let call_result = read_func.call1(py, (size,))?;
//        debug!("call_result: {:?}", &call_result);

        // Extract the PyBytes into a Box<Vec<u8>>
        match call_result.extract(py) {
            Ok(r) => Ok(Box::new(r)),
            Err(error) => if py.is_instance::<PyString, _>(call_result.as_ref(py))? {
                return Err(exc::TypeError::new(format!(
                    "The file {:?} is not open in binary mode. (Cause: {:?})",
                    self.file_like.as_ref(py),
                    error.to_object(py).as_ref(py),
                )));
            } else {
                return Err(error);
            },
        }
    }
}

impl Read for PyReader {
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

impl Drop for PyReader {
    fn drop(&mut self) {
        debug!("Dropping {:?}", self);
    }
}

#[cfg(test)]
mod tests {
    use super::PyReader;
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

        let mut rdr = PyReader::new(file_like);
        let mut buffer = String::new();
        rdr.read_to_string(&mut buffer).unwrap_or_else(|err| {
            panic!("Could not read to string: {}", err);
        });
        assert_eq!(contents, buffer);
    }
}
