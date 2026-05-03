mod cli;
mod data;
mod logic;

use clap::Parser;

fn main() {
    let args = cli::Cli::parse();

    // 実際の処理は logic モジュールに丸投げする
    if let Err(e) = logic::run(args) {
        eprintln!("エラーが発生しました: {}", e);
        std::process::exit(1);
    }
}
