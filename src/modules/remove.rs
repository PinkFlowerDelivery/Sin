use clap::{Arg, Command};
use tokio::fs;

use crate::errors::Errors;

pub fn remove_command() -> Command {
    Command::new("remove")
        .alias("rm")
        .about("Remove package")
        .arg(Arg::new("name").value_name("NAME").required(true))
}

pub async fn remove_handle(name: &str) -> Result<(), Errors> {
    fs::remove_file(format!("/usr/local/bin/{}", name)).await?;
    Ok(())
}
