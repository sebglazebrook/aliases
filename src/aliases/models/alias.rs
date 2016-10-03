use aliases::models::{Conditional, UserConfirmation};
use aliases::builders::CommandBuilder;

use std::path::PathBuf;

#[derive(PartialOrd,Ord,PartialEq,Eq,Debug,Clone)]
pub struct Alias {
    pub name: String,
    pub command: String,
    pub command_arguments: Vec<String>,
    pub positional_arguments: Vec<String>,
    pub args: Vec<String>,
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
            positional_arguments: vec![],
            args: vec![],
            enable_positional_arguments: false,
            quiet: false,
        }
    }

    pub fn execute(&self) {
        let mut command = CommandBuilder::using_bash()
                                    .for_command(&self.command)
                                    .with_args(&self.args)
                                    .build();

        let mut process = command.spawn().unwrap_or_else(|e| { panic!("failed to execute child: {}", e) });
        let _ = process.wait()
            .unwrap_or_else(|e| { panic!("failed to wait on child: {}", e) });
    }

    pub fn command(&self) -> String {
        // TODO update this
        let mut command = self.command.clone();
        if self.positional_arguments.len() > 0 {
            command = command.replace("$@", self.positional_arguments.last().unwrap());
        }
        for arg in self.command_arguments.clone().iter().skip(self.positional_arguments.len()) {
            if arg.contains(" ") {
                command = format!(r#"{} "{}""#, command, arg);
            } else {
                command = format!("{} {}", command, arg);
            }
        }
        command
    }

    pub fn add_arguments(&mut self, arguments: Vec<String>) {
        self.args = arguments.clone();
    }
}
