use pyo3::prelude::pyfunction;
use super::fib_number::fibonacci_number;

#[pyfunction]
pub fn fibonacci_numbers(numbers: Vec<i32>) -> Vec<u64> {
    let mut vec = vec![];
    for n in numbers {
        vec.push(fibonacci_number(n));
    }
    return vec;
}