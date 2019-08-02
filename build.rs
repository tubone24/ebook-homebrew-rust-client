#[macro_use]
extern crate clap;

use clap::{Shell};

include!("src/cli.rs");

fn main() {
    let mut app = build_cli();
    app.gen_completions("ebook-homebrew-rust-client", Shell::Bash, "./completions/bash/");
    app.gen_completions("ebook-homebrew-rust-client", Shell::Zsh, "./completions/zsh/");
    app.gen_completions("ebook-homebrew-rust-client", Shell::Fish, "./completions/fish/");
    app.gen_completions("ebook-homebrew-rust-client", Shell::PowerShell, "./completions/powershell/");
}
