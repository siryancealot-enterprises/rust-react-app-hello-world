use rust_react_app_hello_world::typescript::ts_utils;

// main.rs
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.is_empty() {
        eprintln!("Usage: runjs <file locaterd in ts_scripts directory>");
        std::process::exit(1);
    }

    ts_utils::execute_typescript(&args[1]);
}
