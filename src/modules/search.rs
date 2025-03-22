use clap::{Arg, Command};
use reqwest::{get};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::errors::Errors;

#[derive(Serialize, Deserialize)]
struct PackageStruct {
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
        .arg(Arg::new("name").value_name("NAME").required(true))
}

pub async fn search_handle(name: &str) -> Result<(), Errors>  {
    let addr = "http://192.168.0.108:3000/download";
    let package_json = get(format!("{addr}/{name}.json")).await?.text().await?;

    let package_meta: PackageStruct = serde_json::from_str(&package_json)?;

    println!(
        "Name: {}
Version: {}
Description: {}
Dependencies: {:?}
Source: {}
Size: {}
Last update: {}",
        package_meta.name,
        package_meta.version,
        package_meta.description,
        package_meta.dependencies,
        package_meta.source,
        package_meta.size,
        package_meta.last_update
    );
    Ok(())
}
