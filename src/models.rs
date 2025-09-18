use crate::potential::{Potential, ChaoticPotential};
use std::collections::HashMap;

fn create_chaotic(params: &HashMap<String, f64>) -> Result<ChaoticPotential, String> {
    let v0 = *params.get("v0").ok_or("parameter 'v0' is missing for Chaotic model")?;
    let power = *params.get("power").ok_or("parameter 'power' is missing for Chaotic model")?;
    Ok(ChaoticPotential {v0, power})
}

pub fn create_potential(model_name: &str, params: &HashMap<String, f64>) 
    -> Result<Box<dyn Potential>, String> {
    match model_name {
        "Chaotic" => {
            let potential = create_chaotic(params)?;
            Ok(Box::new(potential))
        }, 
        _ => Err(format!("Unknown model: {}", model_name)), 
    }
}