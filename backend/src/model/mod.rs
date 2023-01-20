use thiserror::Error as ThisError;
mod db;
mod todo;

//re-export Db
pub use db::init_db;
pub use db::Db;
pub use todo::TodoMac;
pub use todo::TodoPatch;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Entity Not Found - {0}{1}")]
    EntityNotFound(&'static str, String),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
