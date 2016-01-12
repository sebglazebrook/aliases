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

    pub fn build(&self) -> Result<Alias,&'static str> {
        match self.command() {
            None => { Err("No command was given") }
            Some(command) => {
                Ok(Alias {
                    name: self.name.clone(),
                    command: command.clone(),
                    confirm: self.confirm(),
                    confirmation_message: self.confirmation_message(command),
                    unit_test: self.unit_test(),
                    conditional: self.conditional(),
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
