pub use self::alias_factory::AliasFactory;
//pub use self::shim_file_factory;

use aliases::models::Alias;
use std::path::PathBuf;
use std::fs;
use std::fs::File;

mod alias_factory;

pub struct ShimFileFactory; 

impl ShimFileFactory {

    pub fn create_global(alias: &Alias, dir: &PathBuf) { // TODO how does this know the global directory??
        let filepath = dir.join(alias.name.clone());
        if !filepath.exists() {
            let _ = File::create(filepath); // TODO don't just create a file, create it with content
        }
    }

    pub fn create_specific(alias: &Alias, dir: &PathBuf, shim_dir: &PathBuf) {
        let nested_path = dir.join(alias.name.clone());
        let shim_specific_path;
        if nested_path.has_root() {
            let mut string = String::from(nested_path.to_str().unwrap()); // TODO handle the none option??
            string.remove(0);
            shim_specific_path = shim_dir.join(string);
        } else {
            shim_specific_path = shim_dir.join(nested_path);
        }
        if !shim_specific_path.exists() {
            let _ = fs::create_dir_all(shim_specific_path.parent().unwrap()); // TODO handle the none case??
            let _ = File::create(shim_specific_path); // TODO don't just create a file, create it with content
        }
    }
}
