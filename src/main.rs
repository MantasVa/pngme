use args::{PngMeArgs, PngSubcommand};
use clap::Parser;

mod args;
mod commands;
mod chunk;
mod chunk_type;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args: PngMeArgs = PngMeArgs::parse();

    match args.command_type {
        PngSubcommand::Encode(e) => {
            let _ = commands::encode(e)?;
            println!("File was encoded successfully!");

            return Ok(())
        },
        PngSubcommand::Decode(d) => {
            let chunk = commands::decode(d)?;
            println!("Chunk was found, chunk value {}", chunk);

            return Ok(());
        },
        PngSubcommand::Remove(r) => {
            let chunk = commands::remove(r)?;
            println!("Chunk was removed {}", chunk);

            return Ok(());
        },
        PngSubcommand::Print(p) => {
            let png = commands::print(p)?;
            println!("{}", png);

            return Ok(());
        }
    };
}