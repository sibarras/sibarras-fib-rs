use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
mod fib_calcs;
use fib_calcs::{
    fib_number::fibonacci_number,
    fib_numbers::fibonacci_numbers
};

#[pyfunction]
fn say_hello() {
    println!("Saying hello from Rust!");
}

#[pymodule]
fn sibarras_fib_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(say_hello))?;
    m.add_wrapped(wrap_pyfunction!(fibonacci_number))?;
    m.add_wrapped(wrap_pyfunction!(fibonacci_numbers))?;
    Ok(())
}
