// n階微分をどう表現するか課題
pub trait Potential {
    fn value(&self, phi: f64) -> f64;
    fn prime(&self, phi: f64) -> f64;
    fn double_prime(&self, phi: f64) -> f64;
}

pub struct ChaoticPotential {
    pub m: f64,
    pub power: f64,
}

impl Potential for ChaoticPotential {
    fn value(&self, phi: f64) -> f64 {
        0.5 * self.m.powi(2) * phi.powi(2)
    }

    fn prime(&self, phi: f64) -> f64 {
        self.m.powi(2) * phi
    }

    fn double_prime(&self, _phi: f64) -> f64 {
        self.m.powi(2)
    }
}


// AAA pattern (Arrange, Act, Assert)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chaotic_potential_methods() {
        // ----- Arrange ----- 
        let m = 2.0;
        let phi: f64 = 10.0;
        let precision = 1e-9;
        let potential = ChaoticPotential { m, power: 2.0 };

        // V = 0.5 * m^2 * phi^2 に基づいて期待値を計算
        let expected_value = 0.5 * m.powi(2) * phi.powi(2);
        // V' = dV/d(phi) = m^2 * phi
        let expected_prime = m.powi(2) * phi;
        // V'' = d^2V/d(phi)^2 = m^2
        let expected_double_prime = m.powi(2);

        // ----- Act -----
        let actual_value = potential.value(phi);
        let actual_prime = potential.prime(phi);
        let actual_double_prime = potential.double_prime(phi);

        // ----- Assert -----
        assert!((actual_value - expected_value).abs() < precision);
        assert!((actual_prime - expected_prime).abs() < precision);
        assert!((actual_double_prime - expected_double_prime).abs() < precision);
    }
}