
use std::collections::HashMap;

pub mod chaotic;
pub mod natural;

pub trait Potential {
    fn v(&self, phi: f64) -> f64;
    fn p(&self, phi: f64) -> f64;
    fn p2(&self, phi: f64) -> f64;
}

pub fn create_potential(model_name: &str, params: &HashMap<String, f64>) 
    -> Result<Box<dyn Potential>, String> {
    match model_name {
        "Chaotic" => {
            let v0 = *params.get("v0").ok_or("parameter 'v0' is missing for Chaotic model")?;
            let power = *params.get("power").ok_or("parameter 'power' is missing for Chaotic model")?;
            let potential = chaotic::potential::ChaoticPotential {v0, power};
            Ok(Box::new(potential))
        }, 
        "Natural" => {
            let v0 = *params.get("v0").ok_or("parameter 'v0' is missing for Natural inflation model")?;
            let f = *params.get("f").ok_or("parameter 'f' is missing for Natural inflation model")?;
            let potential = natural::potential::NaturalPotential {v0, f};
            Ok(Box::new(potential))
        }, 
        _ => Err(format!("Unknown model: {}", model_name)), 
    }
}