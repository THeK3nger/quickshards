use std::{
    env::temp_dir,
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
    process::exit,
};

use std::process::Command;

use serde::Deserialize;

use clap::Parser;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct Settings {
    obsidian_vault_path: String,
    daily_path: String,
    daily_format: Option<String>,
    working_memory_file_path: Option<String>,
    #[serde(default = "editor_default")]
    text_editor: String,
    tags: Vec<Tag>,
}

#[derive(Deserialize, Debug)]
struct Tag {
    tag: String,
    value: String,
}

fn editor_default() -> String {
    "vim".to_owned()
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Write on the Working Memory File.
    #[clap(short, long)]
    working_memory: bool,

    /// Quick open settings editor.
    #[clap(short, long)]
    edit_settings: bool,

    /// Edit Entry in Text Editor
    #[clap(short, long)]
    interactive: bool,

    /// The messge of the entry you want to add.
    text: Option<String>,
}

fn edit_configuration_file(editor: &str) {
    let config_dir = dirs::config_dir().unwrap().join("QuickShards");
    fs::create_dir_all(&config_dir).unwrap();
    let config_file_path = Path::new(&config_dir).join("config.toml");
    Command::new(editor)
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

fn handle_tags(tags: &Vec<Tag>, message: String) -> String {
    for tag in tags {
        if message.starts_with(&tag.tag) {
            return format!("{}:: {}", tag.value, message.replace(&tag.tag, ""));
        }
    }
    message
}

fn append_log_line(file: &mut File, timestamp: &str, entry: &str) {
    let total_message_body = entry
        .lines()
        .filter(|&x| !x.is_empty())
        .map(|x| format!("\t- {}", x))
        .collect::<Vec<String>>();

    let total_message = format!("\n- {}\n{}", timestamp, total_message_body.join("\n"));

    if let Err(e) = writeln!(file, "{}", total_message) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn append_line(file: &mut File, entry: &str) {
    let message = format!("- {}", entry);

    if let Err(e) = writeln!(file, "{}", message) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn interactive_editor(editor: &str) -> String {
    let tmp_dir = temp_dir();
    let filename = format!("{}.md", Uuid::new_v4());
    let tmp_file_path = Path::new(&tmp_dir).join(&filename);

    Command::new(editor)
        .arg(&tmp_file_path)
        .spawn()
        .expect("Cannot open VIM.")
        .wait()
        .unwrap();

    match fs::read_to_string(&tmp_file_path) {
        Ok(config_str) => config_str,
        Err(err) => {
            panic!(
                "Cannot open file '{:?}'. ERROR IS: {:#?}",
                tmp_file_path, err
            )
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let config = load_configuration_file();

    if cli.edit_settings {
        edit_configuration_file(&config.text_editor);
        exit(0)
    }

    let mut message = if !cli.interactive {
        cli.text.unwrap()
    } else {
        interactive_editor(&config.text_editor)
    };

    message = handle_tags(&config.tags, message);

    if cli.working_memory {
        let working_memory_path =
            Path::new(&config.obsidian_vault_path).join(&config.working_memory_file_path.unwrap());
        // Append the message at the end of the file.
        let mut file = match OpenOptions::new().append(true).open(&working_memory_path) {
            Ok(f) => f,
            Err(err) => panic!(
                "Cannot open file '{:?}'. ERROR IS: {:#?}",
                working_memory_path, err
            ),
        };
        append_line(&mut file, &message);
        exit(0);
    }

    let daily_format = config
        .daily_format
        .unwrap_or_else(|| String::from("%Y-%m-%d"));
    let date = chrono::offset::Local::now();
    let daily_file = format!("{0}.md", date.format(&daily_format));
    let daily_file_path = Path::new(&config.obsidian_vault_path)
        .join(&config.daily_path)
        .join(&daily_file);

    // Assemble the message
    let timestamp = date.format("%H:%M");

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

    append_log_line(&mut file, &timestamp.to_string(), &message)
}
