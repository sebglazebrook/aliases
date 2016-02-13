use std::process::Command;

#[derive(PartialOrd,Ord,PartialEq,Eq,Debug,Clone)]
pub struct Conditional {
    command: String,
}

impl Conditional {

    pub fn new(command: String) -> Self {
        Conditional { command: command }
    }

    pub fn default() -> Self {
       Conditional { command: String::from("true") }
    }

    pub fn execute(&self) -> bool {
        let status = Command::new("bash")
            .arg("-c")
            .arg(&self.command)
            .status()
            .unwrap_or_else(|e| { panic!("failed to execute child: {}", e) });
        status.success()
    }

}
