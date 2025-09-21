use crate::models::Potential;

pub struct NaturalPotential {
    pub v0: f64, 
    pub f: f64, 
}

impl Potential for NaturalPotential {
    fn v(&self, phi: f64) -> f64 {
        self.v0 * ( 1.0 - (phi / self.f).cos() )
    }

    fn p(&self, phi: f64) -> f64 {
        if phi == 0.0 { return 0.0; }
        self.v0 / self.f * (phi / self.f).sin()
    }

    fn p2(&self, phi: f64) -> f64 {
        self.v0 / self.f.powi(2) * (phi / self.f).cos()
    }
}
