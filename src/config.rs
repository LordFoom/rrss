use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::read_to_string, path::Path};

use crate::model::App;

#[derive(Deserialize, Serialize)]
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
    let maybe_config = match Path::new(&config_file).exists() {
        true => {
            //load the string from the file
            let toml_str = read_to_string(config_file)?;
            let cfg = toml::from_str(&toml_str)?;
            Some(cfg)
        }
        false => None,
    };
    Ok(maybe_config)
}

pub fn save_config(path: Option<String>, app: &App) -> Result<()> {
    let mut channel_map = HashMap::new();
    app.channels
        .channels
        .iter()
        .for_each(|channel| {
            channel_map.insert(channel.title, channel.get_link());
        } );
    //TODO actual saving


    Ok(())

}

#[cfg(test)]
mod test {
    use super::*;
    //TODO test to check config creation
    //
    pub fn test_load_config() {
        //create our test toml file
        let toml_str = r"[channels]
Between Two Cairns"
    }
}
