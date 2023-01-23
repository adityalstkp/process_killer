use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProcsConfig {
    pub name: String,
    pub expired: String,
}


pub fn parse_config(config_str: &str) -> Result<Vec<ProcsConfig>, serde_json::Error> {
    let deserialized_cfg:Vec<ProcsConfig> = serde_json::from_str(config_str)?;
    Ok(deserialized_cfg)
}
