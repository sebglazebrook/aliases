use std::io;
use std::io::Write;
use tabwriter::TabWriter;

use aliases::collections::Aliases;

pub struct AliasesView {
    aliases: Aliases,
}

impl AliasesView {

    pub fn new(aliases: Aliases) -> Self {
        AliasesView { aliases: aliases }
    }

    pub fn render(&self) {
        let mut tw = TabWriter::new(io::stdout());
        tw.write("\nALIAS\tCOMMAND\tCONFIRM\n".as_bytes()).unwrap();
        for alias in self.aliases.raw_collection.iter() {
            let alias_row = String::new() + &alias.name + "\t"
                                          + &alias.command + "\t"
                                          + &alias.confirm.to_string() + "\n";
            tw.write(alias_row.as_bytes()).unwrap();
        }
        tw.flush().unwrap();
    }
}

