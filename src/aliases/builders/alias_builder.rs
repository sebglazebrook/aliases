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

    pub fn build(&self) -> Result<Alias, &'static str> {
        match self.command() {
            None => { Err("No command given. Each alias needs a command.") }
            Some(command) => {
                Ok(Alias {
                    name: self.name.clone(),
                    command: command.clone(),
                    confirm: self.confirm(),
                    confirmation_message: self.confirmation_message(command.clone()),
                    user_confirmation: UserConfirmation::new(self.confirm(), self.confirmation_message(command)),
                    delayed_backout: self.delayed_backout(),
                    unit_test: self.unit_test(),
                    conditional: self.conditional(),
                    basename: self.basename.clone(),
                    command_arguments: vec![],
                    positional_arguments: vec![],
                    args: vec![],
                    enable_positional_arguments: self.enable_positional_arguments(),
                    quiet: self.quiet(),
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

    fn delayed_backout(&self) -> usize {
        match self.yaml["backout_seconds"].as_i64() {
            None => 0,
            Some(seconds) => seconds.abs() as usize
        }
    }
    
    fn unit_test(&self) -> String {
        match self.yaml["unit_test"].as_str() {
            None => String::from("true"),
            Some(s) => s.to_string(),
        }
    }

    fn quiet(&self) -> bool {
        match self.yaml["quiet"].as_bool() {
            None => false,
            Some(value) => value,
        }
    }

    fn enable_positional_arguments(&self) -> bool {
        match self.yaml["enable_positional_arguments"].as_bool() {
            None => false,
            Some(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yaml_rust::YamlLoader;
    use std::path::PathBuf;

    #[test]
    fn extracts_the_command_string_from_the_yaml() {
        let string =
"
command: test-command
";
        let yaml = YamlLoader::load_from_str(string).unwrap();
        let document = yaml[0].clone();
        let builder = AliasBuilder::from_yaml("test", PathBuf::new(), document);
        let alias = builder.build().unwrap();
        assert!(alias.command == "test-command");
    }

    #[test]
    fn enables_the_positional_arguments_if_set_in_yaml() {
        let string =
"
command: test-command
enable_positional_arguments: true
";
        let yaml = YamlLoader::load_from_str(string).unwrap();
        let document = yaml[0].clone();
        let builder = AliasBuilder::from_yaml("test", PathBuf::new(), document);
        let alias = builder.build().unwrap();
        assert!(alias.enable_positional_arguments == true);
    }

}
