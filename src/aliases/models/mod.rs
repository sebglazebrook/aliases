pub struct Alias {
    pub name: String,
    pub command: String,
    pub confirm: bool,
    pub confirmation_message: String,
    pub conditional: Option<String>,
    pub unit_test: Option<String>,
}
