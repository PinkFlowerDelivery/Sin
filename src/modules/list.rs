use clap::Command;
use tokio::fs::read_dir;

pub fn list_command() -> Command {
    Command::new("list")
        .alias("ls")
        .about("List of all package")
}

pub async fn list_handle() {
    let mut dir = read_dir("/usr/local/bin").await.unwrap();
    while let Some(entry) = dir.next_entry().await.expect("Error") {
        println!("{}", entry.file_name().to_string_lossy());
    }   
}
