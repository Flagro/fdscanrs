use pyo3::prelude::*;
use rayon::prelude::*;

mod fdscan;

#[pymodule]
fn fdscanrs(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mine_candidate_keys, m)?)?;
    Ok(())
}

#[pyfunction]
fn mine_candidate_keys(py: Python, data: Vec<Vec<String>>) -> PyResult<Vec<Vec<usize>>> {
    Ok(fdscan::brute_force_keys(data))
}
