use aliases::models::{Conditional, UserConfirmation};

use std::path::PathBuf;
use std::process::Command;

#[derive(PartialOrd,Ord,PartialEq,Eq,Debug,Clone)]
pub struct Alias {
    pub name: String,
    pub command: String,
    pub command_arguments: Vec<String>,
    pub confirm: bool,
    pub confirmation_message: String,
    pub conditional: Conditional,
    pub user_confirmation: UserConfirmation,
    pub delayed_backout: usize,
    pub unit_test: String,
    pub basename: PathBuf,
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
        }
    }

    pub fn execute(&self) {
        println!("Executing: {}", self.command());
        let mut process = Command::new("bash")
            .arg("-c")
            .arg(self.command())
            .spawn()
            .unwrap_or_else(|e| { panic!("failed to execute child: {}", e) });

        let _ = process.wait()
            .unwrap_or_else(|e| { panic!("failed to wait on child: {}", e) });
    }

    pub fn command(&self) -> String {
        let mut command = self.command.clone();
        for arg in self.command_arguments.clone() {
            command = command + " " + &arg;
        }
        command
    }
}
