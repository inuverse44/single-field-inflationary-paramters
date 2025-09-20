pub trait Potential {
    fn v(&self, phi: f64) -> f64;
    fn p(&self, phi: f64) -> f64;
    fn p2(&self, phi: f64) -> f64;
}

pub struct ChaoticPotential {
    pub v0: f64,
    pub power: f64,
}

impl Potential for ChaoticPotential {
    fn v(&self, phi: f64) -> f64 {
        self.v0 * phi.powf(self.power)
    }

    fn p(&self, phi: f64) -> f64 {
        if phi == 0.0 && self.power <= 1.0 { return 0.0; }
        self.v0 * self.power * phi.powf(self.power - 1.0)
    }

    fn p2(&self, phi: f64) -> f64 {
        if phi == 0.0 && self.power <= 2.0 { return 0.0; }
        self.v0 * self.power * (self.power - 1.0) * phi.powf(self.power - 2.0)
    }
}


// AAA pattern (Arrange, Act, Assert)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chaotic_potential_p2() {
        // Arange
        let potential = ChaoticPotential { v0: 1.0, power: 2.0 };
        let phi = 10.0;

        // Act
        // Assert
        assert_eq!(potential.v(phi), 100.0);
        assert_eq!(potential.p(phi), 20.0);
        assert_eq!(potential.p2(phi), 2.0);
    }

    #[test]
    fn test_chaotic_potential_p4() {
        // Arange
        let potential = ChaoticPotential { v0: 1.0, power: 4.0 };
        let phi = 10.0;

        // Act
        // Assert
        assert_eq!(potential.v(phi), 10000.0);
        assert_eq!(potential.p(phi), 4000.0);
        assert_eq!(potential.p2(phi), 1200.0);
    }
}