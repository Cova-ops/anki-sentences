use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "anki-sentences")]
#[command(author = "Daniel Covarrubias Sánchez <portfolio.dacovasan.dev>")]
#[command(version)]
#[command(about = "CLI para repasar alemán estilo Anki (worte/setze), con DB SQLite y audio TTS.")]
pub struct Cli {
    /// Optional path to DB file (defaults to anki_satze.sql)
    #[arg(long)]
    pub db: Option<String>,

    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Import {
        #[command(subcommand)]
        cmd: ExportImportCmd,
    },
    Export {
        #[command(subcommand)]
        cmd: ExportImportCmd,
    },
    Review {
        #[command(subcommand)]
        cmd: ReviewCmd,
    },
    Audio {
        #[command(subcommand)]
        cmd: AudioCmd,
    },
    Profile {
        #[command(subcommand)]
        cmd: ProfileCmd,
    },
}

#[derive(ValueEnum, Debug, Clone)]
pub enum TypeFile {
    JSON,
    CSV,
}

#[derive(Subcommand, Debug)]
pub enum ExportImportCmd {
    Worte {
        #[arg(long)]
        path: String,

        #[arg(long, value_enum)]
        type_file: TypeFile,
    },
    Setze {
        #[arg(long)]
        path: String,

        #[arg(long, value_enum)]
        type_file: TypeFile,
    },
}

#[derive(ValueEnum, Debug, Clone)]
pub enum ReviewWorteSection {
    OnlyNew,
    OnlyReview,
    SeparableVerbs,
    ReflexiveVerbs,
    Level,

    NewAndReview,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum ReviewSetzeSection {
    OnlyNew,
    OnlyReview,
    SpecificTopic,
    Level,

    NewAndReview,
}

#[derive(Subcommand, Debug)]
pub enum ReviewCmd {
    Worte {
        #[arg(long, value_enum, default_value_t = ReviewWorteSection::NewAndReview)]
        section: ReviewWorteSection,
        #[arg(long, default_value_t = 20)]
        batch: usize,
        #[arg(long, action = clap::ArgAction::SetFalse)]
        no_shuffle: bool,
    },
    Setze {
        #[arg(long, value_enum, default_value_t = ReviewSetzeSection::NewAndReview)]
        section: ReviewSetzeSection,
        #[arg(long, default_value_t = 20)]
        batch: usize,
        #[arg(long, action = clap::ArgAction::SetFalse)]
        no_shuffle: bool,
    },
}

#[derive(ValueEnum, Debug, Clone)]
pub enum AudioTarget {
    Worte,
    Setze,
}

#[derive(Subcommand, Debug)]
pub enum AudioCmd {
    Prefetch {
        target: AudioTarget,
        #[arg(long, default_value_t = 1000)]
        limit: u32,
        #[arg(long, default_value = "es")]
        lang: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum ProfileCmd {
    New,
    Use,
    Remove,
}
