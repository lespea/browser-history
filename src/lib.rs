pub mod profiles;

#[cfg(windows)]
pub mod platform {
    mod windows;
    pub use self::windows::get_profiles_iter;
}

pub use platform::get_profiles_iter;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::env;
    use super::get_profiles_iter;

    #[test]
    fn has_profiles() {
        match get_profiles_iter() {
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
                match get_profiles_iter() {
                    Err(e) => panic!(e),
                    Ok(dirs) => {
                        for dir in dirs {
                            match dir {
                                Err(e) => panic!(e),
                                Ok(path) => {
                                    if let Ok(info) = path.metadata() {
                                        if info.is_dir() {
                                            let profile = path.file_name().into_string().unwrap();
                                            assert!(expected.contains(&profile),
                                                    "Didn't find the profile '{}'",
                                                    profile);
                                            found.insert(profile);
                                        }
                                    }
                                }
                            };
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
