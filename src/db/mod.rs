use crate::{
    error::{Error, Result},
    print_error, print_system_green, read_line, read_line_hidden, system_wait,
};
use diesel::{
    PgConnection,
    r2d2::{self, ConnectionManager},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use std::io;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub mod contacts;

pub struct DatabaseConfig {
    pub host: String,
    pub port: String,
    pub user: String,
    pub password: String,
}

impl DatabaseConfig {
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/address_book",
            self.user, self.password, self.host, self.port
        )
    }
}

pub fn connect(stdout: &mut io::Stdout, stdin: &io::Stdin) -> Result<DatabaseConfig> {
    // -- Get database config
    let host = read_line(stdout, stdin, "Hostname")?;
    let port = read_line(stdout, stdin, "Port")?;
    let user = read_line(stdout, stdin, "User")?;
    let password = read_line_hidden(stdout, "Password")?;

    Ok(DatabaseConfig {
        host: host.trim().to_string(),
        port: port.trim().to_string(),
        user: user.trim().to_string(),
        password: password.trim().to_string(),
    })
}

pub fn establish_connection(stdout: &mut io::Stdout, db_url: String) -> Result<PgPool> {
    print_system_green(stdout, "Establishing db connection...")?;
    // -- connect
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = match r2d2::Pool::builder().build(manager) {
        Ok(pool) => pool,
        Err(_) => {
            print_error(stdout, "Failed connecting to postgres!")?;
            return Err(Error::R2d2);
        }
    };

    // -- run migrations
    let mut conn = pool.get().map_err(|_| Error::R2d2)?;
    if conn.run_pending_migrations(MIGRATIONS).is_err() {
        print_error(stdout, "Failed running migrations!")?;
        return Err(Error::MigrationFailed);
    }
    print_system_green(stdout, "Success!")?;

    // -- sleep 1 sec
    system_wait(1);

    Ok(pool)
}
