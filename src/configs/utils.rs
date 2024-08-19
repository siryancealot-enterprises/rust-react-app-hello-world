/// Takes a enviornemnt variable name found in the root .env file assumed to be an integer,
/// retrireves it, and converts it to a number
pub fn get_env_var_as_number(var_name: &str) -> u32 {
    std::env::var(var_name)
        .unwrap_or_else(|_| panic!("{} {}", var_name, " must be set in .env file."))
        .parse()
        .expect("must be an integer ")
}
