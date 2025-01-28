use std::error::Error;

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

pub trait ConnectionFactory: Send + Sync + 'static {
    fn get_connection(&self) -> Result<PooledConnection<SqliteConnectionManager>, Box<dyn Error>>;
}

pub struct SqliteConnectionFactory {
    pool: Pool<SqliteConnectionManager>,
}

impl SqliteConnectionFactory {
    pub fn new(database_url: &str) -> Self {
        let manager = SqliteConnectionManager::file(database_url);
        let pool = Pool::new(manager).expect("Failed to create pool");
        Self { pool }
    }
}

impl ConnectionFactory for SqliteConnectionFactory {
    fn get_connection(&self) -> Result<PooledConnection<SqliteConnectionManager>, Box<dyn Error>> {
        let connection = self.pool.get()?;
        Ok(connection)
    }
}