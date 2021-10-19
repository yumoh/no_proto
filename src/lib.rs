
mod proto;
use pyo3::prelude::*;
use proto::{Proto,Buffer};

#[pyfunction]
fn ping() -> String {
    return "pong".to_string()
}

#[pymodule]
fn libno_proto(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(ping))?;
    m.add_class::<Proto>()?;
    m.add_class::<Buffer>()?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
