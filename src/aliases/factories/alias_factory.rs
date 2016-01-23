use aliases::collections::Aliases;
use aliases::builders::AliasBuilder;

use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;
use std::result::Result;
use yaml_rust::{YamlLoader, Yaml};

pub struct AliasFactory;

impl AliasFactory {

    pub fn create_empty() -> Aliases {
        Aliases::new(vec![])
    }

    pub fn create_from_file(data_file: PathBuf) -> Result<Aliases, &'static str> {
        match AliasFactory::parse_file(&data_file) {
            Err(error_string) => { Err(error_string) }
            Ok(yaml) => {
                match yaml.as_hash() {
                    None => { Err("File invalid content.") },
                    Some(hash) => {
                        let mut aliases = vec![];
                        let basename = data_file.as_path().parent().unwrap().to_path_buf(); // handle this right??
                        for (command_name, command_args) in hash {
                            match AliasBuilder::from_yaml(command_name.as_str().unwrap(), basename.clone(), command_args.clone()).build() {
                                Err(_) => {}, // maybe get a more specific error here?
                                Ok(alias) => { aliases.push(alias) },
                            }
                        }
                        Ok(Aliases::new(aliases))
                    },
                }
            }
        }
    }

    pub fn create_from_files(data_files: Vec<PathBuf>) -> Aliases {
        let aliases = AliasFactory::create_empty();
        for data_file in data_files {
            match AliasFactory::create_from_file(data_file) {
                Err(_) => {},
                Ok(a) => { aliases.merge(a); }
            }
        }
        aliases
    }

    // ------------ private methods --------- //

    fn parse_file(data_file: &PathBuf) -> Result<Yaml, &'static str> {
        match File::open(data_file) {
            Err(_) => Err("File did not exist."),
            Ok(mut file) => {
                let mut file_contents = String::new();
                match file.read_to_string(&mut file_contents) {
                    Err(_) => Err("Error reading the file."),
                    Ok(_) => {
                        match YamlLoader::load_from_str(&file_contents) {
                            Ok(yaml) => Ok(yaml[0].clone()), // only expect 1 yaml document expect more in future??
                            Err(_) => Err("Could not parse the yaml.")
                        }
                    }
                }
            },
        }
    }
}
