use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::read_to_string, path::Path};

#[derive(Deserialize, Serialize)]
pub struct RssConfig {
    ///channel name to url map
    pub channels: HashMap<String, String>,
}
///Load config from file, if file exists
///If no file passed, will default to checking for './.rrss.toml'
pub fn load_config(path: Option<String>) -> Result<Option<RssConfig>> {
    let config_file = if let Some(fp) = path {
        info!("config file: {}", fp);
        fp
    } else {
        info!("no config file supplied, defaulting to '.rrss.toml'");
        ".rrss.toml".to_string()
    };
    let maybe_config = match Path::new(&config_file).exists() {
        true => {
            info!("Found config file, parsing");
            //load the string from the file
            let toml_str = read_to_string(config_file)?;
            let cfg = toml::from_str(&toml_str)?;
            Some(cfg)
        }
        false => {
            info!("No config found");
            None
        }
    };
    Ok(maybe_config)
}

pub fn save_config(path: Option<String>, cfg: RssConfig) -> Result<()> {
    let config_file = if let Some(fp) = path {
        fp
    } else {
        ".rrss.toml".to_string()
    };
    info!("Saving config to {config_file}");
    let config_file_contents = toml::to_string(&cfg)?;
    info!("Saving cfg is  {config_file_contents}");
    std::fs::write(config_file, config_file_contents)?;
    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

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
        let two_cairns_found = cfg.channels.iter().any(|(k, v)| {
            println!("key={k}");
            return k == "Between Two Cairns" && v == "https://feeds.buzzsprout.com/2042709.rss";
        });
        let fobdf = cfg.channels.iter().any(|(k, v)| {
            println!("key={k}");
            return k == "Fear of a Black Dragon" && v == "http://feeds.libsyn.com/103241/rss";
        });
        std::fs::remove_file(test_file).unwrap();
        assert!(two_cairns_found, "Did not find Between Two Cairns");
        assert!(fobdf, "Did not find Fear of a Black Dragon");
    }

    #[test]
    pub fn test_save_config() {
        let mut cfg_map = HashMap::new();
        cfg_map.insert("First test".to_string(), "https://testing.test".to_string());
        let cfg = RssConfig { channels: cfg_map };
        let test_path = "test_file_save.toml".to_string();
        let res = save_config(Some(test_path.clone()), cfg);
        assert!(res.is_ok());
        let cfg = load_config(Some(test_path.clone())).unwrap().unwrap();
        assert!(cfg.channels.len() == 1);
        let cfg_found = cfg.channels.iter().any(|(k, v)| {
            println!("key={k}");
            return k == "First test" && v == "https://testing.test";
        });
        std::fs::remove_file(test_path.clone()).unwrap();
        assert!(cfg_found, "Did not find cfg");
    }
}
