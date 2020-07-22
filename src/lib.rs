#[macro_use]
extern crate log;
extern crate yaml_rust;
extern crate crypto;
extern crate rustc_serialize;
extern crate tabwriter;
extern crate countdown;
extern crate crossbeam;
extern crate regex;
extern crate scoped_pool;
extern crate prettytable;

#[macro_use]
extern crate lazy_static;

mod aliases;

pub use aliases::App;

// TODO everything below here shouldn't be here.
// had to do this for the tests, why?
pub use aliases::commands::{Rehash, Exec};
pub use aliases::builders::AliasBuilder;
pub use aliases::models::{Alias, Conditional};
pub use aliases::factories::{AliasFactory, ShimFileFactory};
pub use aliases::collections::Aliases;

use prettytable::format::LineSeparator;
use prettytable::format::LinePosition;
use prettytable::format::FormatBuilder;
use prettytable::format::TableFormat;

lazy_static! {
    static ref COOL_SEP: LineSeparator = LineSeparator::new('\u{2256}', '\u{2256}', '\u{2256}', '\u{2256}');

    pub static ref COOL_FORMAT: TableFormat = FormatBuilder::new()
      .column_separator('\u{22EE}')
      .borders('\u{22EE}')
      .separator(LinePosition::Title, *COOL_SEP)
      .separator(LinePosition::Bottom, *COOL_SEP)
      .separator(LinePosition::Top, *COOL_SEP)
      .padding(1, 1)
      .build();
}
