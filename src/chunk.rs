use std::fmt;
use std::{convert::TryFrom, str::from_utf8};
use std::io::{BufReader, Read};
use crate::{chunk_type, Error, Result};
use crate::chunk_type::ChunkType;
use crc::crc32;

const MAXIMUM_LENGTH: u32 = (1 << 31) - 1;

// Errors implementation
#[derive(Debug)]
pub struct ChunkError {
    message: String,
}

impl ChunkError {
    fn boxed(message: String) -> Box<Self> {
        Box::new(Self {message})
    }
}

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bad chunk: {}", self.message)
    }
}

impl std::error::Error for ChunkError {}

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
        let mut reader = BufReader::new(value);
        let mut buf: [u8; 4] = [0; 4];
        reader.read_exact(&mut buf)?;
        
        // Length
        let length = u32::from_be_bytes(buf);
        //Error handling
        if length > MAXIMUM_LENGTH {
            return Err(ChunkError::boxed(format!(
                "Length is too long ({} > 2^31 - 1)",
                length
            )));
        }

        // Chunk Type
        reader.read_exact(&mut buf);
        let chunk_type: ChunkType = ChunkType::try_from(buf)?;
        
        // Data
        let mut chunk_data: Vec<u8> = vec![0; usize::try_from(length)?];
        reader.read_exact(&mut chunk_data);
        if chunk_data.len() != length.try_into()? {
            return Err(ChunkError::boxed(format!(
                "Data has invalid length. Expected {}, but got {}.",
                length, chunk_data.len()
            )));
        }

        reader.read_exact(&mut buf)?;
        let provided_crc = u32::from_be_bytes(buf);
        let actual_crc = crc::crc32::checksum_ieee(&[&chunk_type.bytes(), chunk_data. as_slice()].concat());
        if provided_crc != actual_crc {
            return Err(ChunkError::boxed(format!(
                "Invalid crc. Expected {}, but got {}.",
                actual_crc, provided_crc
            )));
        }

        Ok(Chunk {
            length: length,
            chunk_type: chunk_type,
            data: chunk_data,
            crc: provided_crc,
        })
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
        let crc = crc32::checksum_ieee(&[&chunk_type.bytes(), data.as_slice()].concat());
        Chunk {
            length: data.len() as u32,
            chunk_type: chunk_type,
            data: data,
            crc: crc,
        }
    }
    fn length(&self) -> u32 {
        self.length
    }
    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    fn data(&self) -> &[u8] {
        &self.data
    }
    fn crc(&self) -> u32 {
        self.crc
    }
    fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.data.clone()).unwrap())
    }
    fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.arr.iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}