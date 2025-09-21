use crate::models::Potential;
use crate::common::solver::{find_phi_end, find_phi_exit};
use crate::cosmology::{epsilon, eta, spectral_index, tensor_to_scalar_ratio};

pub mod potential;

pub fn calculate(
    potential: &dyn Potential,
    n_target: f64,
    precision: f64,
    simpson_max_iter: usize,
) -> Result<(f64, f64), String> {
    // For chaotic models, we can use a fixed search range.
    let phi_end_search_range = (0.0, 20.0);
    let phi_end = find_phi_end(potential, phi_end_search_range, precision)
        .map_err(|e| format!("Could not find phi_end: {}", e))?;

    let phi_exit_search_range = (phi_end + 0.1, 30.0);
    let phi_exit = find_phi_exit(potential, phi_end, n_target, phi_exit_search_range, precision, simpson_max_iter)
        .map_err(|e| format!("Could not find phi_exit: {}", e))?;

    let eps = epsilon(potential, phi_exit);
    let eta_val = eta(potential, phi_exit);

    let ns = spectral_index(eps, eta_val);
    let r = tensor_to_scalar_ratio(eps);

    Ok((ns, r))
}
