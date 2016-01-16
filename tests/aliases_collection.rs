#![feature(plugin,const_fn)]
#![plugin(stainless)]

extern crate aliases;

#[cfg(test)]
mod tests {

    pub use aliases::{Aliases, Alias};

    describe! merge {

        before_each {
            let mut first = Alias::new();
            first.name = "first".to_string();
            let mut second = Alias::new();
            second.name = "second".to_string();
            let mut third = Alias::new();
            third.name = "third".to_string();
            let mut forth = Alias::new();
            forth.name = "forth".to_string();
            let subject = Aliases::new(vec![first.clone(), second.clone()]);
        }

        describe! when_there_are_no_duplicates {

            before_each {
                let other = Aliases::new(vec![third.clone(), forth.clone()]);
                let result = subject.merge(other);
            }

            it "returns a new collection with the aliases from both" {
                assert_eq!(result, Aliases::new(vec![first.clone(), second.clone(), third.clone(), forth.clone()]))
            }
        }

        describe! when_there_are_duplicates {

            before_each {
                let other = Aliases::new(vec![first.clone(), third.clone(), forth.clone()]);
                let result = subject.merge(other);
            }

            it "returns a new collection without duplicates" {
                assert_eq!(result, Aliases::new(vec![first.clone(), second.clone(), third.clone(), forth.clone()]));
            }
        }
    }
}
