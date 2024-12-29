use clap::{Command, Arg};
use tokio::fs;

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
        println!("Package removed.");
    } else {
        println!("Failed to remove package.");
    };
}
