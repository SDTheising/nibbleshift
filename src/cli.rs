use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "revver", about = "Encode or decode .rev files")]
pub struct Cli {
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long)]
    pub unrev: bool,
}
