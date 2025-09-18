use crate::potential::Potential;
use crate::solver::{find_phi_end, find_phi_exit};
use crate::cosmology::{epsilon, eta, spectral_index, tensor_to_scalar_ratio};

pub fn calculate_ns_r(
    potential: &dyn Potential,
    n_target: f64,
    precision: f64,
) -> Result<(f64, f64), String> {
    // 1. find_phi_end を呼び出す
    // TODO: この探索範囲は将来的にモデルごとに調整が必要になるかもしれない
    let phi_end_search_range = (20.0, 1.0);
    let phi_end = find_phi_end(potential, phi_end_search_range, precision)
        .map_err(|e| format!("Could not find phi_end: {}", e))?;

    // 2. find_phi_exit を呼び出す
    // TODO: この探索範囲も同様
    let phi_exit_search_range = (phi_end + 0.1, 30.0);
    let phi_exit = find_phi_exit(potential, phi_end, n_target, phi_exit_search_range, precision)
        .map_err(|e| format!("Could not find phi_exit: {}", e))?;

    // 3. epsilon, eta を計算する
    let eps = epsilon(potential, phi_exit);
    let eta_val = eta(potential, phi_exit);

    // 4. spectral_index, tensor_to_scalar_ratio を計算する
    let ns = spectral_index(eps, eta_val);
    let r = tensor_to_scalar_ratio(eps);

    // 5. Ok((ns, r)) を返す
    Ok((ns, r))
}