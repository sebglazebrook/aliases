mod init;
mod list;
mod add;
mod remove;
mod rehash;
mod exec;
mod users;
mod move_user;
mod clone_repo;
mod pull_repo;
mod enable_user;
mod disable_user;
mod directories;

pub use self::init::Init;
pub use self::list::List;
pub use self::add::Add;
pub use self::remove::Remove;
pub use self::rehash::Rehash;
pub use self::exec::Exec;
pub use self::users::Users;
pub use self::move_user::MoveUser;
pub use self::clone_repo::CloneRepo;
pub use self::pull_repo::PullRepo;
pub use self::enable_user::EnableUser;
pub use self::disable_user::DisableUser;
pub use self::directories::Directories;

pub trait AliasCommand {
    fn execute(&self) -> CommandResponse;
}

pub enum CommandResponse {
    Success,
    Error { code: u8, message: Option<String> }
}

impl CommandResponse {

    pub fn success() -> Self {
        CommandResponse::Success
    }

    pub fn error(code: u8, message: Option<String>) -> Self {
        CommandResponse::Error{ code, message }
    }


    pub fn is_error(&self) -> bool {
        match self {
            CommandResponse::Error { .. } => true,
            _ => false,
        }
    }

    pub fn print_error_message(&self) {
        match self {
            CommandResponse::Error { code: _, message: Some(ref message) }  => { println!("An error occurred:\n {}", message); }
            _ => {}
        }
    }

}
