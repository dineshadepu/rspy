use ndarray::prelude::*;
// use rayon::prelude::*;

/// Find the sum of all elements in a given array
pub fn sum_array<T:num_traits::identities::Zero + std::clone::Clone>(arr: &Array1<T>) -> T
// where T: Zero
{
    arr.scalar_sum()
}

#[test]
fn test_array(){
    let arr = Array::range(0., 5., 1.);
    let res = sum_array(&arr);
    assert_eq!(10., res);
}
