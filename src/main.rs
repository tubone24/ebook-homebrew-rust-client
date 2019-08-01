extern crate reqwest;

use std::collections::HashMap;

fn main() {
    check_status().expect("Server is down");
}

fn check_status() -> Result<(), Box<std::error::Error>> {
    println!("============================");
    println!("Welcome to Ebook-homebrew!!");
    println!("============================");
    let resp: HashMap<String, String> = reqwest::get("https://ebook-homebrew.herokuapp.com/status")?
        .json()?;
    if resp["status"]  == "ok" {
        println!("Server is running...");
    } else {
        println!("Status is NG");
    }
    println!("API Version: {:#?}", resp["version"]);
    Ok(())
}
