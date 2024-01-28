use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt;
use std::cmp::PartialEq;

struct ChunkType {
    arr: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType { arr: value })
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err("Invalid length");
        }

        let bytes = s.as_bytes();

        let mut arr = [0u8; 4];
        for (i, &byte) in bytes.iter().enumerate() {
            arr[i] = byte;
        }

        Ok(ChunkType { arr })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.arr)
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.arr == other.arr
    }
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.arr
    }
    fn is_valid(&self) -> bool {
        self.is_critical() && self.is_public() && self.is_reserved_bit_valid() && self.is_safe_to_copy()
    }
    fn is_critical(&self) -> bool {
        self.arr[0].is_ascii_uppercase()
    }
    fn is_public(&self) -> bool {
        self.arr[1].is_ascii_uppercase()
    }
    fn is_reserved_bit_valid(&self) -> bool {
        self.arr[2].is_ascii_uppercase()
    }
    fn is_safe_to_copy(&self) -> bool {
        self.arr[3].is_ascii_lowercase()
    }
}