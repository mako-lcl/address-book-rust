use derive_more::From;

pub type Result<T> = std::result::Result<T, Error>;

#[allow(dead_code)]
#[derive(Debug, From)]
pub enum Error {
    // -- DB errors
    MigrationFailed,
    R2d2,
    DbConnectionError,

    // -- External errors
    #[from]
    Diesel(diesel::result::Error),
    #[from]
    IO(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
