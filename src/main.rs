use std::{env, fs::OpenOptions, io::Write, path::Path};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Settings {
    daily_path: String,
    daily_format: Option<String>,
}

fn main() {
    match envy::prefixed("QSHARD_").from_env::<Settings>() {
        Ok(config) => {
            let daily_format = config
                .daily_format
                .unwrap_or_else(|| String::from("%Y-%m-%d"));
            let date = chrono::offset::Local::now();
            let daily_file = format!("{0}.md", date.format(&daily_format));
            let daily_file_path = Path::new(&config.daily_path).join(&daily_file);

            // Assemble the message
            let timestamp = date.format("%H:%M");
            let message = env::args().collect::<Vec<String>>()[1..].join(" ");
            let total_message = format!("\n- {}\n\t- {}", timestamp, message);

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

            if let Err(e) = writeln!(file, "{}", total_message) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
        Err(error) => panic!("{:#?}", error),
    }
}
