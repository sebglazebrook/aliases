#[macro_use]
extern crate log;
extern crate yaml_rust;
extern crate crypto;
extern crate serde_json;
extern crate serde;
extern crate tabwriter;
extern crate countdown;
extern crate crossbeam;
extern crate regex;
extern crate scoped_pool;


mod aliases;

pub use aliases::App;

// TODO everything below here shouldn't be here.
// had to do this for the tests, why?
pub use aliases::commands::{Rehash, Exec};
pub use aliases::builders::AliasBuilder;
pub use aliases::models::{Alias, Conditional};
pub use aliases::factories::{AliasFactory, ShimFileFactory};
pub use aliases::collections::Aliases;
