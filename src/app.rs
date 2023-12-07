use serde_yaml;
use log::{info, debug, warn};
use serde::{Serialize, Deserialize};
use std::{path::PathBuf, str::FromStr};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RouteConfig {
    pub path: String,
    pub method: String,
    pub config: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RouterConfig {
    pub routes: Vec<RouteConfig>
}

impl From<PathBuf> for RouterConfig {
    fn from(path: PathBuf) -> Self {
        let mut file = std::fs::File::open(path.clone()).expect("Failed to open input file");
        let mut contents = String::new();
        std::io::Read::read_to_string(&mut file, &mut contents).expect("Failed to read input file");
        debug!("{}", &contents);
        serde_yaml::from_str(&contents).expect(
            format!(
                "Error while deserializing config file: {}",
                path.to_string_lossy()
            )
            .as_str(),
        )
    }
}


