extern crate yaml_rust;

mod aliases;

use aliases::commands::init::Init;
use aliases::commands::list::List;
pub use aliases::builders::AliasBuilder; // had to do this for the tests, why?
pub use aliases::models::Alias; // had to do this for the tests, why?
pub use aliases::factories::AliasFactory; // had to do this for the tests, why?

use std::env;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::File;


pub struct App {
    pub current_path: PathBuf,
}

impl App {

    pub fn new() -> App {
        App { current_path: env::current_dir().unwrap() }
    }

    pub fn execute_init(&mut self, global: bool) {
        let target_path;
        if global {
            match env::var("HOME") {
                Ok(home_dir) => {
                   target_path = PathBuf::from(home_dir);
                },
                Err(e) => {
                    target_path = self.current_path.clone();
                },
            }
        } else {
            target_path = self.current_path.clone();
        }
        Init::new(target_path).execute();
    }

    pub fn execute_list(&mut self) {
        List::new(self.current_path.clone()).execute();
    }

    //pub fn execute_rehash(&mut self, global: bool) {
        //// TODO
    //}
}
