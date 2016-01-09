#![feature(plugin,const_fn)]
#![plugin(stainless)]

extern crate aliases;
extern crate gag;

#[cfg(test)]
mod tests {
    pub use aliases::App;
    pub use std::path::PathBuf;
    pub use std::io::prelude::*;
    pub use std::fs;
    pub use std::env;
    pub use std::fs::File;
    pub use gag::BufferRedirect;


    pub fn content_for_file(path: &str) -> String {
        let mut file = File::open(path).unwrap();
        String::new()
    }

    describe! app {

        describe! execute_init {

            before_each {
                let mut app = App::new();
                let mut global = false;
            }

            describe! when_the_current_directory_has_not_previously_been_initialized {

                before_each {
                    let path = PathBuf::from("tests/fixtures/uninitialized_dir");
                    app.current_path = path.clone();
                    fs::remove_file(path.join(".aliases"));
                }

                it "creates a .aliases file using the stored template" {
                    app.execute_init(global);
                    let template_aliases_file_content = content_for_file("src/templates/aliases");
                    let target_dir_aliases_file_content = content_for_file("tests/fixtures/uninitialized_dir/.aliases");
                    assert_eq!(target_dir_aliases_file_content, template_aliases_file_content);
                }
            }

            describe! when_the_current_directory_has_already_been_initialized {

                before_each {
                   let path = PathBuf::from("tests/fixtures/uninitialized_dir");
                   app.current_path = path.clone();
                   let mut buf = BufferRedirect::stdout().unwrap(); 
                   File::create(path.join(".aliases"));
                }

                it "it alerts the user" {
                    app.execute_init(global);
                    let mut output = String::new();
                    buf.read_to_string(&mut output).unwrap();
                    assert_eq!(&output[..], "Directory already initialized.\n");
                }
            }

            describe! when_global_is_true {

                before_each {
                    let mut global = true;
                    env::set_var("HOME", "/tmp");
                }

                describe! and_the_home_directory_has_not_previously_been_initialized {

                    before_each {
                        fs::remove_file("/tmp/.aliases");
                    }

                    it "creates a .aliases file using the stored template" {
                        app.execute_init(global);
                        let template_aliases_file_content = content_for_file("src/templates/aliases");
                        let target_dir_aliases_file_content = content_for_file("/tmp/.aliases");
                        assert_eq!(target_dir_aliases_file_content, template_aliases_file_content);
                    }
                }

                describe! and_the_home_directory_has_already_been_initialized {

                    before_each {
                        let path = PathBuf::from("/tmp");
                        let mut buf = BufferRedirect::stdout().unwrap(); 
                        File::create(path.join(".aliases"));
                    }

                    it "it alerts the user" {
                        app.execute_init(global);
                        let mut output = String::new();
                        buf.read_to_string(&mut output).unwrap();
                        assert_eq!(&output[..], "Directory already initialized.\n");
                    }
                }
            }
        }
    }
}

