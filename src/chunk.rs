use crate::chunk_type;
use chunk_type::ChunkType;
use std::{fmt, str::FromStr};
use crc::crc32;


pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
}

impl Chunk {

    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        return Chunk {
            chunk_type,
            data
        }
    }

    fn length(&self) -> usize {
        return self.data.len();
    }

    pub fn chunk_type(&self) -> &ChunkType {
        return &self.chunk_type;
    }

    fn data(&self) -> &[u8] {
        return &self.data
    }

    fn crc(&self) -> u32 {
        let crc_calculation_bytes: Vec<u8> = 
            self.chunk_type.bytes().iter()
            .chain(self.data.iter())
            .copied().collect();

        let checksum = crc32::checksum_ieee(crc_calculation_bytes.as_ref());
        return checksum;
    }

    fn data_as_string(&self) -> Result<String, &str> {
        let s = match std::str::from_utf8(&self.data) {
            Ok(v) => v,
            Err(_) => return Err("Failed")
        };
        
        let data = String::from_str(s).unwrap();
        return Ok(data)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        return self.length()
                .to_be_bytes().iter()
                .chain(self.chunk_type.bytes().iter())
                .chain(self.data.iter())
                .chain(self.crc().to_be_bytes().iter())
                .copied()
                .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 12 {
            return Err("Chunk is not complete");
        }

        //Byte count is checked before
        let length: [u8; 4] = value[..4].try_into().unwrap();
        let length =  u32::from_be_bytes(length);
        let length_usize = usize::try_from(length).unwrap();

        let chunk_type: [u8; 4] = value[4..=7].try_into().unwrap();
        let chunk_type = chunk_type::ChunkType::try_from(chunk_type)?;

        // chunk data length + 4 for length + 4 for chunk_type + 4 for CRC
        if value.len() != length_usize + 12 {
            return Err("Not correct bytes count");
        }

        let data = value[8..=length_usize + 7].to_vec();

        let chunk = Chunk {
            chunk_type,
            data,
        };

        let crc: [u8; 4] = value[length_usize + 8..].try_into().unwrap();
        let crc = u32::from_be_bytes(crc);
        let chunk_crc = chunk.crc();

        if crc != chunk_crc {
            return Err("Chunk crc is not correct");
        }

        return Ok(chunk);
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_str = std::str::from_utf8(&self.data).map_err(|_| std::fmt::Error)?;
        write!(f, "{}{}", self.chunk_type.to_string(), data_str)
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