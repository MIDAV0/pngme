use std::fmt;
use std::str::FromStr;
use std::{convert::TryFrom, str::from_utf8};
use crate::chunk_type::ChunkType;
use crate::{chunk::Chunk, chunk_type, Error, Result};
use std::io::{BufReader, Read};
use std::path::Path;
use std::fs;

#[derive(Debug)]
pub struct PngError {
    message: String,
}

impl PngError {
    fn boxed(message: String) -> Box<Self> {
        Box::new(Self {message})
    }
}

impl fmt::Display for PngError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bad chunk: {}", self.message)
    }
}

impl std::error::Error for PngError {}

struct Png {
    chunks: Vec<Chunk>,
}

impl TryFrom<&[u8]> for Png {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let mut reader = BufReader::new(value);
        let mut header: [u8; 8] = [0; 8];
        reader.read_exact(&mut header)?;
        
        if header != Png::STANDARD_HEADER {
            return Err(PngError::boxed(format!(
                "Signature is not valid"
            )));
        }

        let mut chunks = Vec::new();
        let mut length = [0; 4];
        while let Ok(()) = reader.read_exact(&mut length) {
            let pos = 4 + u32::from_be_bytes(length) + 4;
            let mut buffer = vec![0; usize::try_from(pos)?];
            reader.read_exact(&mut buffer)?;
            let all_bytes: Vec<u8> = length
                .iter()
                .copied()
                .chain(buffer.into_iter())
                .collect();
            let chunk = Chunk::try_from(all_bytes.as_slice())?;
            chunks.push(chunk);
        }
        Ok(Png::from_chunks(chunks))
    }
}

impl fmt::Display for Png {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Png Error")
    }
}

impl Png {
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    fn from_chunks(chunks: Vec<Chunk>) -> Png {
        Png { chunks: chunks }
    }
    fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }
    fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {
        if let Some(pos) = self
            .chunks
            .iter()
            .position(|c| *c.chunk_type() == ChunkType::from_str(chunk_type).unwrap())
        {
            Ok(self.chunks.remove(pos))
        } else {
            Err(PngError::boxed(format!("No matching chunk found")))
        }
    }
    fn header(&self) -> &[u8; 8] {
        &Png::STANDARD_HEADER
    }
    fn chunks(&self) -> &[Chunk] {
        &self.chunks.as_slice()
    }
    fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        // for (i, chunk) in self.chunks.iter().enumerate() {
        //     if chunk.chunk_type == ChunkType::from_str(chunk_type).unwrap() {
        //         return Some(chunk);
        //     }
        // }
        // None
        self.chunks.iter().find(|c| *c.chunk_type() == ChunkType::from_str(chunk_type).unwrap())
    }
    fn as_bytes(&self) -> Vec<u8> {
        let chunks: Vec<u8> = self.chunks.iter().map(|c| c.as_bytes()).flatten().collect();
        self.header()
            .iter()
            .chain(chunks.iter())
            .copied()
            .collect()
    }
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = &fs::read(path)?;
        Ok(Png::try_from(file.as_slice())?)
    }
}

