use cfdrust::*;
use std::thread;
use pyo3::prelude::*;
use ndarray::prelude::*;
use numpy::{PyArray1,PyArray2,ToPyArray};

/// Some simple array operations
#[pymodinit]
fn array_operation(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "sum")]
    /// Return sum of the elements of an array
    ///
    /// Args:
    ///     arr : An array with numbers as elements
    ///
    /// Returns:
    ///     sum of all elements
    // fn sum_array_py<T:num_traits::identities::Zero + std::clone::Clone>(py: Python, arr: PyArray1<T>) -> T {
    //     sum_array(&arr)
    // }
    fn sum_array_py(py: Python, arr: PyArray1<f64>) -> f64 {
        let close = arr.to_owned();
        let close = close.as_array();
        sum_array(&close)
    }
    Ok(())
}
