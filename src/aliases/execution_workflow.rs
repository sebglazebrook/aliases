use std::io::Write;

use countdown::Countdown;
use aliases::models::Alias;

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let result =  writeln!(&mut ::std::io::stderr(), $($arg)*);
        result.expect("failed printing to stderr");
    } }
);

pub struct ExecutionWorkflow {
    alias: Alias,
}

impl ExecutionWorkflow {

    pub fn new(alias: Alias) -> Self {
        ExecutionWorkflow { alias }
    }

    pub fn execute(&self) {
        if self.conditional_passes() {
            if self.user_confirmation_successful() {
                self.allow_for_backout();
                self.output_command_to_be_executed();
                self.execute_command();
            }
        } else {
            // TODO alert the user
        }
    }

    //------------- private -----------//

    fn conditional_passes(&self) -> bool {
        self.alias.conditional.execute()
    }

    fn user_confirmation_successful(&self) -> bool {
        self.alias.user_confirmation.execute()
    }

    fn allow_for_backout(&self) {
        if self.alias.delayed_backout > 0 {
            println_stderr!("Executing '{}' in {} seconds", self.alias.command(), self.alias.delayed_backout);
            println_stderr!("Press ctrl + c to cancel execution.");
            Countdown::new(self.alias.delayed_backout.clone()).start();
        }
    }

    fn output_command_to_be_executed(&self) {
        if !self.alias.quiet {
            println_stderr!("Executing: {}", self.alias.command());
        }
    }

    fn execute_command(&self) {
        self.alias.execute();
    }
}
