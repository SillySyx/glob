use crate::{Glob, GlobEntry};
use crate::convert::{to_u32, to_u64};

use std::{
    io::{Read, Seek, SeekFrom},
    fs::{File, OpenOptions},
    error::Error,
    convert::TryFrom,
};

pub trait FindGlobEntry {
    fn find<Predicate: Fn(&str) -> bool>(&self, predicate: Predicate) -> Result<GlobEntry, Box<dyn Error>>;
}

impl FindGlobEntry for Glob {
    fn find<Predicate: Fn(&str) -> bool>(&self, predicate: Predicate) -> Result<GlobEntry, Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.file_path)?;
        
        let file_lenght = stream_lenght(&mut file)?;
    
        loop {
            let start_index = stream_position(&mut file)?;
    
            if file_lenght == start_index {
                break;
            }
    
            let mut entry = GlobEntry::new();
            entry.start_index = start_index;
    
            let (name_length, name) = read_name(&mut file)?;
            entry.name_length = name_length;

            let data_length = read_data_length(&mut file)?;
            entry.data_length = data_length;

            if predicate(&name) {
                return Ok(entry);
            }
    
            let seek_length = i64::try_from(entry.data_length)?;
            file.seek(SeekFrom::Current(seek_length))?;
        }
    
        Err(Box::from("not-found"))
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

fn read_name(file: &mut File) -> Result<(u32, String), Box<dyn Error>> {
    let mut buffer = [0u8; 4];
    file.read(&mut buffer)?;

    let name_length = to_u32(&buffer);

    let mut buffer = vec![0u8; name_length as usize];
    file.read(&mut buffer[..name_length as usize])?;

    let resource_name = String::from_utf8(buffer)?;

    Ok((name_length, resource_name))
}

fn read_data_length(file: &mut File) -> Result<u64, Box<dyn Error>> {
    let mut buffer = [0u8; 8];
    file.read(&mut buffer)?;

    let data_length = to_u64(&buffer);

    Ok(data_length)
}