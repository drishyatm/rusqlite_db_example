use rusqlite::{Connection, Result};
use std::fs;

fn db_exists(db_file_path: &str) -> bool {
// This function checks if a database file exists at the specified file path
    fs::metadata(db_file_path).is_ok()
}

fn main() -> Result<()>  {
    // File path for the employee database file
    let db_file_path = "employee.db";
    // check for DB Exists or not
    if !db_exists(&db_file_path) {         
        println!("No existing database! New database will be created in the path:{}", db_file_path);
    }
    else {        
        println!("Database Exists !!! Opened DB created at: {}", db_file_path);
    }
    // Connect to the database
    let _conn = match Connection::open(db_file_path) {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error connecting to database: {}.", e);
            return Err(e);
        }
    };    

    Ok(())
}