
use crate::models::Potential;
use crate::common::solver::{find_phi_end, find_phi_exit};
use crate::cosmology::{epsilon, eta, spectral_index, tensor_to_scalar_ratio};
use crate::constants::M_P;
use std::f64::consts::PI;

pub mod potential;

pub fn calculate(
    potential: &dyn Potential,
    f: f64, // Receive the parameter 'f' directly
    n_target: f64,
    precision: f64,
    simpson_max_iter: usize,
) -> Result<(f64, f64), String> {

    // Dynamically set the search range based on the value of 'f'
    let phi_end_search_range = (0.0, PI * f);
    // let phi_end = find_phi_end(potential, phi_end_search_range, precision)
    //     .map_err(|e| format!("Could not find phi_end: {}", e))?;
    // println!("phi_end: {}", phi_end);
    // println!("epsilon(phi_end): {}", epsilon(potential, phi_end));
    let phi_end = natural_phi_end(f);
    println!("phi_end: {}", phi_end);
    println!("epsilon(phi_end): {}", epsilon(potential, phi_end));

    // Also set the exit search range dynamically
    let phi_exit_search_range = (phi_end+0.00001, PI * f); // Add a buffer
    let phi_exit = find_phi_exit(potential, phi_end, n_target, phi_exit_search_range, precision, simpson_max_iter)
        .map_err(|e| format!("Could not find phi_exit: {}", e))?;

    let eps = epsilon(potential, phi_exit);
    let eta_val = eta(potential, phi_exit);

    let ns = spectral_index(eps, eta_val);
    let r = tensor_to_scalar_ratio(eps);

    Ok((ns, r))
}

fn natural_phi_end(f: f64) -> f64 {
    2.0 * f * (M_P / (2.0_f64.sqrt() * f)).atan()
}