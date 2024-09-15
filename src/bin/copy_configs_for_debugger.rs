use config::Config;
/// This utility generates two files: 1) copies .cargo/config.toml file and rewrites it into target/debug/debugger-config.env file,
/// and 2) generates a .cargo/test-config.toml overriden version and writes it to a target/debug/debugger-test-config.env file.
/// But Why????
///
///     1. We need our VSCode debugger to pass to the launch configruations in launch.json our app config values, which are located
///     in the .cargo directory in [*]config.toml files to the running application (like cargo run does
///     automatically). And the debugger launch config has a parameter called "envFile" to do this.
///     2. BUT our debugger, CodeLLDB (very popular, currently best Rust debbugger), doens't accept .toml files in it's envFile
///     parameter for our launch configuration. It barfs on the table syntax (e.g. [env]). Seems like a bug, and I've
///     (filed one)[https://github.com/vadimcn/codelldb/issues/1139].
///
/// So this utiltiy simply copies our .cargo/config.toml file, removes the table syntax of [env] and [alias] and then writes
/// it to our target/debug directory. It does this for a test-specific override version as well.  We rename both .env because
/// at this point that's basically what it is.
///
/// TODO SWY: This whole thing needs to be removed. As its brittle, hopefully the bug I filed resolves it, and plus its
/// inherently flawed as we lose any layered config file behavior: if we add a dev-config.toml we won't be able to use it and
/// merge it with the base config.toml.
use std::fs;

fn main() {
    // Generates the base config file for use in app debugger launch.conf targets.
    let mut config_toml_data =
        fs::read_to_string(".cargo/config.toml").expect("Unable to read file");

    config_toml_data.insert_str(0, "### THIS IS A COPIED FILE FROM .cargo/config.toml\n\n### SEE copy_config_for_debugger.rs FOR WHY AND HOW\n\n\n\n");

    let transformed_config_toml: String =
        config_toml_data.replace("[env]", "").replace("[alias]", "");

    fs::write("target/debug/debugger-config.env", transformed_config_toml)
        .expect("Unable to write file");

    // Now we need to generate a config file for our testing launch configuraitons, which means it should utilize the
    // test-config.toml override file. So we'll need to use the Config crate below to process the overrides and then
    // write out that file for the specific configurations in launch.json to use
    let settings = Config::builder()
        .add_source(config::File::with_name(".cargo/config.toml"))
        .add_source(config::File::with_name(".cargo/test-config.toml"))
        .build()
        .unwrap();

    let mut test_override_configs: String = "### THIS IS A GENERATED FILE FROM .cargo/config.toml AND .cargo/test-config.toml\n\n### SEE copy_config_for_debugger.rs FOR WHY AND HOW\n\n\n\n".to_string();

    // Only need to grab the pairs from the [env] secion of this file
    for pair in settings.get_table("env").unwrap().iter() {
        test_override_configs.push_str(
            format!(
                "{} = \"{}\"\n",
                pair.0,
                pair.1.to_owned().into_string().unwrap()
            )
            .as_str(),
        );
    }

    fs::write(
        "target/debug/debugger-test-config.env",
        test_override_configs,
    )
    .expect("Unable to write file");
}
