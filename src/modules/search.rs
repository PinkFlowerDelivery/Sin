use clap::{Command, Arg};
use serde::{Serialize, Deserialize};
use reqwest;
use serde_json;

#[derive(Serialize, Deserialize)]
struct Info {
    name: String,
    version: String,
    description: String,
    url: String,
    dependencies: Vec<String>,
    source: String,
    size: String,
    last_update: String,
}

pub fn search_command() -> Command {
    Command::new("search")
        .alias("s")
        .about("Search package")
        .arg(Arg::new("name")
            .value_name("NAME")
            .required(true)
        )
}

pub async fn search_handle(name: &str) {
    let meta = match reqwest::get(format!("http://localhost:3000/download/{}.json", name)).await {
        Ok(meta) => {
            match meta.text().await {
                Ok(meta) => meta,
                Err(_) => {
                    println!("Failed converting.");
                    return;
                },
            }
        },
        Err(_) => { println!("Failed to send request."); return },
    };

    let json: Info = if let Ok(json) = serde_json::from_str(&meta) {
        json
    } else {
        println!("Failed to parse json.");
        return;
    };
    println!("Name: {}
Version: {}
Description: {}
Dependencies: {:?}
Source: {}
Size: {}
Last update: {}", json.name, json.version, json.description, json.dependencies, json.source, json.size, json.last_update);
}
