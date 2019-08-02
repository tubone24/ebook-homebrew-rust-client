use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    app_from_crate!()
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(Arg::from_usage("-v --version 'CLI version'"))
        .subcommand(
            SubCommand::with_name("status")
                .about("check server status")
                .arg(
                    Arg::from_usage("--host [OPT] 'ebook-homebrew host URL'")
                        .default_value("https://ebook-homebrew.herokuapp.com")
                        .validator(check_url),
                )
                .arg(Arg::from_usage("--port [OPT] 'ebook-homebrew port'").default_value("443")),
        )
        .subcommand(
            SubCommand::with_name("upload")
                .about("upload image files")
                .arg(Arg::from_usage("<directory> 'image files directory'"))
                .arg(
                    Arg::from_usage("<extension> 'image files extension'")
                        .possible_values(&["jpg", "png", "gif"]),
                )
                .arg(
                    Arg::from_usage("--host [OPT] 'ebook-homebrew host URL'")
                        .default_value("https://ebook-homebrew.herokuapp.com")
                        .validator(check_url),
                )
                .arg(Arg::from_usage("--port [OPT] 'ebook-homebrew port'").default_value("443")),
        )
        .subcommand(
            SubCommand::with_name("convert")
                .about("convert image files to PDF")
                .arg(Arg::from_usage("<upload_id> 'upload ID'"))
                .arg(
                    Arg::from_usage("<extension> 'image files extension'")
                        .possible_values(&["jpg", "png", "gif"]),
                )
                .arg(
                    Arg::from_usage("--host [OPT] 'ebook-homebrew host URL'")
                        .default_value("https://ebook-homebrew.herokuapp.com")
                        .validator(check_url),
                )
                .arg(Arg::from_usage("--port [OPT] 'ebook-homebrew port'").default_value("443")),
        )
        .subcommand(
            SubCommand::with_name("download")
                .about("download converted PDF file")
                .arg(Arg::from_usage("<upload_id> 'upload ID'"))
                .arg(Arg::from_usage("<filename> 'PDF filename'"))
                .arg(
                    Arg::from_usage("--host [OPT] 'ebook-homebrew host URL'")
                        .default_value("https://ebook-homebrew.herokuapp.com")
                        .validator(check_url),
                )
                .arg(Arg::from_usage("--port [OPT] 'ebook-homebrew port'").default_value("443")),
        )
}

fn check_url(v: String) -> Result<(), String> {
    if !(v.contains("http://")) & !(v.contains("https://")) {
        return Err(String::from("Only use http or https proto"));
    }
    Ok(())
}
