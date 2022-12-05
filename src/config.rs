use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Formatted path string
    pub path: Option<String>,
}

impl Config {
    /// load the `.yaadv.ron` config file from pwd, returns default if fails to load.
    pub fn load() -> Self {
        let cfgs = [".yaadv.ron", ".yaadv"];
        let mut cfg_file = Path::new(cfgs[0]);
        for path in cfgs.iter().skip(1) {
            let cfg = Path::new(path);
            if cfg.is_file() {
                cfg_file = cfg;
                break;
            }
        }
        ron::from_str(fs::read_to_string(cfg_file).unwrap_or_default().as_str()).unwrap_or_default()
    }
}
