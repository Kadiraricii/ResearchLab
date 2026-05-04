pub mod crud;
pub mod models;
pub mod schema;

use std::path::PathBuf;
use std::sync::Mutex;
use rusqlite::Connection;
use crate::error::AppError;

/// Thread-safe SQLite connection wrapper.
pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    /// Open (or create) the SQLite database at `path`, then run migrations.
    pub fn open(path: &PathBuf) -> Result<Self, AppError> {
        let conn = Connection::open(path)?;
        schema::migrate(&conn)?;
        Ok(Self { conn: Mutex::new(conn) })
    }

    /// Open an in-memory database (useful for tests and when no path is set).
    pub fn open_in_memory() -> Result<Self, AppError> {
        let conn = Connection::open_in_memory()?;
        schema::migrate(&conn)?;
        Ok(Self { conn: Mutex::new(conn) })
    }
}
