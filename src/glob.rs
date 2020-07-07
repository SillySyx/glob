use std::error::Error;
use std::io::{Seek, SeekFrom, Read, Write, Cursor};
use std::fs::OpenOptions;

use super::GlobEntry;
use super::convert::{to_u32, to_u64, to_u32_array, to_u64_array};

pub struct Glob {
    entries: Vec<GlobEntry>,
}

impl Glob {
    pub fn new() -> Self {
        Self {
            entries: vec![],
        }
    }

    pub fn from(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        let mut entries = vec![];

        let mut buffer = Cursor::new(bytes);
        let buffer_length = buffer.seek(SeekFrom::End(0))?;
        buffer.seek(SeekFrom::Start(0))?;

        loop {
            let buffer_position = buffer.position();
            if buffer_length == buffer_position {
                break;
            }

            let name_length = read_name_length(&mut buffer)?;
            let name = read_name(&mut buffer, name_length)?;
            let data_length = read_data_length(&mut buffer)?;
            let data = read_data(&mut buffer, data_length)?;

            entries.push(GlobEntry {
                name,
                data: Some(data),
            });
        }

        Ok(Self {
            entries,
        })
    }

    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(file_path)?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        Self::from(&buffer)
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = vec![];

        for entry in &self.entries {
            let name_length = entry.name.len() as u32;
            buffer.write(&to_u32_array(name_length))?;

            let name_bytes = entry.name.as_bytes();
            buffer.write(name_bytes)?;

            if let Some(data) = &entry.data {
                let data_length = data.len() as u64;
                buffer.write(&to_u64_array(data_length))?;
                buffer.write(data)?;
            }
            else {
                buffer.write(&to_u64_array(0))?;
            }
        }

        Ok(buffer)
    }

    pub fn find<P: Fn(&GlobEntry) -> bool>(&self, predicate: P) -> Result<&GlobEntry, Box<dyn Error>> {
        for entry in &self.entries {
            if predicate(&entry) {
                return Ok(entry);
            }
        }

        Err(Box::from("not found"))
    }

    pub fn add(&mut self, entry: GlobEntry) {
        self.remove(|e| e.name == entry.name);

        self.entries.push(entry);
    }

    pub fn remove<P: Fn(&GlobEntry) -> bool>(&mut self, predicate: P) -> Option<()> {
        let index = self.entries
            .iter()
            .position(predicate)?;

        self.entries.remove(index);

        Some(())
    }
}

fn read_name_length(buffer: &mut Cursor<&[u8]>) -> Result<u32, Box<dyn Error>> {
    let mut data = [0u8; 4];
    buffer.read(&mut data)?;

    Ok(to_u32(&data))
}

fn read_name(buffer: &mut Cursor<&[u8]>, name_length: u32) -> Result<String, Box<dyn Error>> {
    let mut data = vec![0u8; name_length as usize];
    buffer.read(&mut data)?;

    Ok(String::from_utf8(data)?)
}

fn read_data_length(buffer: &mut Cursor<&[u8]>) -> Result<u64, Box<dyn Error>> {
    let mut data = [0u8; 8];
    buffer.read(&mut data)?;

    Ok(to_u64(&data))
}

fn read_data(buffer: &mut Cursor<&[u8]>, data_length: u64) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut data = vec![0u8; data_length as usize];
    buffer.read(&mut data)?;

    Ok(data)
}