use nalgebra::{DMatrix, DVector};
use rust_arena::gauss_newton_step;

#[test]
fn test_integration_gauss_newton() {
    let j = DMatrix::from_row_slice(3, 2, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let r = DVector::from_vec(vec![1.0, 2.0, 3.0]);

    let step =
        gauss_newton_step(&j, &r).expect("Failed to compute Gauss-Newton step in integration test");

    assert!((step[0] - 0.0).abs() < 1e-10);
    assert!((step[1] - 0.5).abs() < 1e-10);
}
