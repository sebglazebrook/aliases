use aliases::models::Alias;
use std::cmp::Ordering;
use std::result::Result;

#[derive(Debug,Clone)]
pub struct Aliases {
    raw_collection: Vec<Alias>,
    iteration_index: usize,
}

impl Aliases {

    pub fn new(raw_collection: Vec<Alias>) -> Aliases {
        Aliases { raw_collection: raw_collection, iteration_index: 0 }
    }

    pub fn merge(&self, others: Aliases) -> Aliases {
        let mut merged_aliases = self.clone();
        for other_alias in others {
            let _ = merged_aliases.push(&other_alias);
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

    pub fn len(&self) -> usize {
        self.raw_collection.len()
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
