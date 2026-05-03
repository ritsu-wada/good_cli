use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 褒めるコマンド
    Praise {
        /// 呼ばれたい名前
        #[arg(short, long, default_value = "マスター")]
        name: String,

        /// 褒め方の熱量
        #[arg(short, long, value_enum, default_value_t = Level::Normal)]
        level: Level,
    },
    /// 名前の登録（将来用）
    AddName { name: String },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Level {
    Casual,
    Normal,
    Extreme,
}
