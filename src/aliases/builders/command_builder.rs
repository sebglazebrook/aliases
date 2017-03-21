use std::process::Command;
use regex::Regex;

pub struct CommandBuilder {
    shell: String,
    command_name: Option<String>,
    args: Vec<String>,
}

impl CommandBuilder {

    pub fn using_bash() -> Self {
        CommandBuilder { shell: String::from("bash"), command_name: None, args: vec![] }
    }

    pub fn for_command(&mut self, command_name: &str) -> &mut Self {
        self.command_name = Some(command_name.to_string());
        self
    }

    pub fn with_args(&mut self, args: &Vec<String>) -> &mut Self {
        self.args = args.clone();
        self
    }

    pub fn build(&self) -> Command {
        let pseudo_command = self.pseudo_build();
        let mut command = Command::new(self.shell.clone());
        command.arg("-c");
        command.arg(pseudo_command.command_string.clone());
        for arg in pseudo_command.args {
            command.arg(arg.clone());
        }
        command
    }

    pub fn pseudo_build(&self) -> PseudoCommand {
        let args = self.build_args();
        let command_string = self.build_command_string();
        PseudoCommand { command_string: command_string, args: args }
    }

    //---------- private ------------//

    fn build_args(&self) -> Vec<String> {
        let num = self.total_numbered_positional_arguments(&self.command_name.clone().unwrap());
        let positional_arguments = self.args.clone().iter().take(num).map(|s| s.clone()).collect::<Vec<_>>();
        positional_arguments
    }

    fn build_command_string(&self) -> String {
        let mut command_string = self.command_name.clone().unwrap();
        let num = self.total_numbered_positional_arguments(&self.command_name.clone().unwrap());
        if self.catch_all() {
            let num = self.total_numbered_positional_arguments(&command_string);
            let remaining = self.args.clone().iter().skip(num).map(|s| s.clone()).collect::<Vec<_>>();
            let result = remaining.iter().fold(String::new(), |acc, ref arg| acc + &arg + " ").trim().to_string();
            command_string = command_string.replace("$@", &result);
        } else {
            let non_positional_args = self.args.clone().iter().skip(num).map(|s| s.clone()).collect::<Vec<_>>();
            for arg in &non_positional_args  {
                command_string = command_string + " " + arg;
            }
        }
        command_string
    }

    fn total_numbered_positional_arguments(&self, command: &String) -> usize {
        let re = Regex::new(r"\$(\d+)").unwrap();
        match re.captures_iter(command).map(|capture| capture.get(1).unwrap().as_str().parse::<usize>().unwrap()).max() {
            Some(value) => { value + 1 }
            None => 0
        }
    }

    fn catch_all(&self) -> bool {
        let re = Regex::new(r"\$@").unwrap();
        re.is_match(&self.command_name.clone().unwrap())
    }
}

pub struct PseudoCommand {
    pub command_string: String,
    pub args: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn without_args_it_builds_a_basic_command() {
        let command = CommandBuilder::using_bash().for_command("seb-test").pseudo_build();
        assert_eq!(command.command_string, "seb-test");
        assert_eq!(command.args.len(), 0);
    }

    #[test]
    fn with_args_it_appends_them_to_the_command() {
        let args = vec!["first".to_string(), "second".to_string()];
        let command = CommandBuilder::using_bash().for_command("seb-test").with_args(&args).pseudo_build();
        assert_eq!(command.command_string, "seb-test first second");
        assert_eq!(command.args.len(), 0);
    }

    #[test]
    fn when_command_references_positional_args_args_are_set_up() {
        let args = vec!["first".to_string(), "second".to_string()];
        let command = CommandBuilder::using_bash().for_command("ls $0 | grep $1").with_args(&args).pseudo_build();
        assert_eq!(command.command_string, "ls $0 | grep $1");
        assert_eq!(command.args, args);
    }

    #[test]
    fn when_command_references_positional_args_args_are_set_up_and_extra_are_appended_to_the_command() {
        let args = vec!["first".to_string(), "second".to_string(), "third".to_string(), "forth".to_string()];
        let command = CommandBuilder::using_bash().for_command("ls $0 | grep $1").with_args(&args).pseudo_build();
        assert_eq!(command.command_string, "ls $0 | grep $1 third forth");
        assert_eq!(command.args, vec!["first".to_string(), "second".to_string()]);
    }

    #[test]
    fn when_command_references_a_catch_all_args_all_args_are_injected_into_the_command_string() {
        let args = vec!["first".to_string(), "second".to_string()];
        let command = CommandBuilder::using_bash().for_command("ls $@ | grep something").with_args(&args).pseudo_build();
        assert_eq!(command.command_string, "ls first second | grep something");
        assert_eq!(command.args.len(), 0);
    }

    #[test]
    fn when_command_references_positional_args_and_a_catch_all_they_are_both_handled() {
        let args = vec!["first".to_string(), "second".to_string(), "third".to_string()];
        let command = CommandBuilder::using_bash().for_command("ls $@ | grep $0 | grep something").with_args(&args).pseudo_build();
        assert_eq!(command.command_string, "ls second third | grep $0 | grep something");
        assert_eq!(command.args, vec!["first"]);
    }
}
