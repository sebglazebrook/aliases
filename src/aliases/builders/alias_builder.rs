use yaml_rust::Yaml;

use aliases::models::Alias;

pub struct AliasBuilder {
    name: String,
    yaml: Yaml,
}

impl AliasBuilder {

    pub fn from_yaml(name: &str, yaml: Yaml) -> AliasBuilder {
        AliasBuilder { name: name.to_string(), yaml: yaml }
    }

    pub fn build(&self) -> Alias { // TODO this should return a result
        Alias {
            name: self.name.clone(),
            command: self.command(),
            confirm: self.confirm(),
            confirmation_message: self.confirmation_message(),
            unit_test: self.unit_test(),
            conditional: self.conditional(),
        }
    }

    // --------- private ---------//
    
    fn command(&self) -> String {
        match self.yaml["command"].as_str() {
            None => String::new(), // TODO handle this one!!!
            Some(string) => string.to_string()
        }
    }

    fn confirm(&self) -> bool {
        match self.yaml["confirm"].as_bool() {
            None => false,
            Some(b) => b
        }
    }

    fn confirmation_message(&self) -> String {
        match self.yaml["confirmation_message"].as_str() {
            None => self.default_confirmation_message(),
            Some(s) => s.to_string()
        }
    }

    fn default_confirmation_message(&self) -> String {
        let message = "About to execute `".to_string();
        message + &self.command() + "`"
    }

    fn conditional(&self) -> Option<String> {
        match self.yaml["conditional"].as_str() {
            None => None,
            Some(s) => Some(s.to_string())
        }
    }
    
    fn unit_test(&self) -> Option<String> {
        match self.yaml["unit_test"].as_str() {
            None => None,
            Some(s) => Some(s.to_string()),
        }
    }
}
