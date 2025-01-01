use clap::{Command, Arg};
use serde_json;
use serde::{Serialize, Deserialize};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use flate2::read::GzDecoder;
use tar::Archive;
use reqwest::header::{HeaderMap, ACCEPT};
use tracing::{info, error, debug};

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

pub fn reinstall_command() -> Command {
    Command::new("reinstall") 
        .alias("rei")
        .about("Reinstall package")
        .arg(Arg::new("name")
            .value_name("NAME")
            .required(true) 
        )
}

pub async fn reinstall_handle(name: &str) {
    if let Ok(_) = fs::remove_file(format!("/usr/local/bin/{}", name)).await {
        info!("Package removed.");
    } else {
        error!("Failed to remove package.");
    };

    let meta = match reqwest::get(format!("http://localhost:3000/download/{}.json", name)).await {
        Ok(meta) => {
            match meta.text().await {
                Ok(meta) => {
                    info!("Downloading package...");
                    meta
                },
                Err(_) => {
                    error!("Converting error.");
                    return;
                },
            } 
        },
        Err(_) => {
            error!("Package not found.");
            return;
        }
    };

    let json: Info = if let Ok(json) = serde_json::from_str(&meta) {
        json
    } else {
        error!("Failed to parse json.");
        return;
    };

    let mut map = HeaderMap::new();
    map.insert(ACCEPT, "application/octet-stream".parse().unwrap());

    let client = reqwest::Client::new();

    let file = if let Ok(resp) = client.get(json.url).headers(map).send().await {
        info!("File successfully received.");
        resp
    } else {
        error!("Error to get file.");
        return;
    };

    if file.status().is_success() {
        debug!("Archive successfully download.");

        let bytes = if let Ok(bytes) = file.bytes().await {
            debug!("Bytes writted.");
            bytes
        } else {
            error!("Error writing bytes.");
            return;
        };

        let mut empty_archive = if let Ok(empty_archive) = File::create(format!("{}.tar.gz", json.name)).await {
            debug!("Empty archive successfully created.");    
            empty_archive
        } else {
            error!("Failed to create empty archive.");
            return
        };

        if let Ok(_) = empty_archive.write_all(&bytes).await {
            debug!("Bytes writed to archive!");
        } else {
            error!("Failed to write bytes to archive.");
        };

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
                error!("Faile to remove archive.");
            }
        } else if let Err(e) = new_archive.unpack("/usr/local/bin") {
            error!("Failed to unarchive tar.gz archive. Error: {}", e);
        };
    }
}
