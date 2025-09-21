/*
 * use crateはモジュールAからモジュールBを参照するときに使われる
 * プロジェクトルートからの絶対パスで別モジュールを指す
 * パスの記述が壊れにくい
 * ベストプラクティス
 */
use crate::models::Potential;
use crate::constants::M_P;

pub fn hubble_parameter(potential: &impl Potential, phi: f64, dot_phi: f64) -> f64 {
        ((0.5 * dot_phi.powi(2) + potential.v(phi)) / (3.0 * M_P)).sqrt()
}

pub fn epsilon(potential: &(impl Potential + ?Sized), phi: f64) -> f64 {
    0.5 * M_P.powi(2) * (potential.p(phi) / potential.v(phi)).powi(2)
}

pub fn eta(potential: &(impl Potential + ?Sized), phi: f64) -> f64 {
    M_P.powi(2) * potential.p2(phi) / potential.v(phi)
}

pub fn spectral_index(epsilon: f64, eta: f64) -> f64 {
    1.0 - 6.0 * epsilon + 2.0 * eta
}

pub fn tensor_to_scalar_ratio(epsilon: f64) -> f64 {
    16.0 * epsilon
}

// AAA pattern (Arrange, Act, Assert)
#[cfg(test)]
mod tests {
    use super::*; // 親モジュール（cosmology）のアイテムをすべてインポート
    use crate::potential::{ChaoticPotential};

    fn setup_potential() -> ChaoticPotential {
        ChaoticPotential { v0: 1.0, power: 2.0 }
    }

    #[test]
    fn test_hubble_parameter() {
        // ----- Arrange -----
        let potential = setup_potential();
        let phi = 10.0;
        let dot_phi: f64 = 1.0;
        let precision = 1e-9;
        let expected_h = ((0.5 * dot_phi.powi(2) + potential.v(phi)) / (3.0 * M_P)).sqrt();

        // ----- Act -----
        let actual_h = hubble_parameter(&potential, phi, dot_phi);

        // ----- Assert -----
        assert!((actual_h - expected_h).abs() < precision);
    }

    #[test]
    fn test_epsilon() {
        // ----- Arrange -----
        let potential = setup_potential();
        let phi = 10.0;
        let precision = 1e-9;
        let expected_epsilon = 0.5 * M_P.powi(2) * (potential.p(phi) / potential.v(phi)).powi(2);

        // ----- Act -----
        let actual_epsilon = epsilon(&potential, phi);

        // ----- Assert -----
        assert!((actual_epsilon - expected_epsilon).abs() < precision);
    }

    #[test]
    fn test_eta() {
        // ----- Arrange -----
        let potential = setup_potential();
        let phi = 10.0;
        let precision = 1e-9;
        let expected_eta = M_P.powi(2) * potential.p2(phi) / potential.v(phi);

        // ----- Act -----
        let actual_eta = eta(&potential, phi);

        // ----- Assert -----
        assert!((actual_eta - expected_eta).abs() < precision);
    }

    #[test]
    fn test_spectral_index() {
        // ----- Arrange -----
        let epsilon_val = 0.01;
        let eta_val = 0.02;
        let precision = 1e-9;
        let expected_ns = 1.0 - 6.0 * epsilon_val + 2.0 * eta_val;

        // ----- Act -----
        let actual_ns = spectral_index(epsilon_val, eta_val);

        // Assert
        assert!((actual_ns - expected_ns).abs() < precision);
    }

    #[test]
    fn test_tensor_to_scalar_ratio() {
        // ----- Arrange -----
        let epsilon_val = 0.01;
        let precision = 1e-9;
        let expected_r = 16.0 * epsilon_val;

        // ----- Act -----
        let actual_r = tensor_to_scalar_ratio(epsilon_val);

        // ----- Assert -----
        assert!((actual_r - expected_r).abs() < precision);
    }
}