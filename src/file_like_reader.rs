extern crate pyo3;

use pyo3::prelude::*;
use pyo3::Python;
use std::io;
use std::io::Read;
use std::io::Write;

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
}

impl<'file_like> Read for FileLikeReader<'file_like> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        debug!("buf.len(): {:?}", buf.len());
        let gil = Python::acquire_gil();
        let py = gil.python();

        // Get the fd.read() callable
        let read_func: &PyObjectRef = self.file_like.getattr("read")?;
        debug!("read_func is {:?}", read_func);

        // Call fd.read(len(buf))
        let call_result: &PyObjectRef = read_func.call1((buf.len(), ))?;
        debug!("call_result: {}: {:?}", call_result, call_result);

        // Extract the PyBytes into a Box<Vec<u8>>
        let read_buf: Box<Vec<u8>> = Box::new(
            call_result.extract()?
        );

        // Write the bytes into "buf"
        // Need to borrow as mutable here, not sure why
        buf.as_mut().write(&read_buf[..])
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
