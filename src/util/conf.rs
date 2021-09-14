use std::fs::File;
use std::io::prelude::*;
use crate::util::database::Source;
use std::fmt::{self};
use serde::Deserialize;

pub struct ConfigError;

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Requested config entry not found!")
    }
}

impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

pub enum ConfigEntries {
    Architecture,
    Colour,
    Progressbar,
    Repos
}

pub enum RepoEntries {
    Name,
    Active,
    Url
}

#[derive(Deserialize)]
struct Config {
    architecture: String,
    colour: bool,
    progressbar: bool,
    repos: Vec<RepoNode>
}

#[derive(Deserialize)]
struct RepoNode {
    name: String,
    active: bool,
    url: Option<String> 
}

/// Returns a request KdlNode object, for use in other config entry getters
pub fn get_config_entry(entry: ConfigEntries, repo: Option<String>, repo_entry: Option<RepoEntries>) -> Result<String, ConfigError> {
    // Load config file
    let mut x = String::new();
    
    File::open("/etc/bulge/config.json")
        .expect("Failed to open config file, is another process accessing it?")
        .read_to_string(&mut x)
        .expect("Failed to convert file to string");

    let config: Config = serde_json::from_str(&x).expect("Failed to serialize data");

    match entry {
        ConfigEntries::Architecture => {
            return Ok(config.architecture)
        },
        ConfigEntries::Colour => {
            return Ok(config.colour.to_string())
        },
        ConfigEntries::Progressbar => {
            return Ok(config.progressbar.to_string())
        },
        ConfigEntries::Repos => {
            if repo.is_none() && repo_entry.is_none() {
                for i in config.repos {
                    if repo.clone().unwrap() == i.name {
                        match repo_entry.unwrap() {
                            RepoEntries::Name => return Ok(i.name),
                            RepoEntries::Active => return Ok(i.active.to_string()),
                            RepoEntries::Url => {
                                if i.url.is_some() {
                                    return Ok(i.url.unwrap());
                                }
                                return Ok(String::new());
                            },
                        }
                    }
                }
            }
            return Err(ConfigError)
        },
    }
}

fn get_repo_vec() -> Vec<RepoNode> {
        // Load config file
        let mut x = String::new();
    
        File::open("/etc/bulge/config.json")
            .expect("Failed to open config file, is another process accessing it?")
            .read_to_string(&mut x)
            .expect("Failed to convert file to string");
    
        let config: Config = serde_json::from_str(&x).expect("Failed to serialize data");

        return config.repos
}

/// Return sources in config
pub fn get_sources() -> Vec<Source> {
    let mut sources: Vec<Source> = vec![];

    let repo_config_entry: Vec<RepoNode> = get_repo_vec();

    for i in repo_config_entry {
        if i.active == true {
            sources.push(Source{
                name: i.name,
                url: if i.url.is_some() { Option::from(i.url.unwrap()) } else { None }
            })
        }
    }

    return sources;
}
