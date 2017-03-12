use aliases::Config;

pub struct Directories;

impl Directories {

    pub fn new() -> Self {
        Directories {}
    }

    pub fn execute(&self) -> i32 {
        let directories = Config::load().directories();
        println!("{:?}", directories);
        0 // TODO make this a real exit code
    }

}
