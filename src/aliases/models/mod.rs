use std::path::PathBuf;
use std::process::Command;

#[derive(PartialOrd,Ord,PartialEq,Eq,Debug,Clone)]
pub struct Alias {
    pub name: String,
    pub command: String,
    pub confirm: bool,
    pub confirmation_message: String,
    pub conditional: Option<String>,
    pub unit_test: Option<String>,
    pub basename: PathBuf,
}

impl Alias {

    pub fn new() -> Alias {
        Alias {
            name: String::new(),
            command: String::new(),
            confirm: false,
            confirmation_message: String::new(),
            conditional: None,
            unit_test: None,
            basename: PathBuf::new(),
        }
    }

    pub fn execute(&self) {
        let mut process = Command::new("bash")
            .arg("-c")
            .arg(&self.command)
            .spawn()
            .unwrap_or_else(|e| { panic!("failed to execute child: {}", e) });

        let _ = process.wait()
                 .unwrap_or_else(|e| { panic!("failed to wait on child: {}", e) });
    }
}
