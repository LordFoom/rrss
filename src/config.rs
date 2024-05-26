use anyhow::Result;
use std::{collections::HashMap, path::Path};

struct Config {
    ///channel name to url map
    channels: HashMap<String, String>,
}
///Load config from file, if file exists
///If no file passed, will default to checking for './.rrss.toml'
pub fn load_config(path: Option<String>) -> Result<Option<Config>> {
    let config_file = if let Some(fp) = path {
        fp
    } else {
        ".rrss.toml".to_owned()
    };
    if Path::new(config_file).exists() {}
}
