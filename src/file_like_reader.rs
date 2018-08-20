extern crate pyo3;

use pyo3::prelude::*;
use pyo3::Python;
use std::io;
use std::io::Read;
use std::io::Write;

#[derive(Debug)]
pub struct FileLikeReader<'file_like> {
    file_like: &'file_like PyObjectRef,
}

impl<'file_like> FileLikeReader<'file_like> {
    pub fn new(file_like: &'file_like PyObjectRef) -> FileLikeReader {
        debug!("Creating from file_like {:?}", file_like);
        FileLikeReader {
            file_like,
        }
    }

    fn read_bytes_via_eval(&self, size: usize) -> PyResult<Box<Vec<u8>>> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let locals = PyDict::new(py);
        let fd = self.file_like.to_object(py);
        locals.set_item("fd", fd)?;
        locals.set_item("length", size)?;

        let call_result = py.eval(
            "fd.read(length)",
            None,
            Some(locals)
        )?;

        debug!("call_result: {:?}", call_result);
        debug!("locals = {:?}", locals);
        Ok(Box::new(call_result.extract()?))
    }

    fn read_bytes(&self, size: usize) -> PyResult<Box<Vec<u8>>> {
        // Get the fd.read() callable
        let read_func: &PyObjectRef = self.file_like.getattr("read")?;
        debug!("read_func is {:?}", read_func);

        // Call fd.read(len(buf))
        let call_result: &PyObjectRef = read_func.call1((size, ))?;
        debug!("call_result: {}: {:?}", call_result, call_result);

        // Extract the PyBytes into a Box<Vec<u8>>
        Ok(
            Box::new(
                call_result.extract()?
            )
        )
    }
}

impl<'file_like> Read for FileLikeReader<'file_like> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        debug!("buf.len(): {:?}", buf.len());
        let read_buf = self.read_bytes(buf.len()).or_else(|err| {
            error!("Python exception");
            err.print(Python::acquire_gil().python());
            panic!("AAAH");
            // TODO: How do i print
            Err(err)
        })?;

        // Write the bytes into "buf"
        // Need to borrow as mutable here, not sure why
        buf.as_mut().write(&read_buf[..])
    }
}

impl<'file_like> Drop for FileLikeReader<'file_like> {
    fn drop(&mut self) {
        debug!("Dropping {:?}", self);
    }
}

#[cfg(test)]
mod tests {
    use pyo3::{PyDict, PyResult, Python};
    use pyo3::prelude::*;
    use std::io::Read;
    use std::io::Write;
    use super::FileLikeReader;
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
        locals.set_item("path", path.to_str())
            .unwrap_or_else(|err| {
                panic!("Could not set local: {:?}", err);
            });

        let file_like = py.eval(
            "open(path, 'rb')", None, Some(&locals),
        ).unwrap_or_else(|err| {
            panic!("Error: {:?}", err);
        });

        let mut rdr = FileLikeReader::new(file_like);
        let mut buffer = String::new();
        rdr.read_to_string(&mut buffer).unwrap_or_else(|err| {
            panic!("Could not read to string: {}", err);
        });
        assert_eq!(contents, buffer);
    }
}
