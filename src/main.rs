mod cli;
mod command;

use cli::{Cli, Command};
use structopt::StructOpt;

use std::fs;

use ansi_term::Color;
use dirs::config_dir;

/// Creates the user configuration directories.
fn create_dirs() -> std::io::Result<()> {
    let base_path = config_dir().unwrap().join("stamp");

    let templates_path = base_path.join("templates");
    fs::create_dir_all(templates_path)
}

/// Runs the function associated with a CLI subcommand
fn run_command(command: &Command) -> Result<(), String> {
    match command {
        Command::List { verbose } => Command::list(*verbose),
        Command::Run {
            template,
            context,
            out,
        } => Command::run(template, context, out),
    }
}

fn main() {
    create_dirs().expect("Unable to create user config directories");

    let args = Cli::from_args();
    let command = &args.command;

    let result = run_command(command);
    if result.is_err() {
        println!(
            "{}: {}",
            Color::Red.bold().paint("error"),
            result.err().unwrap()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn global_dirs_created() {
        let result = create_dirs();
        assert!(result.is_ok());
        assert!(config_dir()
            .unwrap()
            .join(Path::new("stamp/templates"))
            .exists());
    }
}
