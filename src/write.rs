use crate::{Glob, FindGlobEntry, RemoveGlob};
use crate::convert::{to_u32_array, to_u64_array};

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
        if let Ok(_entry) = self.find(|entry_name| entry_name == name) {
            self.remove(name)?;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;

        file.seek(SeekFrom::End(0))?;

        let name_length = name.len() as u32;
        file.write(&to_u32_array(name_length))?;

        let name_bytes = name.as_bytes();
        file.write(name_bytes)?;

        let data_length = data.len() as u64;
        file.write(&to_u64_array(data_length))?;

        file.write(&data)?;
    
        Ok(())
    }

    fn write_as<T>(&self, name: &str, data: T) -> Result<(), Box<dyn Error>> where T: Serialize {
        let bytes = bincode::serialize(&data)?;
        
        Self::write(self, name, bytes)
    }
}