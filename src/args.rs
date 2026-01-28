

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = "mechyvibes_rust",
    version,
    about = "A rust cli tool that make every keyboard to sound like a mechanical keyboard"
)]
#[clap(propagate_version = true)]
pub struct ArgParser {
    pub soundpack : String,

    #[arg(short, long)]
    pub volume: Option<u16>

}