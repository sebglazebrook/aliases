extern crate aliases;
extern crate yaml_rust;

use yaml_rust::YamlLoader;
use aliases::{Alias, AliasBuilder, Conditional};
use std::path::PathBuf;

#[test]
fn from_yaml_creates_an_alias_with_all_the_fields_set_when_all_fields_are_included() {
    let basename = PathBuf::new();
    let yaml_string =
"command: ./super_command.sh
confirm: true
confirmation_message: Are you really really sure??
conditional: /bin/true
backout_seconds: 3
unit_test: '[ true = true ]'
quiet: true
";
    let docs = YamlLoader::load_from_str(yaml_string).unwrap();
    let doc = &docs[0];

    let mut alias = Alias::new();
    match AliasBuilder::from_yaml("command_name", basename.clone(), doc.clone()).build() {
        Ok(a) => { alias = a; },
        Err(_) => { },
    };

    assert_eq!(alias.name, "command_name");
    assert_eq!(alias.command, "./super_command.sh");
    assert_eq!(alias.confirm, true);
    assert_eq!(alias.confirmation_message, "Are you really really sure??");
    assert_eq!(alias.conditional, Conditional::new("/bin/true".to_string()));
    assert_eq!(alias.delayed_backout, 3);
    assert_eq!(alias.unit_test, "[ true = true ]".to_string());
    assert_eq!(alias.quiet, true);
}

#[test]
fn from_yaml_builds_with_confirmation_turned_off_when_confirm_is_absent() {
    let basename = PathBuf::new();
    let yaml_string =
"command: ./super_command.sh
unit_test: '[ true == true ]'
conditional: /bin/true
confirmation_message: Are you really really sure??
";
    let docs = YamlLoader::load_from_str(yaml_string).unwrap();
    let doc = &docs[0];

    let mut alias = Alias::new();
    match AliasBuilder::from_yaml("command_name", basename.clone(), doc.clone()).build() {
        Err(_) => {},
        Ok(a) => { alias = a; }
    }
    assert_eq!(alias.confirm, false);
}

#[test]
fn from_yaml_builds_with_a_default_confirmation_message_when_confirmation_message_is_absent() {
    let basename = PathBuf::new();
    let yaml_string =
"command: ./super_command.sh
confirm: true
unit_test: '[ true == true ]'
conditional: /bin/true
";
    let docs = YamlLoader::load_from_str(yaml_string).unwrap();
    let doc = &docs[0];

    let mut alias = Alias::new();
    match AliasBuilder::from_yaml("command_name", basename.clone(), doc.clone()).build() {
        Err(_) => {},
        Ok(a) => { alias = a; },
    }
    assert_eq!(alias.confirmation_message, "About to execute `./super_command.sh`");
}

#[test]
fn from_yaml_builds_without_a_conditional_when_conditional_is_absent() {
    let basename = PathBuf::new();
    let yaml_string =
"command: ./super_command.sh
confirm: true
confirmation_message: Are you really really sure??
unit_test: '[ true == true ]'
";
    let docs = YamlLoader::load_from_str(yaml_string).unwrap();
    let doc = &docs[0];

    assert!(AliasBuilder::from_yaml("command_name", basename.clone(), doc.clone()).build().is_ok());
}

#[test]
fn from_yaml_builds_without_a_unit_test_when_unit_test_is_absent() {
    let basename = PathBuf::new();
    let yaml_string =
"command: ./super_command.sh
confirm: true
confirmation_message: Are you really really sure??
conditional: /bin/true
";
    let docs = YamlLoader::load_from_str(yaml_string).unwrap();
    let doc = &docs[0];

    assert!(AliasBuilder::from_yaml("command_name", basename.clone(), doc.clone()).build().is_ok());
}

#[test]
fn from_yaml_builds_without_a_delayed_backout_when_backout_seconds_is_absent() {
    let basename = PathBuf::new();
    let yaml_string =
"command: ./super_command.sh
confirm: true
confirmation_message: Are you really really sure??
conditional: /bin/true
";
    let docs = YamlLoader::load_from_str(yaml_string).unwrap();
    let doc = &docs[0];

    assert!(AliasBuilder::from_yaml("command_name", basename.clone(), doc.clone()).build().is_ok());
}
