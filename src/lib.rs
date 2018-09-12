#![feature(specialization, extern_prelude, core_intrinsics)]

extern crate built;
extern crate csv;
extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate pyo3;
// Used for testing
extern crate tempfile;

pub mod py_file;
pub mod reader;
pub mod record;
pub mod util;
pub mod writer;

use pyo3::prelude::*;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
macro_rules! pyo3_built {
    ($py: ident, $info: ident) => {{
        use $crate::built::util::strptime;
        use $crate::pyo3::prelude::PyDict;

        let info = PyDict::new($py);

        // Rustc
        let build = PyDict::new($py);
        build.set_item("rustc", $info::RUSTC)?;
        build.set_item("rustc-version", $info::RUSTC_VERSION)?;
        build.set_item("opt-level", $info::OPT_LEVEL)?;
        build.set_item("debug", $info::DEBUG)?;
        build.set_item("jobs", $info::NUM_JOBS)?;
        info.set_item("build", build)?;

        // info time
        let ts = strptime($info::BUILT_TIME_UTC).to_timespec();
        let dt = $py
            .import("datetime")?
            .get("datetime")?
            .into_object($py)
            .call_method1($py, "fromtimestamp", (ts.sec,))?;
        info.set_item("info-time", dt)?;

        // info dependencies
        let deps = PyDict::new($py);
        for (name, version) in $info::DEPENDENCIES.iter() {
            deps.set_item(name, version)?;
        }
        info.set_item("dependencies", deps)?;

        // Features
        let features = $info::FEATURES
            .iter()
            .map(|feat| PyString::new($py, feat))
            .collect::<Vec<_>>();
        info.set_item("features", features)?;

        // Host
        let host = PyDict::new($py);
        host.set_item("triple", $info::HOST)?;
        info.set_item("host", host)?;

        // Target
        let target = PyDict::new($py);
        target.set_item("arch", $info::CFG_TARGET_ARCH)?;
        target.set_item("os", $info::CFG_OS)?;
        target.set_item("family", $info::CFG_FAMILY)?;
        target.set_item("env", $info::CFG_ENV)?;
        target.set_item("triple", $info::TARGET)?;
        target.set_item("endianness", $info::CFG_ENDIAN)?;
        target.set_item("pointer-width", $info::CFG_POINTER_WIDTH)?;
        target.set_item("profile", $info::PROFILE)?;
        info.set_item("target", target)?;

        info.into_object($py)
    }};
}

// Add bindings to the generated python module
// N.B: names: "_rustcsv" must be the name of the `.so` or `.pyd` file
/// PyO3 + rust-csv
/// An exploration in reading CSV as fast as possible from Python.
#[pymodinit(rustcsv)]
pub fn rustcsv(_py: Python, m: &PyModule) -> PyResult<()> {
    use built_info;
    env_logger::init();
    m.add_class::<reader::CSVReader>()?;
    m.add_class::<writer::CSVWriter>()?;
    m.add::<PyObject>("__build__", pyo3_built!(_py, built_info))?;
    Ok(())
}
