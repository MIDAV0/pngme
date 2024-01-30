use std::fmt;
use std::{convert::TryFrom, str::from_utf8};
use crate::{Error, Result};
use crate::chunk_type::ChunkType;

const CRCCONST: u32 = 2882656334;

#[derive(Debug, Clone)]
struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        Ok(Chunk { data: Vec::from(value) })
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

impl Chunk {
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let data_length = data.len() as u32;
        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.arr.iter())
            .chain(data.iter())
            .chain(CRCCONST.to_be_bytes().iter())
            .copied()
            .collect();
        Chunk::try_from(chunk_data.as_ref().unwrap())
    }
    fn length(&self) -> u32 {

    }
    fn chunk_type(&self) -> &ChunkType {

    }
    fn data(&self) -> &[u8] {

    }
    fn crc(&self) -> u32 {

    }
    fn data_as_string(&self) -> Result<String> {

    }
    fn as_bytes(&self) -> Vec<u8> {

    }
}