use aliases::collections::Aliases;
use aliases::models::Alias;
use aliases::builders::AliasBuilder;

use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;
use yaml_rust::{YamlLoader, Yaml};

struct ParseError;

pub struct AliasFactory;

impl AliasFactory {

    pub fn create_empty() -> Aliases {
        Aliases::new(vec![])
    }

    pub fn create_from_file(data_file: PathBuf) -> Aliases {
        match AliasFactory::parse_file(data_file) {
            Err(_) => { AliasFactory::create_empty() }
            Ok(yaml) => {
                match yaml.as_vec() {
                    // TODO remove the hardcoding from the below
                    None => { Aliases::new(vec![AliasBuilder::from_yaml("todo", yaml.clone()).build()]) },
                    Some(aliases_yaml) => {
                        let mut aliases = vec![];
                        for alias_yaml in aliases_yaml {
                            // TODO remove the hardcoding from the below
                            aliases.push(AliasBuilder::from_yaml("todo", alias_yaml.clone()).build()); // remove empties? handle empties?
                        }
                        Aliases::new(aliases)
                    },
                }
            }
        }
    }

    pub fn create_from_files(data_files: Vec<PathBuf>) -> Aliases {
        let mut aliases = AliasFactory::create_empty();
        for data_file in data_files {
            aliases.merge(AliasFactory::create_from_file(data_file));
        }
        aliases
    }

    // ------------ private methods --------- //

    fn parse_file(data_file: PathBuf) -> Result<Yaml, ParseError> {
        match File::open(data_file) {
            Err(_) => Err(ParseError),
            Ok(mut file) => {
                let mut file_contents = String::new();
                match file.read_to_string(&mut file_contents) {
                    Err(_) => Err(ParseError),
                    Ok(_) => {
                        match YamlLoader::load_from_str(&file_contents) {
                            Ok(yaml) => Ok(yaml[0].clone()), // only expect 1 yaml document expect more in future??
                            Err(_) => Err(ParseError)
                        }
                    }
                }
            },
        }
    }
}
