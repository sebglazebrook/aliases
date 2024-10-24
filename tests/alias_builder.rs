#![feature(plugin,const_fn)]

extern crate aliases;
extern crate yaml_rust;

#[cfg(test)]
mod tests {

    pub use yaml_rust::{YamlLoader};
    pub use aliases::{Alias, AliasBuilder, Conditional};
    pub use std::path::PathBuf;

    //describe! alias_builder {

        //describe! from_yaml {

            //before_each {
                //let basename = PathBuf::new();
                //let yaml_string =
//"command: ./super_command.sh
//confirm: true
//confirmation_message: Are you really really sure??
//conditional: /bin/true
//backout_seconds: 3
//unit_test: '[ true = true ]'
//quiet: true
//";
                //let docs = YamlLoader::load_from_str(yaml_string).unwrap();
                //let doc = &docs[0];
            //}

            //describe! with_all_the_field_included {

                //before_each {
                    //let mut alias = Alias::new();
                    //match AliasBuilder::from_yaml("command_name", basename.clone(), doc.clone()).build() {
                        //Ok(a) => { alias = a; },
                        //Err(_) => { },
                    //};
                //}

                //it "creates an Alias with all the fields set" {
                    //assert_eq!(alias.name, "command_name");
                    //assert_eq!(alias.command, "./super_command.sh");
                    //assert_eq!(alias.confirm, true);
                    //assert_eq!(alias.confirmation_message, "Are you really really sure??");
                    //assert_eq!(alias.conditional, Conditional::new("/bin/true".to_string()));
                    //assert_eq!(alias.delayed_backout, 3);
                    //assert_eq!(alias.unit_test, "[ true = true ]".to_string());
                    //assert_eq!(alias.quiet, true);
                //}
            //}

            //describe! when_there_is_no_name {
            //}

            //describe! when_there_is_no_command {
            //}

            //describe! when_there_is_no_confirmation {

                //before_each {
                //let yaml_string =
//"command: ./super_command.sh
//unit_test: '[ true == true ]'
//conditional: /bin/true
//confirmation_message: Are you really really sure??
//";
                //let docs = YamlLoader::load_from_str(yaml_string).unwrap();
                //let doc = &docs[0];
                //}

                //it "builds with confirmation turned off" {
                    //let mut alias = Alias::new();
                    //match AliasBuilder::from_yaml("command_name", basename.clone(), doc.clone()).build() {
                        //Err(_) => {},
                        //Ok(a) => { alias = a; }
                    //}
                    //assert_eq!(alias.confirm, false);
                //}
            //}

            //describe! when_there_is_no_confirmation_message {

                //before_each {
                //let yaml_string =
//"command: ./super_command.sh
//confirm: true
//unit_test: '[ true == true ]'
//conditional: /bin/true
//";
                //let docs = YamlLoader::load_from_str(yaml_string).unwrap();
                //let doc = &docs[0];
                //}

                //it "builds with a default confirmation message" {
                    //let mut alias = Alias::new();
                    //match AliasBuilder::from_yaml("command_name", basename.clone(), doc.clone()).build() {
                        //Err(_) => {},
                        //Ok(a) => { alias = a; },
                    //}
                    //assert_eq!(alias.confirmation_message, "About to execute `./super_command.sh`");
                //}
            //}

            //describe! when_there_is_no_conditional {

                //before_each {
                //let yaml_string =
//"command: ./super_command.sh
//confirm: true
//confirmation_message: Are you really really sure??
//unit_test: '[ true == true ]'
//";
                //let docs = YamlLoader::load_from_str(yaml_string).unwrap();
                //let doc = &docs[0];
                //}

                //it "builds without a conditional" {
                    //AliasBuilder::from_yaml("command_name", basename.clone(), doc.clone()).build().is_ok();
                //}
            //}

            //describe! when_there_is_no_unit_test {

                //before_each {
                //let yaml_string =
//"command: ./super_command.sh
//confirm: true
//confirmation_message: Are you really really sure??
//conditional: /bin/true
//";
                //let docs = YamlLoader::load_from_str(yaml_string).unwrap();
                //let doc = &docs[0];
                //}

                //it "builds without a unit test" {
                    //AliasBuilder::from_yaml("command_name", basename.clone(), doc.clone()).build().is_ok();
                //}
            //}

            //describe! when_there_is_no_delay_backout {
                //before_each {
                    //let yaml_string =
//"command: ./super_command.sh
//confirm: true
//confirmation_message: Are you really really sure??
//conditional: /bin/true
//";
                //let docs = YamlLoader::load_from_str(yaml_string).unwrap();
                //let doc = &docs[0];
                //}

                //it "builds without a delayed backout" {
                    //AliasBuilder::from_yaml("command_name", basename.clone(), doc.clone()).build().is_ok();
                //}
            //}
        //}
    //}
}
