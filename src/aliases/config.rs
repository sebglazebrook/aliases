use std::env;
use std::path::{PathBuf};
use std::io::prelude::*;
use std::fs::File;
use std::process::Command;
use rustc_serialize::json;
use std::io;
use ansi_term::Colour::{Fixed, Blue};
use std::string::ToString;

#[derive(Clone, RustcDecodable, RustcEncodable)]
pub struct Config {
    pub shim_directory: String,
    alias_directories: Vec<String>,
    users: Vec<String>,
    disabled_users: Option<Vec<String>>,
    dry_run_prefix_string: Option<String>,
    dry_run_prefix_color: Option<u8>
}

impl Config {

    pub fn load() -> Config {
        if Config::config_file_path().exists() {
            Config::load_file(&Config::config_file_path())
        } else {
            Config::create(&Config::config_file_path())
        }
    }

    pub fn shim_path(&self) -> PathBuf {
        let mut command = String::from("echo \"");
        command.push_str(&self.shim_directory);
        command.push_str("\"");
        let output = Command::new("bash")
            .arg("-c")
            .arg(&command)
            .output()
            .unwrap_or_else(|e| { panic!("failed to execute child: {}", e) });

        PathBuf::from(String::from_utf8(output.stdout).unwrap().trim())
    }

    pub fn alias_paths(&self) -> Vec<PathBuf> {
        self.alias_directories.iter().map(|dir| PathBuf::from(dir)).collect()
    }

    pub fn directories(&self) -> Vec<String> {
        self.alias_directories.clone()
    }

    pub fn users(&self) -> Vec<String> {
        self.users.clone()
    }

    pub fn update_users(&mut self, users: Vec<String>) {
        self.users = users;
        self.users.dedup();
        self.update_file();
    }

    pub fn enable_user(&mut self, username: &str) -> Result<(), io::Error> {
        match self.disabled_users.clone() {
            Some(users) => {
                self.disabled_users = Some(users.clone().into_iter().filter(|user| user != username).collect());
                let mut users = self.users();
                users.push(username.to_string());
                self.update_users(users);
            },
            None => {
                self.disabled_users = Some(vec![]);
            }
        }
        self.update_file()
    }

    pub fn disabled_users(&self) -> Vec<String> {
        match self.disabled_users.clone() {
            Some(users) => { users },
            None =>  { vec![] }
        }

    }

    pub fn disable_user(&mut self, username: &str) {
        match self.disabled_users.clone() {
            Some(ref mut users) => {
                users.push(username.to_string());
                self.disabled_users = Some(users.clone());
            },
            None => { self.disabled_users = Some(vec![username.to_string()]); }
        };
        self.update_file();
    }

    pub fn add_alias_directory(&mut self, directory: &PathBuf, username: &String) {
        let string = directory.to_str().unwrap().to_owned();
        self.alias_directories.push(string);
        self.alias_directories.dedup();
        self.users.push(username.to_owned());
        self.users.dedup();
        self.update_file();
    }

    pub fn set_user_priority(&mut self, username: &String, priority: usize) -> Result<(), String> {
        match self.users().into_iter().position(|user| {user == username.to_owned()}) {
            Some(index) => {
                let mut users = self.users();
                let user = users.remove(index);
                users.insert(priority - 1, user);
                self.update_users(users);
                Ok(())
            },
            None => Err(format!("Error! Could not find the user {}.", username)),
        }
    }

    pub fn dry_run_prefix(&self) -> String {
        match self.dry_run_prefix_string.clone() {
            None => { Blue.bold().paint("Executing:").to_string() }
            Some(string) => {
                match self.dry_run_prefix_color.clone() {
                    Some(value) => { Fixed(value).paint(string).to_string() }
                    None => {
                        // if here remove double escapes
//                        println!(r#"{}"#, string);
                        // let result = str::replace(&string, r#"\\"#, r#"\"#);
 //                       println!("{:?}", result);
                        //result
                        string
                    }
                }
            }
        }
    }

    // ------- private methods --------//

    fn config_file_path() -> PathBuf {
        match env::var("HOME") {
            Ok(home_dir) => {
                PathBuf::from(home_dir).join(".aliases_cfg")
            },
            Err(_) => {
                PathBuf::new() // TODO need to handle this better
            },
        }
    }

    fn create(path: &PathBuf) -> Config {
        let mut file = File::create(path).unwrap(); //TODO handle the error case
        let default_config = TemplateRepository::config_template();
        file.write_all(default_config.as_bytes()).unwrap();
        let config: Config = json::decode(&default_config).unwrap();
        config
    }

    fn load_file(path: &PathBuf) -> Config {
        let mut file = File::open(path).unwrap(); //TODO handle the error case
        let mut content = String::new();
        let _ = file.read_to_string(&mut content);
        let config: Config = json::decode(&content).unwrap();
        config
    }

    fn update_file(&self) -> Result<(), io::Error> {
        let mut file = try!(File::create(Self::config_file_path()));
        let encoded = json::encode(&self).unwrap();
        try!(file.write_all(encoded.as_bytes()));
        Ok(())
    }
}

struct TemplateRepository;

impl TemplateRepository {

    pub fn config_template() -> String {
        "{
            \"shim_directory\" : \"${HOME}/.aliases.d/shims\",
            \"alias_directories\" : [],
            \"users\" : [\"default\"]
        }
        ".to_string()
    }
}

