use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandArgs {
    #[arg(long, default_value_t = String::from("0.0.0.0"))]
    pub host: String,
    #[arg(long, default_value_t = 8000)]
    pub port: u16,
    #[arg(long, default_value_t = Protocol::Http)]
    pub scheme: Protocol,
}

#[derive(Debug, Clone, clap::ValueEnum, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum Protocol {
    Http,
    Https,
}
