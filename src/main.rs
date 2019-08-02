extern crate reqwest;
#[macro_use]
extern crate clap;

use std::collections::HashMap;

mod cli;

fn main() {
    let matches = cli::build_cli().get_matches();
    if matches.is_present("version"){
        print_version();
    }

    if let Some(ref matches) = matches.subcommand_matches("upload") {
        println!("used sub"); // subが指定されていればメッセージを表示
        // subflgのON/OFFで表示するメッセージを切り替え
        println!("subflg is {}", if matches.is_present("subflg") {"ON"} else {"OFF"});
    }
}

fn print_version() {
    println!("Version: {:#?}", crate_version!());
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
