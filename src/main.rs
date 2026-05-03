mod cli;
use clap::Parser;

fn main() {
    let args = cli::Cli::parse();

    match args.command {
        cli::Commands::Praise { name, level } => {
            // ここに「褒める」ロジックを実装する
            println!("Debug: 名前={}, レベル={:?}", name, level);
        }
        cli::Commands::AddName { name } => {
            // ここに「名前登録」ロジックを実装する
            println!("Debug: {} さんを登録しました（予定）", name);
        }
    }
}
