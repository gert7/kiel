use std::{fs::File, io::Write};

use chrono::Utc;
use color_eyre::owo_colors::OwoColorize;
use fs2::FileExt;

const LOCKFILE_NAME: &str = "/tmp/kiel_lockfile";

pub fn wait_for_file() -> File {
    let lockfile = File::open(LOCKFILE_NAME);

    let mut lockfile = match lockfile {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        },
    };

    let lock = lockfile.lock_exclusive();
    let mut lockfile = File::create(LOCKFILE_NAME).unwrap();
    let timestamp = Utc::now().timestamp();
    let pid_text = format!("{}:{}\n", std::process::id(), timestamp);
    lockfile.write(pid_text.as_bytes()).expect("Unable to write to lockfile!");
    lockfile.flush().ok();

    lock.map(|_| lockfile).unwrap()
}
