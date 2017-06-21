#[macro_use] extern crate serde_derive;
extern crate toml;

pub mod economy; // Economy model of bitcoin blockchain

use std::collections::HashMap;
use economy::Economy;

#[derive(Serialize, Deserialize)]
enum ValidationRules {
    Current,
    Simple,
    Multilevel(u64)
}

#[derive(Serialize, Deserialize)]
pub struct Simulation {
    pub validation_rules: ValidationRules,
    #[serde(serialize_with = "toml::ser::tables_last")]
    pub hashrate_distribution: HashMap<String, u64>, // (owner, hashrate)
}

fn main() {
    // Command line interface specification and minimalistic help

    // Load general settings, consensus parameters, initial conditions
    
    // Numerical simulation of bitcoin network economy model over time
    loop {

        //Stop criteria
        break; // Unconditional
    }

}
