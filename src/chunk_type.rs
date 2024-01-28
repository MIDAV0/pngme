use std::convert::TryFrom;
use std::str::FromStr;

struct ChunkType {}

impl TryFrom<[u8; 4]>, FromStr for ChunkType {
    type Error = ();

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
    fn from_str(s: &str) -> Result<Self, Self::Error> {
        if s.len() != 4 {
            return Err(());
        }

        let bytes = s.as_bytes();

        let mut arr = [0u8; 4];
        for (i, &byte) in bytes.iter().enumerate() {
            arr[i] = byte;
        }

        Ok(ChunkType { arr })
    }

}