extern crate aliases;

use aliases::{Aliases, Alias};

fn sample_aliases() -> (Alias, Alias, Alias, Alias) {
    let mut first = Alias::new();
    first.name = "first".to_string();
    let mut second = Alias::new();
    second.name = "second".to_string();
    let mut third = Alias::new();
    third.name = "third".to_string();
    let mut forth = Alias::new();
    forth.name = "forth".to_string();
    (first, second, third, forth)
}

#[test]
fn merge_returns_a_new_collection_with_the_aliases_from_both_when_there_are_no_duplicates() {
    let (first, second, third, forth) = sample_aliases();
    let subject = Aliases::new(vec![first.clone(), second.clone()]);
    let other = Aliases::new(vec![third.clone(), forth.clone()]);

    let result = subject.merge(other);

    assert_eq!(result, Aliases::new(vec![first, second, third, forth]));
}

#[test]
fn merge_returns_a_new_collection_without_duplicates_when_there_are_duplicates() {
    let (first, second, third, forth) = sample_aliases();
    let subject = Aliases::new(vec![first.clone(), second.clone()]);
    let other = Aliases::new(vec![first.clone(), third.clone(), forth.clone()]);

    let result = subject.merge(other);

    assert_eq!(result, Aliases::new(vec![first, second, third, forth]));
}
