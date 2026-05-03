use crate::cli::{Cli, Commands, Level};

pub fn run(args: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        Commands::Praise { name, level } => {
            // ここでメッセージを生成
            let message = match level {
                Level::Casual => format!("{}、いい感じじゃん！その調子！", name),
                Level::Normal => {
                    format!("{}さん、あなたの努力は必ず報われます。素晴らしい！", name)
                }
                Level::Extreme => format!("{}様！全宇宙があなたの才能にひれ伏しています！！", name),
            };
            println!("{}", message);
        }
        Commands::AddName { name } => {
            println!("{} さんを登録しました（予定。次はここにserdeの出番）", name);
        }
    }
    Ok(())
}
