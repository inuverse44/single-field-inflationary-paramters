use crate::models::Potential;

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
