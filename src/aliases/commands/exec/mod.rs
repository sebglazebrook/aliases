use aliases::collections::Aliases;
use aliases::models::Alias;
use aliases::ExecutionWorkflow;
use aliases::repositories::AliasRepository;
use aliases::commands::{AliasCommand, CommandResponse};

pub struct Exec {
    directory: String,
    name: String,
    forwarding_args: Vec<String>,
}

impl Exec {

    pub fn new(directory: String, name: String, forwarding_args: Vec<String>) -> Self {
        Exec {
            directory: directory,
            name: name,
            forwarding_args: forwarding_args,
        }
    }


    //----------- private -----------//

    fn find_alias(&self) -> Result<Alias, &'static str> {
        match self.directory_aliases() {
            Err(message) => { Err(message) },
            Ok(aliases) => {
                match aliases.into_iter().find(|alias| { alias.name == self.name }) {
                    None => { Err("could not find an alias to execute") },
                    Some(alias) => { Ok(alias) },
                }
            }
        }
    }

    fn directory_aliases(&self) -> Result<Aliases, &'static str> {
        AliasRepository::find_for_directory(&self.directory)
    }
}

impl AliasCommand for Exec {

    fn execute(&self) -> CommandResponse {
        match self.find_alias() {
            Err(message) => { println!("Error! {}", message); } // TODO handle this better?
            Ok(mut alias) => {
                alias.add_arguments(self.forwarding_args.clone());
                ExecutionWorkflow::new(alias).execute();
            }
        }
        CommandResponse::success()
    }

}
