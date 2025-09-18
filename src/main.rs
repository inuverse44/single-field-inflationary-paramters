use ns_r::potential::{ChaoticPotential};
use ns_r::solver::{find_phi_end, find_phi_exit};
use ns_r::cosmology::{epsilon, eta, spectral_index, tensor_to_scalar_ratio};

fn main() {
    println!("--- Inflationary Parameters Calculation ---");

    // --- 物理モデルのパラメータ設定 ---
    let potential = ChaoticPotential{ m: 1.0, power: 2.0 };

    // --- 数値計算のパラメータ設定 ---
    let precision = 1e-6; // 計算精度
    let n_target = 60.0;    // 計算対象のe-fold数

    // find_phi_end を呼び出し、インフレーションの終点を探す
    let phi_end_search_range = (20.0, 1.0);
    let phi_end = match find_phi_end(&potential, phi_end_search_range, precision) {
        Ok(phi) => {
            println!("Inflation ends at phi_end = {}", phi);
            phi
        },
        Err(e) => {
            println!("[Error] Could not find phi_end: {}", e);
            return;
        }
    };

    // find_phi_exit を呼び出し、e-fold=60となる点を逆算する
    let phi_exit_search_range = (phi_end + 0.1, 30.0); 
    let phi_exit = match find_phi_exit(&potential, phi_end, n_target, phi_exit_search_range, precision) {
        Ok(phi_exit_val) => {
            println!("Horizon exit (N={}) occurs at phi_exit = {}", n_target, phi_exit_val);
            phi_exit_val
        },
        Err(e) => {
            println!("[Error] Could not find phi_exit: {}", e);
            return;
        }
    };

    // スローロールパラメータと、そこから導出される物理量を計算
    let e = epsilon(&potential, phi_exit);
    let h = eta(&potential, phi_exit);
    println!("epsilon = {}", e);
    println!("eta = {}", h);

    let ns = spectral_index(e, h);
    let r = tensor_to_scalar_ratio(e);
    println!("spectral index = {}", ns);
    println!("tensor to scalar ratio = {}", r);
}