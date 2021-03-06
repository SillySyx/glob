use std::io::{Read, Write, Seek, SeekFrom};
use std::error::Error;

use crate::entry::Entry;

pub trait Archive {
    fn add_entry(&mut self, name: &str, data: &[u8]) -> Result<Entry, Box<dyn Error>>;
    fn replace_entry(&mut self, entry: &Entry, name: &str, data: &[u8]) -> Result<Entry, Box<dyn Error>>;
    
    fn find_entry(&mut self, name: &str) -> Result<Entry, Box<dyn Error>>;

    fn read_entries(&mut self) -> Result<Vec<Entry>, Box<dyn Error>>;
    fn read_entry_data(&mut self, entry: &Entry) -> Result<Vec<u8>, Box<dyn Error>>;

    fn remove_entry(&mut self, entry: &Entry) -> Result<(), Box<dyn Error>>;
}

pub fn write_entry_to_archive<Archive: Write + Seek>(archive: &mut Archive, name: &str, data: &[u8]) -> Result<Entry, Box<dyn Error>> {
    let position = archive.seek(SeekFrom::End(0))?;

    archive.write(&name.len().to_be_bytes())?;
    archive.write(&name.as_bytes())?;
    archive.write(&data.len().to_be_bytes())?;
    archive.write(data)?;

    let entry = Entry {
        name: name.to_owned(),
        position,
        data_length: data.len(),
    };

    Ok(entry)
}

pub fn read_entries_from_archive<Archive: Read + Seek>(archive: &mut Archive) -> Result<Vec<Entry>, Box<dyn Error>> {
    let mut entries = vec![];

    let end_position = archive.seek(SeekFrom::End(0))?;
    archive.seek(SeekFrom::Start(0))?;

    loop {
        let current_position = archive.seek(SeekFrom::Current(0))?;
        if current_position == end_position {
            break;
        }

        let mut buffer = [0u8; std::mem::size_of::<usize>()];
        archive.read(&mut buffer)?;
        let name_length = usize::from_be_bytes(buffer);

        let mut buffer = vec![0u8; name_length];
        archive.read(&mut buffer)?;
        let name = String::from_utf8(buffer)?;

        let mut buffer = [0u8; std::mem::size_of::<usize>()];
        archive.read(&mut buffer)?;
        let data_length = usize::from_be_bytes(buffer);

        archive.seek(SeekFrom::Current(data_length as i64))?;

        entries.push(Entry {
            name,
            position: current_position,
            data_length,
        });
    }

    Ok(entries)
}

pub fn read_entry_data<Archive: Read + Seek>(archive: &mut Archive, entry: &Entry) -> Result<Vec<u8>, Box<dyn Error>> {
    archive.seek(SeekFrom::Start(entry.position))?;
    archive.seek(SeekFrom::Current(std::mem::size_of::<usize>() as i64))?;
    archive.seek(SeekFrom::Current(entry.name.len() as i64))?;
    archive.seek(SeekFrom::Current(std::mem::size_of::<usize>() as i64))?;

    let mut buffer = vec![0u8; entry.data_length];
    archive.read(&mut buffer)?;

    Ok(buffer.to_vec())
}

pub fn find_entry_in_archive<Archive: Read + Seek>(archive: &mut Archive, name: &str) -> Result<Entry, Box<dyn Error>> {
    let entries = read_entries_from_archive(archive)?;

    for entry in entries {
        if entry.name == name {
            return Ok(entry);
        }
    }

    Err(Box::from("failed to find entry in archive"))
}

pub fn remove_entry_from_archive<Archive: Read + Write + Seek>(archive: &mut Archive, entry: &Entry) -> Result<u64, Box<dyn Error>> {
    let end_position = archive.seek(SeekFrom::End(0))?;
    
    let mut write_position = archive.seek(SeekFrom::Start(entry.position))?;

    let position_offset = entry.name.len() 
        + entry.data_length 
        + std::mem::size_of::<usize>() 
        + std::mem::size_of::<usize>();

    let mut read_position = archive.seek(SeekFrom::Current(position_offset as i64))?;
    
    let end_position = end_position - position_offset as u64;

    loop {
        if write_position == end_position {
            break;
        }

        archive.seek(SeekFrom::Start(read_position))?;

        let mut buffer = [0u8; 1024];

        let bytes_read = archive.read(&mut buffer)?;
        if bytes_read == 0 {
            return Err(Box::from("failed to read any more bytes!"));
        }

        read_position += bytes_read as u64;

        archive.seek(SeekFrom::Start(write_position))?;

        let bytes_written = archive.write(&buffer[..bytes_read])?;

        write_position += bytes_written as u64;
    }

    Ok(end_position)
}