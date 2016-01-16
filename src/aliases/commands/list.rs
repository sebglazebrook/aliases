use std::path::PathBuf;
use std::env;

use aliases::views::AliasesView;
use aliases::collections::Aliases;
use aliases::factories::AliasFactory;

pub struct List {
    current_path: PathBuf,
}

impl List {

    pub fn new(current_path: PathBuf) -> Self {
        List { current_path: current_path }
    }

    pub fn execute(&mut self) {
        let aliases = self.global_aliases().merge(self.parent_aliases()).merge(self.local_aliases());
        AliasesView::new(aliases).render();
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
