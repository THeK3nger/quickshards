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
            let dailiy_file_path = Path::new(&config.daily_path).join(daily_file);

            // Assemble the message
            let timestamp = date.format("%H:%M");
            let message = env::args().collect::<Vec<String>>()[1..].join(" ");
            let total_message = format!("\n- {}\n\t- {}", timestamp, message);

            // Append the message at the end of the file.
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(dailiy_file_path)
                .unwrap();

            if let Err(e) = writeln!(file, "{}", total_message) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
        Err(error) => panic!("{:#?}", error),
    }
}
