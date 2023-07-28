use crate::DB_CONNECTION_POOL;
use mongodb::Database;

pub fn get_global() -> Database {
    DB_CONNECTION_POOL.lock().unwrap().clone().unwrap()
}

pub fn set_global(new_database_connection: mongodb::Database) {
    DB_CONNECTION_POOL.lock().unwrap().get_or_insert(new_database_connection);
}
