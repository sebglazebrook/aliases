use std::path::PathBuf;
use std::env;

use aliases::views::AliasesView;
use aliases::collections::Aliases;
use aliases::factories::AliasFactory;

pub struct List {
    current_path: PathBuf,
    directory_filter: Option<String>,
    name_filter: Option<String>,
}

impl List {

    pub fn new(current_path: PathBuf, directory_filter: Option<&str>, name_filter: Option<&str>) -> Self {
        // TODO there has to be a better way to do this right?
        let directory_string;
        match directory_filter {
            Some(string) => directory_string = Some(string.to_string()),
            None         => directory_string = None,
        }
        let name_string;
        match name_filter {
            Some(string) => name_string = Some(string.to_string()),
            None         => name_string = None,
        }
        List {
            current_path: current_path,
            directory_filter: directory_string,
            name_filter: name_string,
        }
    }

    pub fn execute(&mut self) -> i32  {
        // TODO this needs to actuall make sure the local aliases are in the global config for aliases
        let mut aliases = self.global_aliases().merge(self.parent_aliases()).merge(self.local_aliases());
        if let Some(ref directory_filter) = self.directory_filter {
            let mut new_collection = vec![];
            for i in aliases.raw_collection.iter().filter(|alias| alias.basename.to_str().unwrap() == *directory_filter) {
                new_collection.push(i.clone());
            }
            aliases = Aliases::new(new_collection);
        }
        if let Some(ref name_filter) = self.name_filter {
            let mut new_collection = vec![];
            for i in aliases.raw_collection.iter().filter(|alias| alias.name == *name_filter) {
                new_collection.push(i.clone());
            }
            aliases = Aliases::new(new_collection);
        }
        AliasesView::new(aliases.clone()).render();
        if aliases.raw_collection.len() > 0 {
            0
        } else {
            1
        }
    }

    // ------ private ----- //

    fn global_aliases(&mut self) -> Aliases {
        if self.global_aliases_data_file().exists() {
            match AliasFactory::create_from_file(self.global_aliases_data_file()) {
                Err(_) => { AliasFactory::create_empty() },
                Ok(aliases) => aliases
            }
        } else {
            AliasFactory::create_empty()
        }
    }

    fn parent_aliases(&mut self) -> Aliases {
        AliasFactory::create_empty() // let's leave this for later, it can be tricky depending on what dir they are in
    }

    fn local_aliases(&mut self) -> Aliases {
        if self.local_aliases_data_file().exists() {
            match AliasFactory::create_from_file(self.local_aliases_data_file()) {
                Err(_) => AliasFactory::create_empty(),
                Ok(aliases) => aliases
            }
        } else {
            AliasFactory::create_empty()
        }
    }

    fn global_aliases_data_file(&self) -> PathBuf {
        match self.home_dir() {
            Some(dir) => dir.join(".aliases"),
            None => PathBuf::new() // does this do what I think it does? as in return false when doing .exists()
        }

    }

    fn home_dir(&self) -> Option<PathBuf> {
        match env::var("home") {
            Ok(home_dir) => {
                Some(PathBuf::from(home_dir))
            },
            Err(_) => {
                None
            },
        }
    }

    fn local_aliases_data_file(&self) -> PathBuf {
        self.current_path.join(".aliases")
    }
}
