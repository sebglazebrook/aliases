use std::io::{self, Write};

#[derive(PartialOrd,Ord,PartialEq,Eq,Debug,Clone)]
pub struct UserConfirmation {
    enabled: bool,
    message: String,
}

impl UserConfirmation {

    pub fn new(enabled: bool, message: String) -> Self {
        UserConfirmation { enabled: enabled, message: message }
    }

    pub fn execute(&self) -> bool {
        if self.enabled {
            print!("{}. Type 'Yes' to continue...  ", self.message);
            io::stdout().flush().unwrap();
            let mut user_input = String::new();
            let _ = io::stdin().read_line(&mut user_input); // TODO potential error to be handled here
            user_input.trim() == "Yes"
        } else {
            true
        }
    }
}
