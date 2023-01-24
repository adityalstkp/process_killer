use core::fmt;
use std::error::Error;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProcsConfig {
    pub name: String,
    pub expired: String,

    pub expired_seconds: Option<u64>,
}

#[derive(Debug)]
pub struct ConfigError(String);

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ConfigError: {}", self.0)
    }
}

impl Error for ConfigError {}


impl ProcsConfig {
    pub fn parse_expired_unit(&mut self) -> Result<(), ConfigError> {
       let match_unit = self.expired.chars().last();
       let last_idx = self.expired.len();
       let n: u64 = self.expired[..last_idx - 1].parse().unwrap();
       if let Some(u) = match_unit {
           if u == 'h' {
               self.expired_seconds = Some(n * 3600);
               return Ok(())
           } else if u == 'm' {
               self.expired_seconds = Some(n * 60);
               return Ok(())
           } else if u == 's' {
               self.expired_seconds = Some(n);
               return Ok(())
           }
       }

       Err(ConfigError("not recognizeable unit".into()))
    }
}

// parse config based on JSON string
pub fn parse_config(config_str: &str) -> Result<Vec<ProcsConfig>, serde_json::Error> {
    let mut deserialized_cfg:Vec<ProcsConfig> = serde_json::from_str(config_str)?;
    for c in deserialized_cfg.iter_mut() {
        c.parse_expired_unit().expect("cannot parse unit");
    }

    Ok(deserialized_cfg)
}
