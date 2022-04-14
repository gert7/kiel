use std::{fs::File, io::Write};

use chrono::Utc;
use fs2::FileExt;

const LOCKFILE_NAME: &str = "/tmp/kiel_lockfile";

pub fn wait_for_file() -> File {
    println!("[LF] wait for file");
    let lockfile_full_name = format!("{}-{}", LOCKFILE_NAME, whoami::username());
    println!("[LF] lockfile full name {}", lockfile_full_name);
    let lockfile = File::create(&lockfile_full_name);

    let lockfile = match lockfile {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        },
    };

    let lock = lockfile.lock_exclusive();
    let mut lockfile = File::create(&lockfile_full_name).unwrap();
    let timestamp = Utc::now().timestamp();
    let pid_text = format!("{}:{}\n", std::process::id(), timestamp);
    lockfile.write(pid_text.as_bytes()).expect("Unable to write to lockfile!");
    lockfile.flush().ok();

    lock.map(|_| lockfile).unwrap()
}
