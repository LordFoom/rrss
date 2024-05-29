use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::read_to_string, path::Path};

use crate::model::App;

#[derive(Deserialize, Serialize)]
pub struct RssConfig {
    ///channel name to url map
    channels: HashMap<String, String>,
}
///Load config from file, if file exists
///If no file passed, will default to checking for './.rrss.toml'
pub fn load_config(path: Option<String>) -> Result<Option<RssConfig>> {
    let config_file = if let Some(fp) = path {
        fp
    } else {
        ".rrss.toml".to_string()
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
    let config_file = if let Some(fp) = path {
        fp
    } else {
        ".rrss.toml".to_string()
    };
    let mut channel_map = HashMap::new();
    app.channels.channels.iter().for_each(|channel| {
        channel_map.insert(channel.title.clone(), channel.get_link());
    });

    let config = RssConfig {
        channels: channel_map,
    };
    let config_file_contents = toml::to_string(&config)?;
    std::fs::write(config_file, config_file_contents)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    //TODO test to check config creation
    //
    #[test]
    pub fn test_load_config() {
        //create our test toml file
        let toml_str = r"[channels]
'Between Two Cairns'='https://feeds.buzzsprout.com/2042709.rss'
'Fear of a Black Dragon'='http://feeds.libsyn.com/103241/rss'";
        let test_file = "test_file.toml";
        std::fs::write(test_file, toml_str).unwrap();
        let cfg = load_config(Some(test_file.to_string())).unwrap().unwrap();
        assert!(cfg.channels.len() == 2);
        let two_cairns_found = cfg.channels.iter().any(|(k, _)| {
            println!("key={k}");
            return k == "Between Two Cairns";
        });
        let fobdf = cfg.channels.iter().any(|(k, _)| {
            println!("key={k}");
            return k == "Fear of a Black Dragon";
        });
        assert!(two_cairns_found, "Did not find Between Two Cairns");
        assert!(fobdf, "Did not find Fear of a Black Dragon");
    }
}
