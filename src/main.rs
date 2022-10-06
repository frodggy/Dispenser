mod commands;
mod serverconfig;

use clap::{Command};
use commands::{create, start, stop, update, init};

fn main() {
    let cli = cli().get_matches();

    if let Some(_) = cli.subcommand_matches("create") {
        create();
    }

    if let Some(_) = cli.subcommand_matches("start") {
        start();
    }

    if let Some(_) = cli.subcommand_matches("stop") {
        stop();
    }

    if let Some(_) = cli.subcommand_matches("update") {
        update();
    }

    if let Some(_) = cli.subcommand_matches("init") {
        init();
    }
}

fn cli() -> Command {
    let cmd = Command::new("dispenser")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(Command::new("create").about("create new Dispenser server"))
        .subcommand(Command::new("start").about("start Dispenser server"))
        .subcommand(Command::new("stop").about("stop Dispenser server"))
        .subcommand(Command::new("update").about("stop Dispenser server"))
        .subcommand(Command::new("init").about("initalize new Dispenser with an existing server"));

    return cmd;
}
