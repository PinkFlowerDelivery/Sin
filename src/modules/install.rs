use clap::{Command, Arg};
use serde_json;
use serde::{Serialize, Deserialize};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use flate2::read::GzDecoder;
use tar::Archive;
use reqwest::header::{HeaderMap, ACCEPT};

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

pub fn install_command() -> Command {
    Command::new("install")
        .alias("i")
        .about("Install package")
        .arg(Arg::new("name")
            .value_name("NAME")
            .required(true) 
        )
}

pub async fn install_handle(name: &str) {
    let meta = match reqwest::get(format!("http://localhost:3000/download/{}.json", name)).await {
        Ok(meta) => {
            match meta.text().await {
                Ok(meta) => meta,
                Err(_) => {
                    println!("Converting error.");
                    return;
                },
            } 
        },
        Err(_) => {
            println!("Package not found.");
            return;
        }
    };

    let json: Info = if let Ok(json) = serde_json::from_str(&meta) {
        json
    } else {
        println!("Failed to parse json.");
        return;
    };

    let mut map = HeaderMap::new();
    map.insert(ACCEPT, "application/octet-stream".parse().unwrap());

    let client = reqwest::Client::new();

    let file = if let Ok(resp) = client.get(json.url).headers(map).send().await {
        println!("File successfully received.");
        resp
    } else {
        println!("Error to get file.");
        return;
    };

    if file.status().is_success() {
        println!("Archive successfully download.");

        let bytes = if let Ok(bytes) = file.bytes().await {
            println!("Bytes writted.");
            bytes
        } else {
            println!("Error writing bytes.");
            return;
        };

        let mut empty_archive = if let Ok(empty_archive) = File::create(format!("{}.tar.gz", json.name)).await {
            println!("Empty archive successfully created.");    
            empty_archive
        } else {
            println!("Failed to create empty archive.");
            return
        };

        if let Ok(_) = empty_archive.write_all(&bytes).await {
            println!("Bytes writed to archive!");
        } else {
            println!("Failed to write bytes to archive.");
        };

        let tar_gz = if let Ok(tar_gz) = std::fs::File::open(format!("{}.tar.gz", json.name)) {
            println!("tar.gz archive opened.");
            tar_gz
        } else {
            println!("Error to open tar.gz archive.");
            return;
        };

        let tar = GzDecoder::new(tar_gz);
        let mut new_archive = Archive::new(tar);

        if let Ok(_) = new_archive.unpack("/usr/local/bin") {
            println!("tar.gz archive successfully unarchived");
        } else if let Err(e) = new_archive.unpack("/usr/local/bin") {
            println!("Failed to unarchive tar.gz archive. Error: {}", e);
        };
    }

}
