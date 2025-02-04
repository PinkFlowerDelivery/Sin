use clap::{Command, Arg};
use serde_json;
use serde::{Serialize, Deserialize};
use tokio::fs::{self, File};
use flate2::read::GzDecoder;
use tar::Archive;
use reqwest::header::{HeaderMap, ACCEPT};
use tracing::{info, error, warn, debug};
use indicatif::{ProgressBar, ProgressStyle};
use std::cmp::min;
use futures_util::stream::StreamExt;
use tokio::io::AsyncWriteExt;

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
    info!("Beginning {} installation", name);

    let meta = if let Ok(meta) = reqwest::get(format!("http://localhost:3000/download/{}.json", name)).await {
        if let Ok(meta) = meta.text().await {
            info!("Downloading package..."); 
            meta
        } else {
            error!("Converting error");
            return;
        }
    } else {
        error!("Package not found.");
        return;
    }; 

    let json: Info = if let Ok(json) = serde_json::from_str(&meta) {
        json
    } else {
        error!("Failed to parse json.");
        return;
    };

    let client_size = reqwest::Client::new();
    let response_size = if let Ok(response_size) = client_size.get(&json.url).send().await {
        debug!("Head request successfully sended");
        response_size
    } else {
        error!("Failed to send head request");
        return;
   };

    let total_size = if let Some(total_size) = response_size.content_length() {
        total_size
    } else {
        error!("Failed to get filesize.");
        return;
    };

    let mut map = HeaderMap::new();
    map.insert(ACCEPT, "application/octet-stream".parse().unwrap());

    let client = reqwest::Client::new();
    let file = if let Ok(resp) = client.get(json.url).headers(map).send().await {
        debug!("File successfully received.");
        resp
    } else {
        error!("Error to get file.");
        return;
    };

    let mut empty_archive = if let Ok(empty_archive) = File::create(format!("{}.tar.gz", json.name)).await {
        debug!("Empty archive successfully created.");    
        empty_archive
    } else {
        error!("Failed to create empty archive.");
        return
    };

    let bar = ProgressBar::new(total_size); 
    bar.set_style(ProgressStyle::default_bar()
        .template("{bar:40.cyan/blue} {percent}% {msg}")
        .unwrap()
        .progress_chars("#>-"));

    let mut downloaded: u64 = 0;
    let mut stream = response_size.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = if let Ok(chunk) = item {
            chunk
        } else {
            error!("Failed to get item");
            return;
        };
        if let Err(_) = empty_archive.write_all(&chunk).await {
            error!("Failed to write bytes to archive"); 
        }
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        bar.set_position(downloaded);
    }


    if file.status().is_success() {
        debug!("Archive successfully download.");

        let tar_gz = if let Ok(tar_gz) = std::fs::File::open(format!("{}.tar.gz", json.name)) {
            debug!("tar.gz archive opened.");
            tar_gz
        } else {
            error!("Error to open tar.gz archive.");
            return;
        };

        let tar = GzDecoder::new(tar_gz);
        let mut new_archive = Archive::new(tar);

        info!("Unpacking the archive...");
        if let Ok(_) = new_archive.unpack("/usr/local/bin") {
            info!("Archive successfully unpacked");
            if let Err(_) = fs::remove_file(format!("{}.tar.gz", json.name)).await {
                warn!("Failed to remove archive.");
            }
        } else if let Err(e) = new_archive.unpack("/usr/local/bin") {
            error!("Failed to unarchive tar.gz archive. Error: {}", e);
        };
    }

}


//Hi guys
