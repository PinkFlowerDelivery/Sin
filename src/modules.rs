use clap::Command;

pub mod install;
pub mod list;
pub mod remove;
pub mod search;

pub fn build_cli() -> Command {
    Command::new("sin")
        .author("PinkFlowerDelivery")
        .version("1.0")
        .arg_required_else_help(true)
        .help_template("\
{usage-heading} {usage}

{all-args}{after-help}

Author: {author}
Version: {version}
")
    .subcommand(install::install_command())
    .subcommand(list::list_command())
        .subcommand(remove::remove_command())
        .subcommand(search::search_command())
 
}
