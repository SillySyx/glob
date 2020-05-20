use crate::glob::Glob;

use std::{
    io::{Write, Seek, SeekFrom},
    fs::{OpenOptions},
    error::Error,
};

use serde::ser::Serialize;

pub trait WriteGlob {
    fn write(&self, name: &str, data: Vec<u8>) -> Result<(), Box<dyn Error>>;
    fn write_as<T>(&self, name: &str, data: T) -> Result<(), Box<dyn Error>> where T: Serialize;
}

impl WriteGlob for Glob {
    fn write(&self, name: &str, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;

        // try to find name 

        file.seek(SeekFrom::End(0))?;

        let name_length = name.len() as u32;
        file.write(&transform_u32_to_array_of_u8(name_length))?;

        let name_bytes = name.as_bytes();
        file.write(name_bytes)?;

        let data_length = data.len() as u64;
        file.write(&transform_u64_to_array_of_u8(data_length))?;

        file.write(&data)?;
    
        Ok(())
    }

    fn write_as<T>(&self, name: &str, data: T) -> Result<(), Box<dyn Error>> where T: Serialize {
        let bytes = bincode::serialize(&data)?;
        
        Self::write(self, name, bytes)
    }
}

fn transform_u32_to_array_of_u8(x: u32) -> [u8; 4] {
    [
        ((x >> 24) & 0xff) as u8,
        ((x >> 16) & 0xff) as u8,
        ((x >> 8)  & 0xff) as u8,
        ((x >> 0)  & 0xff) as u8,
    ]
}

fn transform_u64_to_array_of_u8(x: u64) -> [u8; 8] {
    [
        ((x >> 56) & 0xff) as u8,
        ((x >> 48) & 0xff) as u8,
        ((x >> 40) & 0xff) as u8,
        ((x >> 32) & 0xff) as u8,
        ((x >> 24) & 0xff) as u8,
        ((x >> 16) & 0xff) as u8,
        ((x >> 8)  & 0xff) as u8,
        ((x >> 0)  & 0xff) as u8,
    ]
}