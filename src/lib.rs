pub mod buf_and_str;
pub mod datetime;
pub mod dict_iter;
pub mod objstore;
pub mod othermod;
pub mod subclassing;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn get_21() -> usize {
    21
}

#[pymodule]
fn mipha(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_21))?;

    Ok(())
}
