use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShardbearerConfig {
    my_ip: String,
    my_port: u16,
    neighbor_ip: String,
    neighbor_port: u16,
    bootstrap_backoff: u64,
}

impl ShardbearerConfig {
    pub fn my_port(&self) -> u16 {
        self.my_port
    }
    pub fn my_ip(&self) -> String {
        self.my_ip.clone()
    }
    pub fn neighbor_port(&self) -> u16 {
        self.neighbor_port
    }
    pub fn neighbor_ip(&self) -> String {
        self.neighbor_ip.clone()
    }

    pub fn bootstrap_backoff(&self) -> u64 {
        self.bootstrap_backoff
    }
}

//#[derive(Serialize, Deserialize)]
//pub struct NeighborConfig{

//}

pub fn parse_cfg(file: &str) -> Result<ShardbearerConfig, Box<dyn Error>> {
    let mut fp = match File::open(file) {
        Err(why) => {
            println!(
                "Could not open provided toml: {}: {}",
                file,
                why.description()
            );
            return Err(Box::new(why));
        }
        Ok(fp) => fp,
    };

    let mut ops = vec![];
    fp.read_to_end(&mut ops)?;

    let cfg = toml::from_slice(&ops)?;
    println!("CONFIG!: {:?}", cfg);
    Ok(cfg)
}
