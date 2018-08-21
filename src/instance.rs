/// Playground for PyObjectRef things
extern crate pyo3;

use pyo3::prelude::*;

pub struct InstanceWrapper<'i> {
    pub instance: &'i PyObjectRef,
}

impl<'i> InstanceWrapper<'i> {
    pub fn get_instance_name(&self) -> PyResult<String> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        // Get the fd.read() callable
        let read_func: &PyObjectRef = self.instance.getattr("get_name")?;
        debug!("read_func is {:?}", read_func);

        // Call fd.read(len(buf))
        let call_result: &PyObjectRef = read_func.call0()?;
        debug!("call_result: {}: {:?}", call_result, call_result);

        // Extract the PyBytes into a Box<Vec<u8>>
        Ok(call_result.extract()?)
    }

    pub fn say_hello_to_instance(&self) -> PyResult<String> {
        let name = self.get_instance_name()?;
        Ok(name)
    }
}

#[pyclass(subclass)]
pub struct PyInstanceWrapper {
    wrapper: InstanceWrapper<'static>,
    token: PyToken,
}

#[pymethods]
impl PyInstanceWrapper {
    #[new]
    fn __new__(obj: &PyRawObject, instance: &'static PyObjectRef) -> PyResult<()> {
        obj.init(|token| PyInstanceWrapper {
            token,
            wrapper: InstanceWrapper { instance },
        })
    }

    fn get_name(&self) -> PyResult<String> {
        self.wrapper.get_instance_name()
    }
}
