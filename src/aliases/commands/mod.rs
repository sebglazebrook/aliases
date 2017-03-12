mod init;
mod list;
mod add;
mod rehash;
mod exec;
mod users;
mod move_user;
mod clone_repo;
mod pull_repo;
mod enable_user;
mod disable_user;

pub use self::init::Init;
pub use self::list::List;
pub use self::add::Add;
pub use self::rehash::Rehash;
pub use self::exec::Exec;
pub use self::users::Users;
pub use self::move_user::MoveUser;
pub use self::clone_repo::CloneRepo;
pub use self::pull_repo::PullRepo;
pub use self::enable_user::EnableUser;
pub use self::disable_user::DisableUser;

pub trait AliasCommand {
    fn execute(&self) -> CommandResponse;
}

pub struct CommandResponse {
    code: u8,
    message: Option<String>,
}

impl CommandResponse {

    pub fn success() -> Self {
        Self::new(0, None)
    }

    pub fn new(code: u8, message: Option<String>) -> Self {
        CommandResponse { code: code, message: message }
    }


    pub fn is_error(&self) -> bool {
        self.code > 0
    }

    pub fn print_error_message(&self) {
        match self.message {
            None => {}
            Some(ref message) => { println!("An error occurred:\n {}", message); }
        }
    }

}
