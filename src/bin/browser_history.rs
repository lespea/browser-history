extern crate browser_history;

pub fn main() {
    if let Ok(profiles) = browser_history::get_users_profiles() {
        for profile in profiles {
            println!("{:?}", profile)
        }
    }
}
