use tracing::{error};
use tracing_subscriber::FmtSubscriber;

mod errors;
mod modules;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_target(false)
        .with_file(false)
        .with_ansi(true)
        .with_thread_ids(false)
        .without_time()
        .compact()
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Error to set subscriber.");

    let matches = modules::build_cli().get_matches();

    match matches.subcommand() {
        Some(("install", sub)) => {
            if let Some(names) = sub.get_many::<String>("name") {
                for package in names {
                    modules::install::install_handle(package).await.unwrap();
                }
            } else {
                error!("Argument NAME not found.");
            }
        }       
        Some(("list", _)) => modules::list::list_handle().await.unwrap(),
        Some(("remove", sub)) => {
            if let Some(name) = sub.get_one::<String>("name") {
                modules::remove::remove_handle(name).await.unwrap();
            } else {
                error!("Argument NAME not found");
            }
        }
        Some(("search", sub)) => {
            if let Some(name) = sub.get_one::<String>("name") {
                modules::search::search_handle(name).await.unwrap();
            } else {
                error!("Argument NAME not found");
            }
        }
        _ => println!("Command not found"),
    
    }
}
