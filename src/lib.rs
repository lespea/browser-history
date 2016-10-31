#![feature(conservative_impl_trait)]

pub mod profiles;

use std::io;
use std::iter::Iterator;
use std::path::PathBuf;

#[cfg(windows)]
pub mod platform {
    mod windows;
    pub use self::windows::get_profiles_iter;
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn get_profiles() -> Result<impl Iterator<Item=PathBuf>, io::Error> {
    platform::get_profiles_iter().map(|dir_iter| {
        dir_iter.filter_map(|dir| {
            dir.ok()
                .and_then(|path| {
                    path.metadata()
                        .ok()
                        .and_then(|info| if info.is_dir() {
                            Some(path.path())
                        } else {
                            None
                        })
                })
        })
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::env;
    use super::get_profiles;

    #[test]
    fn has_profiles() {
        match get_profiles() {
            Err(e) => panic!(e),
            Ok(mut dirs) => assert!(dirs.next().is_some()),
        }
    }

    // Set the env var `BROWSER_HISTORY_EXPECTED_USERS` to a comma separated string of
    // just the base dir names that you expect to find.  If this is not found or empty
    // then the test isn't run
    #[test]
    fn exptected_profiles() {
        if let Ok(paths) = env::var("BROWSER_HISTORY_EXPECTED_USERS") {
            let mut expected = HashSet::with_capacity(64);
            for path in paths.split(',') {
                expected.insert(path.to_string());
            }
            if expected.len() > 0 {
                let mut found = HashSet::with_capacity(expected.len());
                match get_profiles() {
                    Err(e) => panic!(e),
                    Ok(dirs) => {
                        for dir in dirs {
                            let name = dir.file_name().unwrap().to_str().unwrap().to_string();
                            assert!(expected.contains(&name),
                                    "Didn't find the profile '{}'",
                                    name);
                            found.insert(name);
                        }
                    }
                };

                if found.len() != expected.len() {
                    for profile in expected {
                        assert!(found.contains(&profile),
                                "Expected to see the profile '{}' but we didn't see it",
                                profile);
                    }
                }
            };
        };
    }
}
