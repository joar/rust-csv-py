extern crate pyo3;
use pyo3::prelude::*;

pub fn get_optional_single_byte(bytes: Option<&PyBytes>, default: u8) -> PyResult<u8> {
    match bytes {
        Some(b) => get_single_byte(b),
        None => Ok(default),
    }
}

/// Extracts a single u8 from a PyBytes object
/// If the PyBytes object contains more or less than 1 byte, an error is returned.
pub fn get_single_byte(bytes: &PyBytes) -> PyResult<u8> {
    let data: &[u8] = bytes.data();
    if data.len() > 1 {
        error!("data is too long: {:?}", data);
        return Err(PyErr::new::<exc::ValueError, _>((format!(
            "Expected a single byte, got {:?}",
            data
        ),)));
    }
    if data.len() < 1 {
        error!("data is too short: {:?}", data);
        return Err(PyErr::new::<exc::ValueError, _>((format!(
            "Expected a single byte, got {:?}",
            data
        ),)));
    }
    return Ok(data[0]);
}
