pub use self::alias_factory::AliasFactory;
//pub use self::shim_file_factory;

use aliases::models::Alias;
use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;
use rustache::*;
use crypto::md5::Md5;
use crypto::digest::Digest;

mod alias_factory;

pub struct ShimFileFactory; 

impl ShimFileFactory {

    pub fn create(alias: &Alias, dir: &PathBuf) {
        let filepath = dir.join(alias.name.clone());
        if !filepath.exists() {
            match File::create(filepath) {
                Err(_) => {}, //TODO handle this some day
                Ok(mut file) => {
                    let _ = file.write_all(&ShimFileFactory::template_string().into_bytes());
                }
            }
        } else {
            if !ShimFileFactory::is_valid(&filepath) {
                match File::create(filepath) {
                    Err(_) => {}, //TODO handle this some day
                    Ok(mut file) => {
                        let _ = file.write_all(&ShimFileFactory::template_string().into_bytes());
                    }
                }

            }
        }
    }

    pub fn is_valid(file_path: &PathBuf) -> bool {
        match File::open(file_path) {
           Err(_) => { false }, // TODO handle this properly
           Ok(mut file) => {
                let mut actual_content = String::new();
                let _ = file.read_to_string(&mut actual_content);
                let actual_md5 = ShimFileFactory::md5_for_string(actual_content);
                let expected_md5 = ShimFileFactory::md5_for_string(ShimFileFactory::build());
                actual_md5 == expected_md5
            }
        }
    }

    // ------- private methods ---------- //

    fn build() -> String {
        let data = HashBuilder::new().insert_string("name", ""); // TODO don't actually need to eval the template
        let result = render_text(&ShimFileFactory::template_string(), data);
        let mut rendered = String::new();
        let _ = result.unwrap().read_to_string(&mut rendered); // TODO handle the error case
        rendered
    }

    fn template_string() -> String {
"#!/usr/bin/env bash
set -e

COMMAND_NAME=\"$(exec basename \"$0\")\"

if ! hash aliases 2>/dev/null; then
  echo \"aliases command doesn't exists, can't continue\"
  exit 1
fi

if exec aliases list -d \"$PWD\" -f \"$COMMAND_NAME\"; then
  # can current directory/context can fulfill the command
  # yes, execute the command in the current context
  echo \"yes\"
else
  # remove alias shims from path
  exec \"$COMMAND_NAME\" \"$@\"
fi
".to_string()
    }

    fn md5_for_string(string: String) -> Vec<u8> {
        let mut md5 = Md5::new();
        md5.input(&string.into_bytes());
        let mut output = String::from("here is my string").into_bytes(); // TODO I know this is bad
        md5.result(&mut output);
        output
    }
}
