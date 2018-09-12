extern crate csv;
extern crate pyo3;

use pyo3::prelude::*;

use py_file::PyFile;
use util::get_optional_single_byte;

#[pyclass(subclass)]
pub struct QuoteStyle {
    token: PyToken,
}

#[pyclass(subclass)]
pub struct CSVWriter {
    token: PyToken,
    writer: csv::Writer<PyFile>,
}

fn parse_quote_style(quote_style: &str) -> PyResult<csv::QuoteStyle> {
    match quote_style {
        "necessary" => Ok(csv::QuoteStyle::Necessary),
        "always" => Ok(csv::QuoteStyle::Always),
        "never" => Ok(csv::QuoteStyle::Never),
        "non_numeric" => Ok(csv::QuoteStyle::NonNumeric),
        _ => Err(exc::ValueError::py_err(format!(
            "Invalid quote style: {:?}",
            quote_style
        ))),
    }
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
        quote_style: Option<String>,
    ) -> PyResult<()> {
        let writer = csv::WriterBuilder::new()
            .double_quote(double_quote.unwrap_or(true))
            .terminator(csv::Terminator::Any(get_optional_single_byte(
                terminator, b'\n',
            )?)).escape(get_optional_single_byte(escape, b'\\')?)
            .quote_style(parse_quote_style(
                quote_style.unwrap_or("necessary".into()).as_str(),
            )?).from_writer(PyFile::extract(fd)?);
        obj.init(|token| CSVWriter { writer, token })
    }

    /// Writes a CSV row to the file.
    fn writerow(&mut self, record: &PyObjectRef) -> PyResult<()> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        if !py.is_instance::<PyTuple, PyObjectRef>(record)? {
            return Err(exc::TypeError::py_err(format!(
                "Expected tuple, got {:?}",
                record
            )));
        }
        debug!("record: {:?}", record);
        let record_tuple: &PyTuple = <PyTuple as PyTryFrom>::try_from(record)?;
        let r = record_tuple.iter().map(|i| {
            // TODO: Better error handling when item is not a string
            i.extract::<String>().unwrap_or_else(|err| {
                error!("Could not convert {:?} to string: {:?}", i, err);
                "invalid".into()
            })
        });
        match self.writer.write_record(r) {
            Ok(r) => Ok(r),
            Err(error) => {
                error!("Could not write record: {:?}", error);
                Err(exc::IOError::py_err(format!(
                    "Could not write record: {:?}",
                    error
                )))
            }
        }
    }

    /// Flush the underlying [PyFile] to disk.
    fn flush(&mut self) -> PyResult<()> {
        Ok(self.writer.flush()?)
    }
}
