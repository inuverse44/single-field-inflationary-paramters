use std::fs::{self, File};
use std::io::{Write};
use std::path::Path;

// ns_rクレート（lib.rsで公開されたモジュール）の各要素をインポート
use ns_r::{
    config::Config,
    models::create_potential,
    calculation::calculate_ns_r,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- ns-r diagram data generator ---");

    println!("[1/4] Loading configuration from config.toml...");
    let config_str = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    println!("Configuration loaded successfully.");

    println!("[3/4] Starting simulations...");
    for sim in &config.simulations {
        println!("\nRunning simulation: {}", sim.name);

        println!("  Preparing output file at: {}", sim.output_file);
        let output_path = Path::new(&sim.output_file);
        if let Some(parent_dir) = output_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }
        let mut file = File::create(output_path)?;
        writeln!(file, "simulation_name,parameter_name,parameter_value,ns,r")?;

        let scan = &sim.scan_parameter;
        let mut current_params = sim.fixed_parameters.clone();

        for i in 0..scan.steps {
            let param_value = if scan.steps == 1 {
                scan.start
            } else {
                scan.start + (scan.end - scan.start) * (i as f64) / ((scan.steps - 1) as f64)
            };
            current_params.insert(scan.name.clone(), param_value);

            print!("  Scanning {}: {:.4e} ... ", scan.name, param_value);

            let potential = match create_potential(&sim.model, &current_params) {
                Ok(p) => p,
                Err(e) => {
                    println!("Error creating potential: {}", e);
                    continue; // 次のパラメータへ
                }
            };

            let n_target = 60.0;
            let precision = 1e-6;
            match calculate_ns_r(potential.as_ref(), n_target, precision) {
                Ok((ns, r)) => {
                    writeln!(file, "{},{},{},{},{}", sim.name, scan.name, param_value, ns, r)?;
                    println!("OK -> (ns, r) = ({:.4}, {:.4e})", ns, r);
                }
                Err(e) => {
                    println!("Failed: {}", e);
                }
            }
        }
    }

    println!("\n[4/4] All simulations finished.");

    Ok(())
}