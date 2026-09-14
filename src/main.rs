use nalgebra::{DMatrix, DVector};
use rust_arena::gauss_newton_step;

fn main() {
    let jacobian = DMatrix::from_row_slice(2, 2, &[1.0, 0.0, 0.0, 1.0]);
    let residuals = DVector::from_vec(vec![2.0, 2.0]);

    match gauss_newton_step(&jacobian, &residuals) {
        Ok(step) => println!("Gauss-Newton step computed: {:?}", step),
        Err(e) => eprintln!("Error: {}", e),
    }
}
