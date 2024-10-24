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
        let command = CommandBuilder::using_bash()
                                    .for_command(&self.command)
                                    .with_args(&self.args)
                                    .pseudo_build();
        command.command_string
    }

    pub fn add_arguments(&mut self, arguments: Vec<String>) {
        self.args = arguments.clone();
    }

    pub fn as_yaml(&self) -> String {
        AliasYamlBuilder::new(&self).build()
    }
}

struct AliasYamlBuilder<'a> {
    alias: &'a Alias,
}

impl<'a> AliasYamlBuilder<'a> {

    pub fn new(alias: &'a Alias) -> Self {
        AliasYamlBuilder { alias }
    }

    pub fn build(&self) -> String {
        let output = self.build_initial_string();
        //self.add_confirm(&mut output);
        //self.add_confirmation_message(&mut output);
        //self.add_conditional(&mut output);
        //self.add_backout_seconds(&mut output);
        //self.add_unit_test(&mut output);
        //self.add_quiet(&mut output);
        output
//#alias_name:
//#  command: ./super_command.sh                         # required
//#  confirm: true                                       # optional
//#  confirmation_message: Are you sure you are sure??   # optional
//#  conditional: /bin/true                              # optional
//#  backout_seconds: 3                                  # optional
//#  unit_test: '[ true = true ]'                        # optional
//#  quiet: false                                        # optional
    }

    //-------- private ------//

    fn build_initial_string(&self) -> String {
        format!("\n{}:\n  command: {}\n", self.alias.name, self.alias.command)
    }

}
