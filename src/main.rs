use tracing_subscriber::FmtSubscriber;
use tracing::{error, warn};
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
    tracing::subscriber::set_global_default(subscriber)
        .expect("Error to set subscriber.");

    let matches = modules::build_cli().get_matches();

    match matches.subcommand() {
        Some(("install", sub)) => {
            if let Some(name) = sub.get_one::<String>("name") {
                modules::install::install_handle(name).await;
            } else {
                error!("Argument NAME not found.");
                return;
            }
        }
        Some(("list", _)) => modules::list::list_handle().await,
        Some(("reinstall" , sub)) => {
            if let Some(name) = sub.get_one::<String>("name") {
                modules::reinstall::reinstall_handle(name).await;
            } else {
                error!("Argument NAME not found.");
                return;
            }
        },
        Some(("remove", sub)) => {
            if let Some(name) = sub.get_one::<String>("name") {
                modules::remove::remove_handle(name).await
            } else {
                error!("Argument NAME not found");
                return;
            }
        },
        Some(("search", sub)) => {
            if let Some(name) = sub.get_one::<String>("name") {
                modules::search::search_handle(name).await
            } else {
                error!("Argument NAME not found");
                return;
            }
        }
        _ => warn!("Command not found."),
    }
}
