pub use self::alias_factory::AliasFactory;

use aliases::models::Alias;
use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;
use crypto::md5::Md5;
use crypto::digest::Digest;
use std::process::Command;

mod alias_factory;

pub struct ShimFileFactory; 

impl ShimFileFactory {

    pub fn create(alias: &Alias, dir: &PathBuf) {
        // TODO this is not deleting old shims
        let filepath = dir.join(alias.name.clone());
        if !filepath.exists() {
            info!("Creating new alias '{}'", &alias.name);
            match File::create(&filepath) {
                Err(error) => {
                    warn!("An error occurred {} : {:?}", error, &filepath);
                },
                Ok(mut file) => {
                    let _ = file.write_all(&ShimFileFactory::template_string().into_bytes());
                    let mut command = String::from("chmod +x ");
                    command.push_str(filepath.to_str().unwrap());
                    // this can fail if there are too many files open we should wait and try again
                    let _ = Command::new("bash")
                        .arg("-c")
                        .arg(&command)
                        .output()
                        .unwrap_or_else(|e| { panic!("failed to execute child: {}", e) });
                    info!("Successfully created alias '{}' in location {:?}", &alias.name, &filepath);
                }
            }
        } else {
            info!("Alias already exists '{}' checking if it's still valid", &alias.name);
            if !ShimFileFactory::is_valid(&filepath) {
                info!("'{}' not valid, recreating...", &alias.name);
                match File::create(&filepath) {
                    Err(error) => {
                        warn!("An error occurred {} : {:?}", error, filepath);
                    },
                    Ok(mut file) => {
                        let _ = file.write_all(&ShimFileFactory::template_string().into_bytes());
                        info!("Successfully created alias '{}' in location {:?}", &alias.name, &filepath);
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
        ShimFileFactory::template_string()
    }

    fn template_string() -> String {
"#!/usr/bin/env bash
set -e

COMMAND_NAME=\"$(exec basename \"$0\")\"

if ! hash aliases 2>/dev/null; then
  echo \"aliases command doesn't exists, can't continue\"
  exit 1
fi

if aliases list --directory \"$PWD\" --name \"$COMMAND_NAME\" >/dev/null 2>&1; then
  aliases exec \"$PWD\" \"$COMMAND_NAME\" -- \"$@\"
elif aliases list --directory \"$HOME\" --name \"$COMMAND_NAME\" >/dev/null 2>&1; then
  aliases exec \"$HOME\" \"$COMMAND_NAME\" -- \"$@\"
else
  PATH=${PATH/$HOME\\/.aliases.d\\/shims:/} # remove shims from path

  if hash $COMMAND_NAME 2>/dev/null; then
    exec \"$COMMAND_NAME\" \"$@\"
  else
    echo \"No alias '$COMMAND_NAME' available in this directory\"
    exit 1
  fi

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
