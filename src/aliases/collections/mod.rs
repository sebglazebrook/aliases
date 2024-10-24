use aliases::models::Alias;
use std::cmp::Ordering;
use std::result::Result;
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug,Clone)]
pub struct Aliases {
    raw_collection: Vec<Alias>,
    iteration_index: usize,
}

impl Aliases {

    pub fn new(raw_collection: Vec<Alias>) -> Aliases {
        Aliases { raw_collection, iteration_index: 0 }
    }

    pub fn merge(&self, others: Aliases) -> Aliases {
        let mut merged_aliases = self.clone();
        for other_alias in others {
            let _ = merged_aliases.push(&other_alias);
        }
        merged_aliases
    }

    pub fn remove(&mut self, alias: &Alias) -> Result<(), &'static str> {
      match self.raw_collection.iter().position(|a| a.name == alias.name) {
        Some(n) => {
          self.raw_collection.remove(n);
          Ok(())
        },
        None => { Err("Alias does not exist") }
      }
    }

    pub fn push(&mut self, alias: &Alias) -> Result<(), &'static str> {
        if self.raw_collection.iter().any(|a| { a.name == alias.name }) {
            Err("Alias was a duplicate")
        } else {
            self.raw_collection.push(alias.clone());
            Ok(())
        }
    }

    pub fn len(&self) -> usize {
        self.raw_collection.len()
    }

    pub fn to_yaml(&self) -> Yaml {
        let mut yaml_string = String::from("");
        self.raw_collection.iter().fold(&mut yaml_string, |acc,  ref alias| {
            acc.push_str(&alias.as_yaml());
            acc.push_str("\n");
            return acc;
        });
        match YamlLoader::load_from_str(&yaml_string).unwrap().get(0) {
            Some(value) => { value.clone() },
            None => { Yaml::from_str("") }
        }
    }
}

impl PartialEq for Aliases {

    fn eq(&self, other: &Self) -> bool {
        self.raw_collection.cmp(&other.raw_collection) == Ordering::Equal
    }
}

impl Iterator for Aliases {

    type Item = Alias;

    fn next(&mut self) -> Option<Alias> {
        if self.iteration_index < self.raw_collection.len()  {
            let alias = Some(self.raw_collection[self.iteration_index].clone());
            self.iteration_index += 1;
            alias
        } else {
            self.iteration_index = 0;
            None
        }
    }
}

#[test]
fn when_empty_it_build_empty_yaml() {
    let collection = Aliases::new(vec![]);
    let result = collection.to_yaml();
    assert_eq!(result, Yaml::from_str(""));
}
