use std::str::FromStr;
use std::path::PathBuf;

use crate::args::{EncodeArgs, DecodeArgs, RemoveArgs, PrintArgs};
use crate::Result;
use crate::png::Png;
use crate::chunk_type::ChunkType;
use crate::chunk::Chunk;

pub fn encode(args: EncodeArgs) -> Result<()> {
    let output_file_path = args.output_file_path.unwrap_or_else(|| {
        args.image_path.clone()
    });
    let mut png: Png = Png::from_file(args.image_path)?;
    let chunk_type: ChunkType = ChunkType::from_str(&args.chunk_type).unwrap();
    let chunk: Chunk = Chunk::new(chunk_type, args.message.as_bytes().to_vec());
    png.append_chunk(chunk);
    png.write_file(output_file_path)?;
    Ok(())
}
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png: Png = Png::from_file(args.image_path)?;
    let chunk = png.chunk_by_type(&args.chunk_type).unwrap();
    println!("Chunk data: {}", chunk.data_as_string()?);
    Ok(())
}
pub fn remove(args: RemoveArgs) -> Result<()> {
    let output_file_path = args.image_path.clone();
    let mut png: Png = Png::from_file(args.image_path)?;
    png.remove_chunk(&args.chunk_type);
    png.write_file(output_file_path)?;
    Ok(())
}
pub fn print(args: PrintArgs) -> Result<()> {
    let png: Png = Png::from_file(args.image_path)?;
    println!("{:?}", png.as_bytes());
    Ok(())
}