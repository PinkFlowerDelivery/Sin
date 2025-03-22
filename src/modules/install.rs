use clap::{Command, Arg};
use serde_json;
use serde::{Serialize, Deserialize};
use reqwest::{self, get, Client};
use reqwest::header::{HeaderMap, ACCEPT};
use tokio::fs::{self, File};
use tokio_tar::Archive;
use async_compression::tokio::bufread::GzipDecoder;
use tokio::io::{AsyncWriteExt, BufReader};
use futures_util::stream::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::cmp::min;

use crate::errors::Errors;

#[derive(Serialize, Deserialize)]
struct PackageStruct {
    name: String,
    version: String,
    description: String,
    url: String,
    dependencies: Vec<String>,
    size: String,
    source: String,
    last_update: String
}

pub fn install_command() -> Command {
    Command::new("install")
        .alias("i")
        .about("Install package")
        .arg(Arg::new("name")
            .value_name("name")
            .required(true)
            .num_args(1..)
        )
}

pub async fn install_handle(name: &str) -> Result<(), Errors> {
    let addr = "http://192.168.0.108:3000/download";
    let package_json = get(format!("{addr}/{name}.json")).await?.text().await?;

    // Parsing json
    let package_meta: PackageStruct = serde_json::from_str(&package_json)?;

    println!("Installing package {}", package_meta.name);
    println!("Size {}", package_meta.size);
    println!("Repo {}", package_meta.source);
    println!("Dependencies {:?}", package_meta.dependencies);

    let mut map = HeaderMap::new();
    map.insert(ACCEPT, "application/octet-stream".parse()?);

    // Parsing file bytes
    let client = Client::new();
    let package_file = client.get(&package_meta.url).headers(map).send().await?;

    let mut empty_archive = File::create(format!("{}.tar.gz", &package_meta.name)).await?;

let status = package_file.status(); // Saving status for if 
    let file_content_len = package_file.content_length();

    let progressbar = ProgressBar::new(file_content_len.unwrap());
    progressbar.set_style(ProgressStyle::default_bar()
        .template(" [{bar:40.white}] {bytes}/{total_bytes}, ETA: {elapsed}").unwrap()
        .progress_chars("##")
        );

    let mut downloaded: u64 = 0;

    // Paste bytes to empty archive
    let mut stream_bytes = package_file.bytes_stream();
    while let Some(item) = stream_bytes.next().await {
        let chunk = item?;
        empty_archive.write_all(&chunk).await?;
        let new = min(downloaded + (chunk.len() as u64), file_content_len.unwrap());
        downloaded = new;
        progressbar.set_position(downloaded);
    }
    println!("Integrity check...");

    if status.is_success() {
        let file = File::open(format!("{}.tar.gz", package_meta.name)).await?;

        // Unpacking archive
        println!("Unpacking archive...");
        let buf_reader = BufReader::new(file);
        let decoder = GzipDecoder::new(buf_reader);
        let mut archive = Archive::new(decoder); 
        archive.unpack("/usr/local/bin").await?;

        fs::remove_file(format!("{}.tar.gz", package_meta.name)).await?;
        println!("Succefully instaled {}", name);
         
    }
    Ok(())
}
