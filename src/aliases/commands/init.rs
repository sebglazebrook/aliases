use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::File;

use aliases::Config;

pub struct Init {
    target_path: PathBuf,
    config: Config,
    global: bool,
}

impl Init {

    pub fn new(target_path: PathBuf, config: Config, global: bool) -> Init {
        Init { target_path: target_path, config: config, global: global}
    }

    pub fn execute(&mut self) {
        if self.global {
            let path_update = String::from("export PATH=\"") + &self.config.shim_directory +  ":${PATH}\"";
            println!("{}\naliases rehash", path_update);
        } else {
            if Path::new(&self.target_path.join(".aliases")).exists() {
                println!("Directory already initialized.");
            } else {
                let mut new_file = File::create(self.target_path.join(".aliases")).unwrap();
                let template_string = self.template_string();
                let array = template_string.as_bytes();
                let _ = new_file.write_all(array);
                self.add_to_global_config();
            }
        }
    }

    // ------------ private ---------- //

    fn template_string(&self) -> String {
String::from("#alias_name:
#  command: ./super_command.sh                         # required
#  confirm: true                                       # optional
#  confirmation_message: Are you sure you are sure??   # optional
#  conditional: /bin/true                              # optional
#  backout_seconds: 3                                  # optional
#  unit_test: '[ true = true ]'                        # optional
")
    }

    fn add_to_global_config(&mut self) {
        self.config.add_alias_directory(&self.target_path);
    }
}
