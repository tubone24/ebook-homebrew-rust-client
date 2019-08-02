extern crate reqwest;

extern crate clap;

use clap::{App, Arg, SubCommand};

use std::collections::HashMap;

const VERSION: &str = "0.1.0";

fn main() {
    let app = App::new("clapex")
        .version("0.1.0")
        .author("tubone24 <tubo.yyyuuu@gmail.com>")
        .about("Ebook-homebrew Command Line Tools")
        .arg(Arg::with_name("version")
                 .help("CLI version")
                 .short("v")
                 .long("version")
        )
        .subcommand(SubCommand::with_name("sub")// サブコマンドを定義
            .about("sample subcommand")         // このサブコマンドについて
            .arg(Arg::with_name("subflg")       // フラグを定義
                     .help("sample flag by sub")     // ヘルプメッセージ
                     .short("f")                     // ショートコマンド
                     .long("flag")                   // ロングコマンド
            )
        );

    let matches = app.get_matches();
    if matches.is_present("version"){
        print_version();
    }

    if let Some(ref matches) = matches.subcommand_matches("sub") {
        println!("used sub"); // subが指定されていればメッセージを表示
        // subflgのON/OFFで表示するメッセージを切り替え
        println!("subflg is {}", if matches.is_present("subflg") {"ON"} else {"OFF"});
    }
}

fn print_version() {
    println!("Version: {:#?}", VERSION);
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
