use std::io;
use std::io::Write;
use prettytable::{Attr, color, cell, format, row, Table};

use aliases::collections::Aliases;

pub struct AliasesView {
    aliases: Aliases,
}

impl AliasesView {

    pub fn new(aliases: Aliases) -> Self {
        AliasesView { aliases: aliases }
    }

    pub fn render(&self) {
        let mut table = Table::new();
        table.set_format(*crate::COOL_FORMAT);
        table.set_titles(row![
            Fc -> "Alias",
            Fc -> "Command",
            Fc -> "Confirm",
        ]);
        for alias in self.aliases.clone().into_iter() {
            table.add_row(row![
            FY -> &alias.name,
            Fg -> &alias.command,
            Fr -> &alias.confirm,
           ]);
        }
        table.print_tty(true);
    }
}
