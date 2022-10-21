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
                eprintln!("database.file_or_memory has to be either \"file\" or \"memory\"");
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
    let conn = match file_or_memory {
        true => Connection::open(db_path).await,
        false => Connection::open_in_memory().await,
    };

    // return error or database
    match conn {
        Ok(conn) => return conn,
        Err(e) => {
            eprintln!("Couldn't connect to database, Error: {}", e);
            exit(1);
        }
    }
}

// Setup new database
pub async fn setup(conn: &Connection) {
    // Create table users if not exists
    conn.call(|conn| {
        match conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                    id integer NOT NULL,
                    username text NOT NULL UNIQUE,
                    public_key text NOT NULL UNIQUE,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    PRIMARY KEY (id)
                  )",
            [],
        )
        // Return error or nothing
        {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Couldn't create table users, Error: {}", e);
                exit(1);
            }
        }

        // Create table messages if not exists
        match conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                    id integer AUTO_INCREMENT,
                    sender_id integer NOT NULL,
                    receiver_id integer NOT NULL,
                    message text NOT NULL,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (sender_id) REFERENCES users (id),
                    FOREIGN KEY (receiver_id) REFERENCES users (id),
                    PRIMARY KEY (id)
                  )",
            [],
        )
        // Return error or nothing
        {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Couldn't create table messages, Error: {}", e);
                exit(1);
            }
        }
    })
    .await
}
