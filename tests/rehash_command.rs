#![feature(plugin,const_fn)]
#![plugin(stainless)]

extern crate aliases;

#[cfg(test)]
mod tests {

    pub use aliases::Rehash;
    pub use aliases::Alias;
    pub use aliases::ShimFileFactory;
    pub use std::path::{Path, PathBuf};
    pub use std::fs;
    pub use std::env;
    pub use std::fs::File;

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
                    assert!(ShimFileFactory::is_valid(&shim_directory.join("test-command")), true);
                }
            }

            describe! when_there_is_already_a_shim_for_an_alias {
                
                //describe! and_the_alias_has_not_changed {

                    //before_each {
                        //let mut  alias = Alias::new();
                        //alias.name = String::from("test-command");
                        //let _ = ShimFileFactory::create(&alias, &shim_directory);
                        //let create
                        //fs::metadata("/some/file/path.txt")
                    //}

                    //it "leaves the existing shim unchanged" {

                    //}
                //}

                describe! and_the_alias_has_changed {

                    before_each {
                        let _ = File::create(shim_directory.join("test-command"));
                    }

                    it "updates the shim" {
                        rehash.execute();
                        assert!(ShimFileFactory::is_valid(&shim_directory.join("test-command")), true);
                    }
                }
            }
        }
    }
}
