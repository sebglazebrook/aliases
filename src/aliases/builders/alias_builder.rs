use yaml_rust::Yaml;
use std::path::PathBuf;

use aliases::models::Alias;
use aliases::models::Conditional;
use aliases::models::UserConfirmation;

pub struct AliasBuilder {
    name: String,
    basename: PathBuf,
    yaml: Yaml,
}

impl AliasBuilder {

    pub fn from_yaml(name: &str, basename: PathBuf, yaml: Yaml) -> AliasBuilder {
        AliasBuilder { name: name.to_string(), basename: basename, yaml: yaml }
    }

    pub fn build(&self) -> Result<Alias,&'static str> {
        match self.command() {
            None => { Err("No command was given") }
            Some(command) => {
                Ok(Alias {
                    name: self.name.clone(),
                    command: command.clone(),
                    confirm: self.confirm(),
                    confirmation_message: self.confirmation_message(command.clone()),
                    user_confirmation: UserConfirmation::new(self.confirm(), self.confirmation_message(command)),
                    unit_test: self.unit_test(),
                    conditional: self.conditional(),
                    basename: self.basename.clone(),
                })
            }
        }
    }

    // --------- private ---------//
    
    fn command(&self) -> Option<String> {
        match self.yaml["command"].as_str() {
            None => None,
            Some(string) => Some(string.to_string())
        }
    }

    fn confirm(&self) -> bool {
        match self.yaml["confirm"].as_bool() {
            None => false,
            Some(b) => b
        }
    }

    fn confirmation_message(&self, command: String) -> String {
        match self.yaml["confirmation_message"].as_str() {
            None => self.default_confirmation_message(&command),
            Some(s) => s.to_string()
        }
    }

    fn default_confirmation_message(&self, command: &String) -> String {
        let message = "About to execute `".to_string();
        message + &command + "`"
    }

    fn conditional(&self) -> Conditional {
        match self.yaml["conditional"].as_str() {
            None => Conditional::new(String::from("true")),
            Some(s) => Conditional::new(s.to_string())
        }
    }
    
    fn unit_test(&self) -> String {
        match self.yaml["unit_test"].as_str() {
            None => String::from("true"),
            Some(s) => s.to_string(),
        }
    }
}
