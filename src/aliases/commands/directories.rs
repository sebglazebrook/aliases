use prettytable::{cell, row, Table};
use aliases::Config;

pub struct Directories;

impl Directories {

    pub fn new() -> Self {
        Directories {}
    }

    pub fn execute(&self) -> i32 {
        let directories = Config::load().directories();

        let mut table = Table::new();
        table.set_format(*crate::COOL_FORMAT);
        table.set_titles(row![
            Fc -> "Directory",
        ]);

        let mut dup_check = vec![];
        for dir in directories.iter() {
            if ! dup_check.contains(&dir) {
                dup_check.push(dir);
                table.add_row(row![
                    FY -> dir,
                ]);
            }
        }

        table.print_tty(true);

        // println!("these dirs{:}", directories);
        0 // TODO make this a real exit code
    }
}
