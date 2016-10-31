use std::fs::ReadDir;
use std::io;
use std::path::PathBuf;

pub fn get_user_profiles_iter() -> Result<ReadDir, io::Error> {
    PathBuf::from("/home").read_dir()
}
