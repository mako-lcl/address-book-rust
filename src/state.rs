use diesel::PgConnection;

use crate::{Error, PgPool, Result};

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        AppState { pool }
    }

    pub fn with_conn<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&mut PgConnection) -> Result<T>,
    {
        let mut conn = self.pool.get().map_err(|_| Error::DbConnectionError)?;
        f(&mut conn)
    }
}
