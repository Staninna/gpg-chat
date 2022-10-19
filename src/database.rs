// Imports
use configparser::ini::Ini;
use std::process::exit;
use tokio_rusqlite::Connection;

// Connect to database file or
pub async fn connect(appconfig: &Ini) -> Connection {
    // get database type
    let file_or_memory = match appconfig.get("database", "file_or_memory") {
        Some(file_or_memory) => {
            if file_or_memory == "file" {
                true
            } else if file_or_memory == "memory" {
                false
            } else {
                eprintln!("database.file_or_memory has to be either 'file' or 'memory'");
                exit(1);
            }
        }
        None => {
            eprintln!("Couldn't read database.file_or_memory from appconfig.ini");
            exit(1);
        }
    };

    // get database path
    let db_path = match appconfig.get("database", "path") {
        Some(path) => path,
        None => {
            eprintln!("Couldn't read database.path from appconfig.ini");
            exit(1);
        }
    };

    // Connect to database
    let connection = match file_or_memory {
        true => Connection::open(db_path).await,
        false => Connection::open_in_memory().await,
    };

    // return error or connection
    match connection {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Couldn't connect to database, Error: {}", e);
            exit(1);
        }
    }
}
