

mod args;
mod start;
mod keycode;
mod sound;

use clap::Parser;
use crate::args::ArgParser;
fn main() {
    let args = ArgParser::parse();
    let soundpack = args.soundpack;
    let volume = args.volume.or(Some(100)).unwrap();

    start::mechyvibes::start_mechyvibes(soundpack, volume);    

    
}