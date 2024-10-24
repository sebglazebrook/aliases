#![feature(plugin,const_fn)]

extern crate aliases;

#[cfg(test)]
mod tests {

    pub use std::path::PathBuf;
    pub use aliases::{Alias, AliasFactory};

    //describe! alias_builder {

        //describe! create_from_file {

            //before_each {
                //let path: PathBuf;
            //}

            //describe! when_the_file_does_not_exists {

                //before_each {
                    //path = PathBuf::new();
                //}

                //it "returns an error" {
                    //let result = AliasFactory::create_from_file(path);
                    //assert!(result.is_err());
                    //assert_eq!(result.err(), Some("File did not exist."));
                //}
            //}

            //describe! when_the_file_exists {

                //describe! but_the_content_is_invalid {

                    //before_each {
                        //path = PathBuf::from("./tests/fixtures/aliases_files/invalid");
                    //}
                    
                    //it "returns an error" {
                        //let result = AliasFactory::create_from_file(path);
                        //assert!(result.is_err());
                        //assert_eq!(result.err(), Some("File invalid content."));
                    //}
                //}

                //describe! and_the_content_is_valid {

                    //before_each {
                        //path = PathBuf::from("./tests/fixtures/aliases_files/valid");
                    //}

                    //it "returns a collection of aliases" {
                        //let result = AliasFactory::create_from_file(path);
                        //assert!(result.is_ok()); // how can I check the class type or something else??
                    //}
                //}
            //}
        //}
    //}
}
