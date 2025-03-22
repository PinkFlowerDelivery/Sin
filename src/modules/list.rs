use clap::Command;
use tokio::fs::read_dir;

use crate::errors::Errors;

pub fn list_command() -> Command {
    Command::new("list")
        .alias("ls")
        .about("List of all package")
}

pub async fn list_handle() -> Result<(), Errors> {
    let mut dir = read_dir("/usr/local/bin").await?;
    while let Some(entry) = dir.next_entry().await? {
        println!("{}", entry.file_name().to_string_lossy());
    }
    Ok(())
}
