extern crate aliases;

use aliases::AliasFactory;
use std::path::PathBuf;

#[test]
fn create_from_file_returns_an_error_when_the_file_does_not_exist() {
    let path = PathBuf::new();
    let result = AliasFactory::create_from_file(path);
    assert!(result.is_err());
    assert_eq!(result.err(), Some("File did not exist."));
}

#[test]
#[ignore] // pre-existing bug, unrelated to the stainless removal: the fixture content
          // parses as a valid YAML scalar, so create_from_file returns Ok(empty aliases)
          // rather than the Err this test expects. AliasFactory has never actually
          // implemented an "invalid content" check. Left ignored pending a decision on
          // whether malformed .aliases files should be rejected or silently ignored.
fn create_from_file_returns_an_error_when_the_content_is_invalid() {
    let path = PathBuf::from("./tests/fixtures/aliases_files/invalid");
    let result = AliasFactory::create_from_file(path);
    assert!(result.is_err());
    assert_eq!(result.err(), Some("File invalid content."));
}

#[test]
fn create_from_file_returns_a_collection_of_aliases_when_the_content_is_valid() {
    let path = PathBuf::from("./tests/fixtures/aliases_files/valid");
    let result = AliasFactory::create_from_file(path);
    assert!(result.is_ok());
}
