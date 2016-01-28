use std::path::PathBuf;
use aliases::factories::AliasFactory;
use aliases::models::Alias;

pub struct Exec {
    directory: String,
    name: String,
}

impl Exec {

    pub fn new(directory: String, name: String) -> Self {
        Exec {
            directory: directory,
            name: name,
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
                ExecutionWorkflow::new(alias.clone()).execute();
           }
        }
    }
}

// TODO move this to own file
struct ExecutionWorkflow {
    alias: Alias,
}

impl ExecutionWorkflow {

    pub fn new(alias: Alias) -> Self {
        ExecutionWorkflow { alias: alias }
    }

    pub fn execute(&self) {
        if self.conditional_passes() {
            if self.user_confirmation_successful() {
                self.execute_command();
            }
        }
    }

    //------------- private -----------//

    fn conditional_passes(&self) -> bool {
        self.alias.conditional.execute()
    }

    fn user_confirmation_successful(&self) -> bool {
        self.alias.user_confirmation.execute()
    }

    fn execute_command(&self) {
        self.alias.execute();
    }
}
