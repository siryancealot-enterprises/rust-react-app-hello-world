//! General purpose set of utililties for interacting with configuration values for in the .env file.

use std::env;

/// Loads a enviornemnt variable in the root .env file, validates its required exsistence,
/// and converts it to a number. Panics if not found or not an integer.
pub fn get_env_var_as_number_or_panic(var_name: &str) -> u32 {
    get_env_var_or_panic(var_name)
        .parse::<u32>()
        .unwrap_or_else(|_| panic!("{} {}", var_name, "must be an integer in .env file."))
}

/// Loads a enviornemnt variable in the root .env file and validates its required exsistence.
/// Panics if not found or not an integer.
pub fn get_env_var_or_panic(var_name: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| panic!("{} {} {}", "Missing", var_name, "in .env file."))
}

#[cfg(test)]
mod tests {

    use super::*;
    use dotenv::dotenv;
    use pretty_assertions::assert_eq;

    #[test]
    #[should_panic(expected = "Missing BLAH in .env file.")]
    fn configs_validate_get_env_var_or_panic_panics() {
        dotenv().ok();
        get_env_var_or_panic("BLAH");
    }

    #[test]
    fn configs_validate_get_env_var_or_panic() {
        dotenv().ok();
        assert_eq!("postgres", get_env_var_or_panic("DATABASE_USER"));
    }
}
