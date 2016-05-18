use aliases::collections::Aliases;
use aliases::models::Alias;
use aliases::ExecutionWorkflow;
use aliases::repositories::AliasRepository;

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

    pub fn execute(&self) {
        match self.find_alias() {
            Err(_) => {} // TODO handle this
            Ok(mut alias) => {
                alias.command_arguments = self.forwarding_args.clone();
                ExecutionWorkflow::new(alias).execute();
            }
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
