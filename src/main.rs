extern crate reqwest;
#[macro_use]
extern crate clap;

use reqwest::header;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{stdout, BufWriter, Write};
use std::time::Duration;

mod base64;
mod cli;
mod utils;

fn main() {
    let matches = cli::build_cli().get_matches();
    if matches.is_present("version") {
        print_version();
    }

    if let Some(ref matches) = matches.subcommand_matches("status") {
        let host: &str = matches.value_of("host").unwrap();
        let port: &str = matches.value_of("port").unwrap();
        check_status(&host, &port).expect("Server is down!");
    }

    if let Some(ref matches) = matches.subcommand_matches("upload") {
        let host: &str = matches.value_of("host").unwrap();
        let port: &str = matches.value_of("port").unwrap();
        let directory: &str = matches.value_of("directory").unwrap();
        let extension: &str = matches.value_of("extension").unwrap();
        upload_image(&host, &port, &directory, &extension).expect("Server is down!");
    }

    if let Some(ref matches) = matches.subcommand_matches("convert") {
        let host: &str = matches.value_of("host").unwrap();
        let port: &str = matches.value_of("port").unwrap();
        let upload_id: &str = matches.value_of("upload_id").unwrap();
        let extension: &str = matches.value_of("extension").unwrap();
        convert_pdf(&host, &port, &upload_id, &extension).expect("Server is down!");
    }

    if let Some(ref matches) = matches.subcommand_matches("download") {
        let host: &str = matches.value_of("host").unwrap();
        let port: &str = matches.value_of("port").unwrap();
        let upload_id: &str = matches.value_of("upload_id").unwrap();
        let filename: &str = matches.value_of("filename").unwrap();
        download_pdf(&host, &port, &upload_id, &filename).expect("Server is down!");
    }
}

fn print_version() {
    println!("Version: {:#?}", crate_version!());
}

fn check_status(host: &str, port: &str) -> Result<(), Box<std::error::Error>> {
    let mystdout = stdout();
    let mut buf = BufWriter::new(mystdout.lock());
    writeln!(buf, "============================").expect("Stdout Error");
    writeln!(buf, "Check server status").expect("Stdout Error");
    writeln!(buf, "============================").expect("Stdout Error");
    let url: String = format!("{}:{}/status", host, port);
    let client: reqwest::Client = reqwest::Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(60))
        .build()?;
    let resp: HashMap<String, String> = client
        .get(&url)
        .header(header::ACCEPT, "application/json")
        .send()?
        .json()
        .unwrap();
    if resp["status"] == "ok" {
        writeln!(buf, "Server is running...").expect("Stdout Error");
    } else {
        writeln!(buf, "Status is NG").expect("Stdout Error");
    }
    writeln!(buf, "API Version: {:#?}", resp["version"]).expect("Stdout Error");
    Ok(())
}

fn upload_image(
    host: &str,
    port: &str,
    directory: &str,
    extension: &str,
) -> Result<(), Box<std::error::Error>> {
    println!("============================");
    println!("Upload image file");
    println!("============================");
    let content_type = utils::change_extension_to_content_type(extension).to_string();
    let images_b64_string = format!(
        "{:?}",
        base64::create_images_b64(directory, &content_type, extension)
    );
    let url = format!("{}:{}/data/upload", host, port);
    let client = reqwest::Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(60))
        .build()?;
    let req_body = format!(
        r#"{{"contentType": "{content_type}","images": {images}}}"#,
        content_type = content_type,
        images = images_b64_string
    );
    let resp: HashMap<String, String> = client.post(&url).body(req_body).send()?.json().unwrap();
    println!("{:?}", resp);
    Ok(())
}

fn convert_pdf(
    host: &str,
    port: &str,
    upload_id: &str,
    extension: &str,
) -> Result<(), Box<std::error::Error>> {
    println!("============================");
    println!("Convert PDF");
    println!("============================");
    let content_type = utils::change_extension_to_content_type(extension).to_string();
    let upload_id_string = upload_id.to_string();
    let url = format!("{}:{}/convert/pdf", host, port);
    let client = reqwest::Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(60))
        .build()?;
    let req_body = format!(
        r#"{{"contentType": "{content_type}","uploadId": "{upload_id}"}}"#,
        content_type = content_type,
        upload_id = upload_id_string
    );
    let resp: HashMap<String, String> = client.post(&url).body(req_body).send()?.json().unwrap();
    println!("{:?}", resp);
    Ok(())
}

fn download_pdf(
    host: &str,
    port: &str,
    upload_id: &str,
    filename: &str,
) -> Result<(), Box<std::error::Error>> {
    println!("============================");
    println!("Download PDF");
    println!("============================");
    let upload_id_string = upload_id.to_string();
    let url = format!("{}:{}/convert/pdf/download", host, port);
    let client = reqwest::Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(60))
        .build()?;
    let req_body = format!(
        r#"{{"uploadId": "{upload_id}"}}"#,
        upload_id = upload_id_string
    );
    let mut resp = client.post(&url).body(req_body).send()?;
    let mut out = File::create(filename.to_string()).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
    println!("{:?}", resp);
    Ok(())
}
