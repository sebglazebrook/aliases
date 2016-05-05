use aliases::Config;

pub struct Users {
    config: Config,
}

impl Users {

    pub fn new(config: Config) -> Self {
        Users { config: config }
    }

    pub fn execute(&self) {
        println!("Users");
        println!("-----");
        println!("{:?}", self.config.users());
    }
}
