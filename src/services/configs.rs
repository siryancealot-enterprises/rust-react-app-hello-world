//! General purpose set of utililties for interacting with configuration values for in the .cargo/[*]config.toml files.

use std::env;

/// Loads a enviornemnt variable in the root .env file, validates its required exsistence,
/// and converts it to a number. Panics if not found or not an integer.
pub fn get_env_var_as_number_or_panic(var_name: &str) -> u32 {
    get_env_var_or_panic(var_name)
        .parse::<u32>()
        .unwrap_or_else(|_| {
            panic!(
                "{} {}",
                var_name, "must be an integer in .cargo/config.toml file."
            )
        })
}

/// Loads a enviornemnt variable in the root .env file and validates its required exsistence.
/// Panics if not found or not an integer.
pub fn get_env_var_or_panic(var_name: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| {
        panic!(
            "{} {} {}",
            "Missing", var_name, "in .cargo/config.toml file."
        )
    })
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    #[should_panic(expected = "Missing BLAH in .cargo/config.toml file.")]
    fn configs_validate_get_env_var_or_panic_panics() {
        get_env_var_or_panic("BLAH");
    }

    #[test]
    fn configs_validate_get_env_var_or_panic() {
        assert_eq!("postgres", get_env_var_or_panic("database_user"));
    }
}
