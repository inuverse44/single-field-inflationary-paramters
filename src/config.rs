use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Config {
    pub simulations: Vec<Simulation>, 
}

#[derive(Deserialize, Clone)]
pub struct Simulation {
    pub name: String, 
    pub model: String, 
    pub output_file: String,
    pub scan_parameter: Scan, 
pub fixed_parameters: HashMap<String, f64>,
    pub solver_precision: f64,
    pub simpson_max_iter: usize,
}

#[derive(Deserialize, Clone)]
pub struct Scan {
    pub name: String, 
    pub start: f64, 
    pub end: f64, 
    pub steps: usize, 
}

#[derive(Deserialize, Clone)]
pub struct SearchRange {
    pub start: f64,
    pub end: f64,
}