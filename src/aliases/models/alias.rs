use aliases::models::{Conditional, UserConfirmation};
use aliases::builders::CommandBuilder;

use std::path::PathBuf;

#[derive(PartialOrd,Ord,PartialEq,Eq,Debug,Clone)]
pub struct Alias {
    pub name: String,
    pub command: String,
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
        // TODO make the print out the expanded command with args
        self.command.clone()
    }

    pub fn add_arguments(&mut self, arguments: Vec<String>) {
        self.args = arguments.clone();
    }
}
