use configparser::ini::Ini;
use std::{fs::create_dir, path::Path, process::exit};

// Write the default appconfig
fn default_appconfig() -> Ini {
    // Make the .config directory
    if !Path::new(".config").exists() {
        match create_dir(".config") {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Couldn't make new directory, Error: {}", e);
                exit(1);
            }
        }
    }

    // Create a new ini file
    let mut appconfig = Ini::new();

    // Hard code the default values

    // Username section
    appconfig.set(
        "username",
        "regex",
        Some(r"^[a-zA-Z0-9_-]{5,10}$".to_string()),
    );
    appconfig.set(
        "username", 
        "comment", 
        Some("The username has to be between 5 and 10 characters long and can only contain letters, numbers, underscores and dashes".to_string())
    );

    // Password section
    appconfig.set("password", "salt", Some("CHANGE_ME".to_string()));
    appconfig.set(
        "password",
        "salt_path",
        Some(".config/SALT_DO_NOT_DELETE".to_string()),
    );

    // Database section
    appconfig.set("database", "path", Some("database.db".to_string()));
    appconfig.set("database", "file_or_memory", Some("file".to_string()));

    // Write the ini file
    match appconfig.write(".config/appconfig.ini") {
        Ok(_) => println!("Appconfig created"),
        Err(e) => {
            eprintln!("Couldn't write new appconfig.ini, Error: {}", e);
            exit(1)
        }
    }

    appconfig
}

// Check appconfig and add default keys to appconfig if they don't exist
fn check_appconfig() -> Ini {
    // Init variables
    let mut fixed = false;

    // Read appconfig
    let mut appconfig = Ini::new();
    match appconfig.load(".config/appconfig.ini") {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Couldn't read appconfig.ini, Error: {}", e);
            exit(1)
        }
    }

    // Has to match with the hardcoded default values in default_appconfig()

    // Check/fix username section
    check_fix(
        &mut appconfig,
        "username",
        "regex",
        r"^[a-zA-Z0-9_-]{5,10}$",
        &mut fixed,
    );
    check_fix(
        &mut appconfig,
        "username",
        "comment",
        "The username has to be between 5 and 10 characters long and can only contain letters, numbers, underscores and dashes",
        &mut fixed,
    );

    // Check/fix password section
    check_fix(&mut appconfig, "password", "salt", "CHANGE_ME", &mut fixed);
    check_fix(
        &mut appconfig,
        "password",
        "salt_path",
        ".config/SALT_DO_NOT_DELETE",
        &mut fixed,
    );

    // Check/fix database section
    check_fix(
        &mut appconfig,
        "database",
        "path",
        "database.db",
        &mut fixed,
    );
    check_fix(
        &mut appconfig,
        "database",
        "file_or_memory",
        "file",
        &mut fixed,
    );

    // Write the ini file
    if fixed {
        match appconfig.write(".config/appconfig.ini") {
            Ok(_) => println!("Appconfig fixed"),
            Err(e) => {
                eprintln!("Couldn't overwrite appconfig.ini, Error: {}", e);
                exit(1)
            }
        }
    }

    appconfig
}

// Helper function for check_appconfig
fn check_fix(appconfig: &mut Ini, section: &str, key: &str, value: &str, fixed: &mut bool) {
    if appconfig.get(section, key).is_none() {
        appconfig.set(section, key, Some(value.to_string()));
        if !*fixed {
            *fixed = true;
        }
    }
}

pub fn appconfig() -> Ini {
    let mut appconfig = Ini::new();
    let appconfig = match appconfig.load(".config/appconfig.ini") {
        Ok(_) => check_appconfig(),
        Err(_) => default_appconfig(),
    };

    appconfig
}
