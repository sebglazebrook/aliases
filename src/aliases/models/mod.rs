use std::cmp::Ordering;

#[derive(PartialOrd,Ord,PartialEq,Eq,Debug,Clone)]
pub struct Alias {
    pub name: String,
    pub command: String,
    pub confirm: bool,
    pub confirmation_message: String,
    pub conditional: Option<String>,
    pub unit_test: Option<String>,
}

impl Alias {

    pub fn new() -> Alias {
        Alias {
            name: String::new(),
            command: String::new(),
            confirm: false,
            confirmation_message: String::new(),
            conditional: None,
            unit_test: None
        }
    }
}
