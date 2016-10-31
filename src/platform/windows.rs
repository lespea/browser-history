use std::env;
use std::fs::ReadDir;
use std::io;
use std::path::PathBuf;

pub fn get_user_profiles_iter() -> Result<ReadDir, io::Error> {
    let users_dir = env::var(r"USERPROFILE")
        .ok()
        .and_then(|d| PathBuf::from(d).parent().map(|p| p.to_owned()))
        .or_else(|| env::var(r"SYSTEMDRIVE").ok().map(|d| PathBuf::from(format!(r"{}\Users", d))))
        .unwrap_or_else(|| PathBuf::from(r"C:\Users"));

    users_dir.read_dir()
}
