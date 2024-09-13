/// This utility copies our .cargo/config.toml file and rewrites it into target/debug/debugger-config.env. Why????
///
///     1. We need our VSCode debugger to pass the configs in config.toml to the running application (like cargo run does
///     automatically), and our debugger launch config has a parameter called "envFile" to do this.
///     2. BUT our debugger, CodeLLDB (very popular, currently best Rust debbugger), doens't accept .toml files in it's envFile
///     parameter for our launch configuration. It barfs on the table syntax (e.g. [env]). Seems like a bug, and I've
///     (filed one)[https://github.com/vadimcn/codelldb/issues/1139].
///
/// So this utiltiy simply copies our .cargo/config.toml file, removes the table syntax of [env] and [alias] and then writes
/// it to our target/debug directory. We rename it .env because at this point that's basically what it is.
///
/// TODO SWY: This whole thing needs to be removed. As its brittle, hopefully the bug I filed resolves it, and plus its
/// inherently flawed as we lose any layered config file behavior: if we add a dev-config.toml we won't be able to use it and
/// merge it with the base config.toml.
use std::fs;

fn main() {
    let mut config_toml_data =
        fs::read_to_string(".cargo/config.toml").expect("Unable to read file");

    config_toml_data.insert_str(0, "### THIS IS A COPIED FILE FROM .cargo/config.toml\n\n### SEE copy_config_for_debugger.rs FOR WHY AND HOW\n\n\n\n");

    let transformed_config_toml: String =
        config_toml_data.replace("[env]", "").replace("[alias]", "");

    fs::write("target/debug/debugger-config.env", transformed_config_toml)
        .expect("Unable to write file");
}
