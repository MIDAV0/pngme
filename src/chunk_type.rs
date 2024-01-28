use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt;

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