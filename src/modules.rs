use clap::Command;

pub mod install;
pub mod list;
pub mod remove;
pub mod search;

pub fn build_cli() -> Command {
    Command::new("sin")
        .author("PinkFlowerDelivery")
        .version("1.0")
        .subcommand(install::install_command())
        .subcommand(list::list_command())
        .subcommand(remove::remove_command())
        .subcommand(search::search_command())
 
}
