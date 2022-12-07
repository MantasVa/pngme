use std::fs;
use std::io::prelude::*;
use std::str::FromStr;
use std::fmt::Display;

use crate::args::{DecodePng, EncodePng, PrintPng, RemovePng };
use crate::chunk_type::ChunkType;
use crate::chunk::Chunk;
use crate::png::Png;
use crate::Result;

pub fn encode(command: EncodePng) -> Result<()> {
    let png_bytes = fs::read(&command.file_path)?;
    let mut png = Png::try_from(png_bytes.as_ref())?;

    let chunk_type = ChunkType::from_str(&command.chunk_type)?;
    let message = command.message.as_bytes();
    let chunk = Chunk::new(chunk_type, message.iter().copied().collect());

    png.append_chunk(chunk);

    if let Some(out) = command.output_path {
        let mut file = fs::File::create(out)?;
        file.write_all(&png.as_bytes())?;
    } else {
        fs::write(&command.file_path, png.as_bytes())?;
    }

    return Ok(());
}

pub fn decode(command: DecodePng) -> Result<Chunk> {
    let png_bytes = fs::read(command.file_path)?;
    let png = Png::try_from(png_bytes.as_ref())?;

    return match png.chunk_by_type(&command.chunk_type) {
        Some(chunk) => Ok(chunk.clone()),
        None => Err(Box::from(LibError::ChunkNotFound(command.chunk_type)))
    }
}

pub fn remove(command: RemovePng) -> Result<Chunk> {
    let png_bytes = fs::read(&command.file_path)?;
    let mut png = Png::try_from(png_bytes.as_ref())?;

    let chunk = png.remove_chunk(&command.chunk_type)?;
    fs::write(&command.file_path, png.as_bytes())?;

    return Ok(chunk);
}

pub fn print(command: PrintPng) -> Result<Png> {
    let png_bytes = fs::read(command.file_path)?;
    let png = Png::try_from(png_bytes.as_ref())?;

    return Ok(png);
}

#[derive(Debug)]
pub enum LibError {
    ChunkNotFound(String)
}

impl std::error::Error for LibError {}
impl Display for LibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LibError::ChunkNotFound(chunk_type) => {
                write!(f, "Chunk by type {} not found in png", chunk_type)
            }
        }
    }
}