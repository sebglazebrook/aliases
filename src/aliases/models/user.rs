#[derive(Debug,Clone)]
pub struct User {
    name: String,
    filename: String,
}

impl User {

    pub fn new(name: String) -> Self {
        let filename;
        if name == "default" {
            filename = String::from(".aliases");
        } else {
            filename = format!(".aliases-{}", &name)
        }
        User{filename: filename , name: name }
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }

}
