/// Convenience utility for working with Docker Composer lifecycle commands for the services defined in this
/// project's compose.yaml file.
///
/// This utility is called by the relevant aliased Cargo commands as defined in the [alias] section of ./cargo/config.toml,
/// with the following defined:
///
/// 1. cargo docker_tear_down: Stop and tear down the project's Containers
/// 2. cargo docker_stop: Simply stop the project's Containers
/// 3. cargo docker_up: Start or create project's Containers
///
use std::{env, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    let docker_command_alias = &args[1];

    let mut docker_args: Vec<&str> = vec!["--env-file", ".cargo/config.toml"];

    if docker_command_alias == "tear_down" {
        docker_args.push("down");
    } else if docker_command_alias == "stop" {
        docker_args.push("stop");
    } else if docker_command_alias == "up" {
        docker_args.push("up");
        docker_args.push("-d");
    } else {
        panic!(
            "unrecognized Docker command alias \"{0}\", check the [alias] section in .cargo/config.toml to fix",
            docker_command_alias
        );
    }

    Command::new("docker-compose")
        .args(docker_args)
        .status()
        .expect("failed to execute process");
}
