use clap::Parser;
#[derive(Parser, Debug)]
#[command(
    name = "grep_lu",
    version = "1.0",
    about = "A minigrep, < > is NECESSAR while [ ] is OPTIONAL"
)]

pub struct Arguments {
    #[arg(help = "The query you want to pull.")]
    pub keyword: String,
    #[arg(help = "The path of the file.")]
    pub path: String,
    #[arg(short, help = "Case Insensitive Method.")]
    pub cis: Option<String>,
    #[arg(short, help = "Full World Match Method.")]
    pub fwm: Option<String>,
}

impl Arguments {
    pub fn new() -> Self {
        Arguments::parse()
    }
}
