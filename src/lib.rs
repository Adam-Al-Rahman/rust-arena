//! Gauss-Newton Update Step implementation.

use nalgebra::{DMatrix, DVector};

/// Computes the Gauss-Newton update step for non-linear optimization.
///
/// Formula: \Delta \beta = (J^T J)^{-1} J^T r
///
/// # Arguments
/// * `jacobian` - The Jacobian matrix (J) of shape (m, n)
/// * `residuals` - The residual vector (r) of length m
///
/// # Returns
/// * `Ok(DVector<f64>)` - The computed step \Delta \beta
/// * `Err(String)` - Description of error (dimension mismatch or singular
///   matrix)
pub fn gauss_newton_step(
    jacobian: &DMatrix<f64>,
    residuals: &DVector<f64>,
) -> Result<DVector<f64>, String> {
    let (m, _) = jacobian.shape();

    if residuals.len() != m {
        return Err(format!(
            "Dimension mismatch: Jacobian has {} rows, but residuals vector has length {}.",
            m,
            residuals.len()
        ));
    }

    // Compute approximate Hessian: J^T J
    let jt = jacobian.transpose();
    let jtj = &jt * jacobian;

    // Compute gradient: J^T r
    let jtr = &jt * residuals;

    // Solve (J^T J) \Delta \beta = J^T r
    // We use SVD for extreme stability and explicit singularity detection.
    let svd = jtj.svd(true, true);

    let min_sv = svd.singular_values.min();

    // Check if the matrix is singular or highly ill-conditioned
    if min_sv < 1e-10 {
        return Err("Matrix is singular or ill-conditioned. \
             Consider using a Levenberg-Marquardt fallback with a damping factor."
            .to_string());
    }

    // Solve via pseudo-inverse using the computed SVD
    let step = svd
        .solve(&jtr, 1e-10)
        .map_err(|_| "SVD solve failed".to_string())?;
    Ok(step)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_standard_step() {
        // J = [[1, 2], [3, 4], [5, 6]]
        let j = DMatrix::from_row_slice(3, 2, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        // r = [1, 2, 3]
        let r = DVector::from_vec(vec![1.0, 2.0, 3.0]);

        let step = gauss_newton_step(&j, &r).unwrap();
        // J^T r = [22, 28]^T
        // J^T J = [[35, 44], [44, 56]]
        // Expected solution: [0, 0.5]^T
        assert!((step[0] - 0.0).abs() < 1e-10);
        assert!((step[1] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_singular_jacobian() {
        // J = [[1, 1], [1, 1]] (Columns are linearly dependent, rank 1)
        let j = DMatrix::from_row_slice(2, 2, &[1.0, 1.0, 1.0, 1.0]);
        let r = DVector::from_vec(vec![1.0, 1.0]);

        let result = gauss_newton_step(&j, &r);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Levenberg-Marquardt"));
    }

    #[test]
    fn test_dimension_mismatch() {
        let j = DMatrix::from_row_slice(2, 2, &[1.0, 0.0, 0.0, 1.0]);
        let r = DVector::from_vec(vec![1.0, 2.0, 3.0]); // 3 elements instead of 2

        let result = gauss_newton_step(&j, &r);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Dimension mismatch"));
    }

    #[test]
    fn test_already_at_minimum() {
        // J = [[1, 0], [0, 1]]
        let j = DMatrix::from_row_slice(2, 2, &[1.0, 0.0, 0.0, 1.0]);
        // Zero residuals
        let r = DVector::from_vec(vec![0.0, 0.0]);

        let step = gauss_newton_step(&j, &r).unwrap();
        // Step should be [0, 0]
        assert!((step[0] - 0.0).abs() < 1e-10);
        assert!((step[1] - 0.0).abs() < 1e-10);
    }
}
