use crate::glob::Glob;

use std::{
    io::{Read, Seek, SeekFrom},
    fs::{File, OpenOptions},
    error::Error,
    convert::TryFrom,
};

use serde::de::DeserializeOwned;

pub trait ReadGlob {
    fn read(&self, name: &str) -> Result<Vec<u8>, Box<dyn Error>>;
    fn read_as<T>(&self, name: &str) -> Result<T, Box<dyn Error>> where T: DeserializeOwned;
}

impl ReadGlob for Glob {
    fn read(&self, name: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.file_path)?;

        match find(&mut file, name) {
            Ok(length) => {
                let mut data = vec![0u8; length as usize];
                file.read(&mut data[..length as usize])?;
                
                Ok(data)
            },
            Err(error) => Err(error),
        }
    }

    fn read_as<T>(&self, name: &str) -> Result<T, Box<dyn Error>> where T: serde::de::DeserializeOwned {
        let bytes = Self::read(self, name)?;
        let d = bytes.clone();
        let instance: T = bincode::deserialize(&d)?;
        Ok(instance)
    }   
}

fn stream_lenght(file: &mut File) -> Result<u64, Box<dyn Error>> {
    let length = file.seek(SeekFrom::End(0))?;
    file.seek(SeekFrom::Start(0))?;

    Ok(length)
}

fn stream_position(file: &mut File) -> Result<u64, Box<dyn Error>> {
    let pos = file.seek(SeekFrom::Current(0))?;

    Ok(pos)
}

fn find(file: &mut File, name: &str) -> Result<u64, Box<dyn Error>> {
    let file_lenght = stream_lenght(file)?;

    loop {
        let position = stream_position(file)?;
        if file_lenght == position {
            break;
        }

        let mut resource_name_length = [0u8; 4];
        file.read(&mut resource_name_length)?;

        let name_length = as_u32(&resource_name_length) as usize;

        let mut resource_name_bytes = vec![0u8; name_length];
        file.read(&mut resource_name_bytes[..name_length])?;

        let mut resource_content_length = [0u8; 8];
        file.read(&mut resource_content_length)?;

        let content_length = as_u64(&resource_content_length);

        let resource_name = String::from_utf8(resource_name_bytes)?;

        if resource_name == name {
            return Ok(content_length);
        }

        let seek_length = i64::try_from(content_length)?;
        file.seek(SeekFrom::Current(seek_length))?;
    }

    Err(Box::from("not-found"))
}

fn as_u32(bytes: &[u8; 4]) -> u32 {
    ((bytes[0] as u32) << 24) +
    ((bytes[1] as u32) << 16) +
    ((bytes[2] as u32) << 8)  +
    ((bytes[3] as u32) << 0)
}

fn as_u64(bytes: &[u8; 8]) -> u64 {
    ((bytes[0] as u64) << 56) +
    ((bytes[1] as u64) << 48) +
    ((bytes[2] as u64) << 40) +
    ((bytes[3] as u64) << 32) +
    ((bytes[4] as u64) << 24) +
    ((bytes[5] as u64) << 16) +
    ((bytes[6] as u64) << 8)  +
    ((bytes[7] as u64) << 0)
}