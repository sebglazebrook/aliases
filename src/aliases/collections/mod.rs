use aliases::models::Alias;

pub struct Aliases {
    raw_collection: Vec<Alias>,
}

impl Aliases {

    pub fn new(raw_collection: Vec<Alias>) -> Aliases {
        Aliases { raw_collection: raw_collection }
    }

    pub fn merge(&self, other: Aliases) -> Aliases {
        Aliases::new(vec![])
    }

}

