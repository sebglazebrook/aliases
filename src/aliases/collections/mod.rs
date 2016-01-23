use aliases::models::Alias;
use std::cmp::Ordering;
use std::result::Result;

#[derive(Debug,Clone)]
pub struct Aliases {
    pub raw_collection: Vec<Alias>,
    iteration_index: usize,
}

impl Aliases {

    pub fn new(raw_collection: Vec<Alias>) -> Aliases {
        Aliases { raw_collection: raw_collection, iteration_index: 0 }
    }

    pub fn merge(&self, other: Aliases) -> Aliases {
        let mut merged_aliases = self.clone();
        for other_alias in other.raw_collection.iter() {
            let _ = merged_aliases.push(other_alias);
        }
        merged_aliases
    }

    pub fn push(&mut self, alias: &Alias) -> Result<(), &'static str> {
        if self.raw_collection.contains(alias) {
            Err("Alias was a duplicate")
        } else {
            self.raw_collection.push(alias.clone());
            Ok(())
        }
    }
}

impl PartialEq for Aliases {

    fn eq(&self, other: &Self) -> bool {
        self.raw_collection.iter().cmp(other.raw_collection.iter()) == Ordering::Equal
    }
}
