use std::{
    env,
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
    process::exit,
};

use std::process::Command;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Settings {
    obsidian_vault_path: String,
    daily_path: String,
    daily_format: Option<String>,
    working_memory_file_path: Option<String>,
}

/** The prefix for "watch log" entries. */
const WATCHED_PREFIX: &str = "@W ";
/** The prefix for "listened log" entries. */
const LISTENED_PREFIX: &str = "@A ";

fn edit_configuration_file() {
    let config_dir = dirs::config_dir().unwrap().join("QuickShards");
    fs::create_dir_all(&config_dir).unwrap();
    let config_file_path = Path::new(&config_dir).join("config.toml");
    Command::new("vim")
        .arg(config_file_path)
        .spawn()
        .expect("Cannot open VIM.")
        .wait()
        .unwrap();
}

fn load_configuration_file() -> Settings {
    let config_dir = dirs::config_dir().unwrap().join("QuickShards");
    fs::create_dir_all(&config_dir).unwrap();
    let config_file_path = Path::new(&config_dir).join("config.toml");
    match fs::read_to_string(&config_file_path) {
        Ok(config_str) => {
            let config: Settings = toml::from_str(&config_str).unwrap();
            config
        }
        Err(err) => {
            panic!(
                "Cannot open configuration file '{:?}'. ERROR IS: {:#?}",
                config_file_path, err
            )
        }
    }
}

fn append_line(file: &mut File, timestamp: &str, entry: &str) {
    let message = match entry {
        x if x.starts_with(WATCHED_PREFIX) => {
            format!("Movie🍿:: {}", x.replace(WATCHED_PREFIX, ""))
        }
        x if x.starts_with(LISTENED_PREFIX) => {
            format!("Music🎧:: {}", x.replace(LISTENED_PREFIX, ""))
        }
        _ => entry.to_owned(),
    };
    let total_message = format!("\n- {}\n\t- {}", timestamp, message);

    if let Err(e) = writeln!(file, "{}", total_message) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn main() {
    if env::args().nth(1).unwrap() == "edit" {
        edit_configuration_file();
        exit(0)
    }
    let config = load_configuration_file();
    let daily_format = config
        .daily_format
        .unwrap_or_else(|| String::from("%Y-%m-%d"));
    let date = chrono::offset::Local::now();
    let daily_file = format!("{0}.md", date.format(&daily_format));
    let daily_file_path = Path::new(&config.obsidian_vault_path)
        .join(&config.daily_path)
        .join(&daily_file);
    println!("{:?}", daily_file_path);
    // Assemble the message
    let timestamp = date.format("%H:%M");
    let message = env::args().collect::<Vec<String>>()[1..].join(" ");

    // Append the message at the end of the file.
    let mut file = match OpenOptions::new().append(true).open(daily_file_path) {
                Ok(f) => f,
                Err(err) => panic!(
                    "Cannot open file '{:?}'. Are you sure you have created it?\n
                    QS will not create a new file because you may want to use a template for that! :)\n
                    ERROR IS: {:#?}",
                    daily_file, err
                ),
            };
    append_line(&mut file, &timestamp.to_string(), &message)
}
