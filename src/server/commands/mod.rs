use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandArgs {
    #[arg(long)]
    pub host: Option<String>,
    #[arg(long)]
    pub port: Option<u16>,
    #[arg(long)]
    pub scheme: Option<Protocol>,
}

#[derive(Debug, Clone, clap::ValueEnum, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum Protocol {
    Http,
    Https,
}
