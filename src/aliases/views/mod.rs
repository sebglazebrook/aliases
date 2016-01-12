use aliases::collections::Aliases;

pub struct AliasesView {
    aliases: Aliases,
}

impl AliasesView {

    pub fn new(aliases: Aliases) -> Self {
        AliasesView { aliases: aliases }
    }

    pub fn render(&self) {
        // TODO
    }
}

