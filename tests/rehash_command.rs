#![feature(plugin,const_fn)]
#![plugin(stainless)]

extern crate aliases;

#[cfg(test)]
mod tests {

    pub use aliases::Rehash;
    pub use std::path::{Path, PathBuf};
    pub use std::fs;
    pub use std::env;

    describe! execute {

        before_each {
            let current_dir = env::current_dir().unwrap();
            let mut rehash = Rehash::new();
            let shim_directory = current_dir.join("tests/fixtures/shims/");
            rehash.shim_directory = shim_directory.clone();
        }

        describe! when_there_are_initialized_aliases {

            before_each {
                let alias_directories = vec![current_dir.join("tests/fixtures/initialized_dir/")];
                rehash.alias_directories = alias_directories;
            }

            describe! when_there_is_no_global_shim_for_an_alias {

                before_each {
                    let _ = fs::remove_file(shim_directory.join("test-command"));
                }

                it "generates one" {
                    rehash.execute();
                    assert!(shim_directory.join("test-command").as_path().exists());
                    // check the content of the shim?
                }
            }

            describe! when_there_is_no_specific_shim_for_an_alias {

                before_each {
                    // TODO remove all the bullshit
                    let nested_path = current_dir.join("tests/fixtures/initialized_dir/test-command");
                    let shim_specific_path;
                    if nested_path.has_root() {
                        let mut string = String::from(nested_path.to_str().unwrap()); // TODO handle the none option??
                        string.remove(0);
                        shim_specific_path = shim_directory.join(string);
                        
                    } else {
                        shim_specific_path = shim_directory.join(nested_path);
                    }

                    let _ = fs::remove_file(shim_specific_path.clone());
                }

                it "generates one" {
                    rehash.execute();
                    assert!(shim_specific_path.as_path().exists());
                }
            }

            describe! when_there_is_already_a_shim_for_an_alias {
                
                describe! and_the_alias_has_not_changed {

                    it "leaves the existing shim unchanged" {}
                }

                describe! and_the_alias_has_changed {
                    it "updates the shim" {}
                }
            }

            describe! when_there_are_erroneous_aliases {

                it "shows a warning" { }
            }
        }

        describe! when_there_are_no_initialized_aliases {

            it "gives the user feedback on how to create aliases" { }
        } 
    }
}
