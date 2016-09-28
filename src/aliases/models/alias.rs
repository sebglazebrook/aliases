use aliases::models::{Conditional, UserConfirmation};

use std::path::PathBuf;
use std::process::Command;
use regex::Regex;

#[derive(PartialOrd,Ord,PartialEq,Eq,Debug,Clone)]
pub struct Alias {
    pub name: String,
    pub command: String,
    pub command_arguments: Vec<String>,
    pub enable_positional_arguments: bool,
    pub confirm: bool,
    pub confirmation_message: String,
    pub conditional: Conditional,
    pub user_confirmation: UserConfirmation,
    pub delayed_backout: usize,
    pub unit_test: String,
    pub basename: PathBuf,
    pub quiet: bool,
}

impl Alias {

    pub fn new() -> Alias {
        Alias {
            name: String::new(),
            command: String::new(),
            confirm: false,
            confirmation_message: String::new(),
            user_confirmation: UserConfirmation::new(false, String::new()),
            delayed_backout: 0,
            conditional: Conditional::default(),
            unit_test: String::from("true"),
            basename: PathBuf::new(),
            command_arguments: vec![],
            enable_positional_arguments: false,
            quiet: false,
        }
    }

    pub fn execute(&self) {
        let mut command = Command::new("bash");
        command.arg("-c");
        command.arg(self.command());
        for argument in self.positional_arguments() {
            command.arg(argument);
        }

        let mut process = command.spawn().unwrap_or_else(|e| { panic!("failed to execute child: {}", e) });
        let _ = process.wait()
            .unwrap_or_else(|e| { panic!("failed to wait on child: {}", e) });
    }

    pub fn command(&self) -> String {
        let mut command = self.command.clone();
        for arg in self.command_arguments.clone().iter().skip(self.num_of_positional_arguments()) {
            if arg.contains(" ") {
                command = format!(r#"{} "{}""#, command, arg);
            } else {
                command = format!("{} {}", command, arg);
            }
        }
        command
    }

    //------------ private ---------//

    fn num_of_positional_arguments(&self) -> usize {
        if self.enable_positional_arguments {
            let re = Regex::new(r"\$(\d+)").unwrap();
            let something = re.captures_iter(&self.command).map(|capture| capture.at(1).unwrap().parse::<usize>().unwrap()).max().unwrap() + 1;
            something
        } else {
            0
        }
    }

    fn positional_arguments(&self) -> Vec<String> {
        let mut command = self.command.clone();
        self.command_arguments.clone().iter().take(self.num_of_positional_arguments()).map(|s| s.clone()).collect()
    }
}
