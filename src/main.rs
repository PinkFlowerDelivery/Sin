mod modules;

#[tokio::main]
async fn main() {
    let matches = modules::build_cli().get_matches();

    match matches.subcommand() {
        Some(("install", sub)) => {
            let _name = match sub.get_one::<String>("name") {
                Some(name) => modules::install::install_handle(name).await,
                None => {
                    println!("Argument NAME not found.");
                    return;
                },
            };
        }
        Some(("list", _)) => modules::list::list_handle().await,
        Some(("reinstall" , sub)) => {
            let _name = match sub.get_one::<String>("name") {
                Some(name) => modules::reinstall::reinstall_handle(name).await,
                None => {
                    println!("Argument NAME not found.");
                    return;
                }
            };
        },
        Some(("remove", sub)) => {
            let _name = match sub.get_one::<String>("name") {
                Some(name) => modules::remove::remove_handle(name).await,
                None => {
                    println!("Argument NAME not found.");
                    return;
                }
            };
        },
        Some(("search", sub)) => {
            let _name = match sub.get_one::<String>("name") {
                Some(name) => modules::search::search_handle(name).await,
                None => { 
                    println!("Argument NAME not found.");
                    return;
                },
            };
        }
        _ => println!("Command not found."),
    }
}
