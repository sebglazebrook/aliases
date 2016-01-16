use aliases::collections::Aliases;

pub struct AliasesView {
    aliases: Aliases,
}

impl AliasesView {

    pub fn new(aliases: Aliases) -> Self {
        AliasesView { aliases: aliases }
    }

    pub fn render(&self) {
        println!("\n\n");
        println!("Here are your available aliases:\n");
        for alias in self.aliases.raw_collection.iter() {
            println!("Alias: {}", alias.name);
            println!("Command: {}\n", alias.command);
        }
        println!("\n\n");
    }
}

