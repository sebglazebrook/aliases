extern crate yaml_rust;
extern crate crypto;
extern crate rustc_serialize;
extern crate tabwriter;
extern crate countdown;

mod aliases;

pub use aliases::App;

// TODO everything below here shouldn't be here.
pub use aliases::commands::Rehash;
pub use aliases::commands::Exec;
pub use aliases::builders::AliasBuilder; // had to do this for the tests, why?
pub use aliases::models::Alias; // had to do this for the tests, why?
pub use aliases::models::Conditional; // had to do this for the tests, why?
pub use aliases::factories::AliasFactory; // had to do this for the tests, why?
pub use aliases::collections::Aliases; // had to do this for the tests, why?
pub use aliases::factories::ShimFileFactory; // had to do this for the tests, why?
