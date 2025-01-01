use clap::{Command, Arg};
use tokio::fs;
use tracing::{error, info};

pub fn remove_command() -> Command {
    Command::new("remove")
        .alias("rm")
        .about("Remove package")
        .arg(Arg::new("name")
            .value_name("NAME")
            .required(true)
        )
}

pub async fn remove_handle(name: &str) {
    if let Ok(_) = fs::remove_file(format!("/usr/local/bin/{}", name)).await {
        info!("Package removed.");
    } else {
        error!("Failed to remove package.");
    };
}
