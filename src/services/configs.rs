//! General purpose set of utililties for interacting with configuration values for in the .env file.

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
    std::env::var(var_name)
        .unwrap_or_else(|_| panic!("{} {} {}", "Missing", var_name, "in .env file."))
}
