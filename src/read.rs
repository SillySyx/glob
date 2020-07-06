use crate::{Glob, FindGlobEntry};

use std::{
    io::{Read, Seek, SeekFrom},
    fs::{OpenOptions},
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
        let entry = self.find(|entry_name| entry_name == name)?;

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.file_path)?;

        file.seek(SeekFrom::Start(entry.start_index))?;
        file.seek(SeekFrom::Current(4))?; // size of name length
        file.seek(SeekFrom::Current(i64::try_from(entry.name_length)?))?;
        file.seek(SeekFrom::Current(8))?; // size of content length

        let mut data = vec![0u8; entry.data_length as usize];
        file.read(&mut data[..entry.data_length as usize])?;

        Ok(data)
    }

    fn read_as<T>(&self, name: &str) -> Result<T, Box<dyn Error>> where T: serde::de::DeserializeOwned {
        let bytes = self.read(name)?;
        let d = bytes.clone();
        let instance: T = bincode::deserialize(&d)?;
        Ok(instance)
    }   
}