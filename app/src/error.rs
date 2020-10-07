use mobc_postgres::tokio_postgres;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    DBPoolError(#[from] mobc::Error<tokio_postgres::Error>),

    #[error(transparent)]
    DBMobcError(#[from] mobc_postgres::tokio_postgres::Error),

    // #[error(transparent)]
    // MobcError(#[from] mobc::Error<mobc_postgres::tokio_postgres::Error>),
    // #[error("error executing DB query: {0}")]
    // DBQueryError(#[from] tokio_postgres::Error),
    #[error(transparent)]
    ReadFileError(#[from] std::io::Error),
}
