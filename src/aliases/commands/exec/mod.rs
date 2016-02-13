use std::path::PathBuf;
use aliases::factories::AliasFactory;
use aliases::ExecutionWorkflow;

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
        let aliases_file =  PathBuf::from(&self.directory).join(".aliases");
        match AliasFactory::create_from_file(aliases_file) {
            Err(_) => {}, // TODO handle this
            Ok(aliases) => {
                let alias = aliases.raw_collection.iter().find(|alias| {
                    alias.name == self.name
                }).unwrap();
                let mut new_alias = alias.clone();
                new_alias.command_arguments = self.forwarding_args.clone();
                ExecutionWorkflow::new(new_alias).execute();
           }
        }
    }
}
