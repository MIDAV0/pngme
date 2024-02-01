mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    match args::PngMeArgs::parse() {
        args::PngMeArgs::Encode(encode_args) => commands::encode(encode_args),
        args::PngMeArgs::Decode(decode_args) => commands::decode(decode_args),
        args::PngMeArgs::Remove(remove_args) => commands::remove(remove_args),
        args::PngMeArgs::Print(print_args) => commands::print(print_args),
    }
}
