extern crate aliases;

use aliases::{AliasCommand, Rehash, ShimFileFactory};
use std::env;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

fn rehash_fixture() -> (Rehash, PathBuf) {
    let current_dir = env::current_dir().unwrap();
    let shim_directory = current_dir.join("tests/fixtures/shims/");
    let rehash = Rehash::new(shim_directory.clone(), vec![current_dir.join("tests/fixtures/initialized_dir/")]);
    (rehash, shim_directory)
}

#[test]
fn execute_generates_a_shim_when_there_is_no_global_shim_for_an_alias() {
    let (rehash, shim_directory) = rehash_fixture();
    let _ = fs::remove_file(shim_directory.join("test-command"));

    rehash.execute();

    assert!(shim_directory.join("test-command").as_path().exists());
    assert!(ShimFileFactory::is_valid(&shim_directory.join("test-command")));
}

#[test]
fn execute_updates_the_shim_when_the_alias_has_changed() {
    let (rehash, shim_directory) = rehash_fixture();
    let _ = File::create(shim_directory.join("test-command"));

    rehash.execute();

    assert!(ShimFileFactory::is_valid(&shim_directory.join("test-command")));
}
