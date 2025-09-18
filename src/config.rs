use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Config {
    pub output_file: String, 
    pub simulations: Vec<Simulation>, 
}

#[derive(Deserialize)]
pub struct Simulation {
    pub name: String, 
    pub model: String, 
    pub scan_parameter: Scan, 
    pub fixed_parameters: HashMap<String, f64>, 
}

#[derive(Deserialize)]
pub struct Scan {
    pub name: String, 
    pub start: f64, 
    pub end: f64, 
    pub steps: usize, 
}

