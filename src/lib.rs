use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn say_hello() {
    println!("Saying hello from Rust!");
}

#[pymodule]
fn sibarras_fib_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(say_hello))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use pyo3::pymodule;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
