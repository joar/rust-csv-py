extern crate csv;
extern crate pyo3;

use pyo3::prelude::*;

use py_file::PyFile;
use util::{get_optional_single_byte};

#[pyclass(subclass)]
pub struct CSVWriter {
    token: PyToken,
    writer: csv::Writer<PyFile>,
}

#[pymethods]
impl CSVWriter {
    #[new]
    fn __new__(
        obj: &PyRawObject,
        fd: &'static PyObjectRef,
        terminator: Option<&PyBytes>,
        escape: Option<&PyBytes>,
        double_quote: Option<bool>,
    ) -> PyResult<()> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let writer = csv::WriterBuilder::new()
            .double_quote(double_quote.unwrap_or(true))
            .terminator(csv::Terminator::Any(get_optional_single_byte(
                terminator, b'\n',
            )?)).escape(get_optional_single_byte(escape, b'\\')?)
            .quote_style(csv::QuoteStyle::Always)
            .from_writer(PyFile::extract(fd)?);
        obj.init(|token| CSVWriter { writer, token })
    }

    fn writerow(&mut self, record: &PyObjectRef) -> PyResult<()> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        if !py.is_instance::<PyTuple, PyObjectRef>(record)? {
            return Err(exc::TypeError::new(format!(
                "Expected tuple, got {:?}",
                record
            )));
        }
        debug!("record: {:?}", record);
        let record_tuple: &PyTuple = <PyTuple as PyTryFrom>::try_from(record)?;
        let r = record_tuple.iter().map(|pi| {
            let i: String = pi.extract().unwrap_or_else(|err| {
                error!("Could not convert {:?} to string: {:?}", pi, err);
                "invalid".into()
            });
            i
        });
        match self.writer.write_record(r) {
            Ok(r) => Ok(r),
            Err(error) => {
                error!("Could not write record: {:?}", error);
                Err(exc::IOError::new(format!(
                    "Could not write record: {:?}",
                    error
                )))
            }
        }
    }

    fn flush(&mut self) -> PyResult<()> {
        Ok(self.writer.flush()?)
    }
}
