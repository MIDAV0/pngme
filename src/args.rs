use std::path::PathBuf;
use clap::Parser;

use crate::chunk_type::ChunkType;


#[derive(Parser, Debug)]
#[clap(name = "pngme")]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Parser, Debug)]
pub struct EncodeArgs {
    pub image_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output_file_path: Option<PathBuf>,
}

#[derive(Parser, Debug)]
pub struct DecodeArgs {
    pub image_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    pub image_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct PrintArgs {
    pub image_path: PathBuf,
}

