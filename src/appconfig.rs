use configparser::ini::Ini;
use std::{fs::create_dir, path::Path, process::exit};

// Write the default appconfig
fn default_appconfig() -> Ini {
    // Make the .config directory
    if !Path::new(".config").exists() {
        match create_dir(".config") {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(1);
            }
        }
    }

    // Create a new ini file
    let mut appconfig = Ini::new();

    // Hard code the default values

    // Username section
    appconfig.set("username", "min_length", Some(String::from("3")));
    appconfig.set("username", "max_length", Some(String::from("20")));
    appconfig.set("username", "regex", Some(String::from(r"^[a-zA-Z0-9_-]+$")));

    // Write the ini file
    match appconfig.write(".config/appconfig.ini") {
        Ok(_) => println!("Appconfig created"),
        Err(e) => {
            println!("Error: {}", e);
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
            println!("Error: {}", e);
            exit(1)
        }
    }

    // Has to match with the hardcoded default values in default_appconfig()

    // Check/fix username section
    check_fix(&mut appconfig, "username", "min_length", "3", &mut fixed);
    check_fix(&mut appconfig, "username", "max_length", "20", &mut fixed);
    check_fix(
        &mut appconfig,
        "username",
        "regex",
        r"^[a-zA-Z0-9_-]+$",
        &mut fixed,
    );

    // Write the ini file
    if fixed {
        match appconfig.write(".config/appconfig.ini") {
            Ok(_) => println!("Appconfig fixed"),
            Err(e) => {
                println!("Error: {}", e);
                exit(1)
            }
        }
    }

    appconfig
}

// Helper function for check_appconfig
fn check_fix(appconfig: &mut Ini, section: &str, key: &str, value: &str, fixed: &mut bool) {
    if appconfig.get(section, key).is_none() {
        appconfig.set(section, key, Some(String::from(value)));
        *fixed = true;
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
