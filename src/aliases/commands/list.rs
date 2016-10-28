use aliases::views::AliasesView;
use aliases::collections::Aliases;
use aliases::factories::AliasFactory;
use aliases::repositories::AliasRepository;

use std::path::PathBuf;
use std::env;

pub struct List {
    current_path: PathBuf,
    directory_filter: Option<String>,
    name_filter: Option<String>,
    alias_paths: Vec<PathBuf>,
}

impl List {

    pub fn new(current_path: PathBuf, directory_filter: Option<&str>, name_filter: Option<&str>, alias_paths: Vec<PathBuf>) -> Self {
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
            alias_paths: alias_paths,
        }
    }

    // TODO this needs to start using the AliasRepository
    pub fn execute(&mut self) -> i32  {
        let mut aliases = self.global_aliases().merge(self.parent_aliases()).merge(self.local_aliases());
        if let Some(ref directory_filter) = self.directory_filter {
            let mut new_collection = vec![];
            for i in aliases.into_iter().filter(|alias| alias.basename.to_str().unwrap() == *directory_filter) {
                new_collection.push(i.clone());
            }
            aliases = Aliases::new(new_collection);
        }
        if let Some(ref name_filter) = self.name_filter {
            let mut new_collection = vec![];
            for i in aliases.into_iter().filter(|alias| alias.name == *name_filter) {
                new_collection.push(i.clone());
            }
            aliases = Aliases::new(new_collection);
        }
        AliasesView::new(aliases.clone()).render();
        if aliases.len() > 0 {
            0
        } else {
            1
        }
    }

    // ------ private ----- //

    fn global_aliases(&mut self) -> Aliases {
        match AliasRepository::find_for_directory(&self.global_aliases_data_file().to_str().unwrap().to_string()) {
            Err(_) => { AliasFactory::create_empty() },
            Ok(aliases) => { aliases }
        }
    }

    fn parent_aliases(&mut self) -> Aliases {
        AliasFactory::create_empty() // TODO let's leave this for later, it can be tricky depending on what dir they are in
    }

    fn local_aliases(&mut self) -> Aliases {
        match AliasRepository::find_for_directory(&self.current_path.to_str().unwrap().to_string()) {
            Err(_) => { AliasFactory::create_empty() },
            Ok(aliases) => { aliases }
        }
    }

    fn global_aliases_data_file(&self) -> PathBuf {
        match self.home_dir() {
            Some(dir) => dir,
            None => PathBuf::new()
        }
    }

    fn home_dir(&self) -> Option<PathBuf> {
        match env::var("HOME") {
            Ok(home_dir) => {
                Some(PathBuf::from(home_dir))
            },
            Err(_) => {
                None
            },
        }
    }
}
