#[derive(PartialEq,Eq,Debug,Clone)]
pub struct User {
    name: String,
    filename: String,
    enabled: bool,
}

impl User {

    pub fn new(name: String, enabled: bool) -> Self {
        let filename;
        if name == "default" {
            filename = String::from(".aliases");
        } else {
            filename = format!(".aliases-{}", &name)
        }
        User{filename: filename , name: name, enabled: enabled }
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

}
