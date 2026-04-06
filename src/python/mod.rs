use crate::config::Config;
use crate::transcriber::{Transcriber, default};
use pyo3::prelude::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[pyfunction]
fn ___version() -> &'static str {
    VERSION
}

pub fn jarvis_transcriber(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(___version))?;
    m.add_wrapped(wrap_pyfunction!(default))?;

    m.add_class::<Config>()?;
    m.add_class::<Transcriber>()?;
    Ok(())
}
