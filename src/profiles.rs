use std::path::PathBuf;

pub enum Browser {
    Chrome,
    Firefox,
}

pub struct Profile {
    pub browser: Browser,
    pub user: String,
    pub path: PathBuf,
}

// pub fn get_profiles
