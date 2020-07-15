use std::error::Error;
use std::io::{Cursor, Read, Seek, SeekFrom};

use crate::entry::Entry;
use crate::archive::{Archive, find_entry_in_archive, read_entries_from_archive, read_entry_data, remove_entry_from_archive, write_entry_to_archive};

#[derive(Debug)]
pub struct MemoryArchive {
    memory: Vec<u8>,
}

impl MemoryArchive {
    pub fn from(memory: Vec<u8>) -> Self {
        Self {
            memory,
        }
    }

    pub fn as_bytes(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        read_all_bytes(&mut Cursor::new(&mut self.memory))
    }
}

impl Archive for MemoryArchive {
    fn add_entry(&mut self, name: &str, data: &[u8]) -> Result<Entry, Box<dyn Error>> {
        if let Ok(entry) = self.find_entry(name) {
            self.remove_entry(&entry)?;
        }

        write_entry_to_archive(&mut Cursor::new(&mut self.memory), name, data)
    }

    fn read_entries(&mut self) -> Result<Vec<Entry>, Box<dyn Error>> {
        read_entries_from_archive(&mut Cursor::new(&mut self.memory))
    }

    fn read_entry_data(&mut self, entry: &Entry) -> Result<Vec<u8>, Box<dyn Error>> {
        read_entry_data(&mut Cursor::new(&mut self.memory), entry)
    }

    fn find_entry(&mut self, name: &str) -> Result<Entry, Box<dyn Error>> {
        find_entry_in_archive(&mut Cursor::new(&mut self.memory), name)
    }

    fn remove_entry(&mut self, entry: &Entry) -> Result<(), Box<dyn Error>> {
        let new_size = remove_entry_from_archive(&mut Cursor::new(&mut self.memory), entry)?;
        self.memory.resize(new_size as usize, 0);
        Ok(())
    }
}

fn read_all_bytes<Archive: Read + Seek>(archive: &mut Archive) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buffer = vec![];
    archive.seek(SeekFrom::Start(0))?;
    archive.read_to_end(&mut buffer)?;
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_able_to_add_entry_to_archive() -> Result<(), Box<dyn Error>> {
        let data = vec![1,2,3];

        let mut archive = MemoryArchive::from(vec![]);
        archive.add_entry("test", &data)?;

        // println!("{:?}", archive);

        Ok(())
    }

    #[test]
    fn should_be_able_to_read_entries_from_archive() -> Result<(), Box<dyn Error>> {
        let data = vec![1,2,3];

        let mut archive = MemoryArchive::from(vec![]);
        archive.add_entry("test", &data)?;

        let entries = archive.read_entries()?;

        assert!(entries.len() == 1);

        Ok(())
    }

    #[test]
    fn should_be_able_to_read_entry_data_from_archive() -> Result<(), Box<dyn Error>> {
        let data = vec![1,2,3];

        let mut archive = MemoryArchive::from(vec![]);
        let entry = archive.add_entry("test", &data)?;

        let entry = archive.read_entry_data(&entry)?;

        // println!("{:?} = {:?}", entry, data);

        assert!(entry == data);

        Ok(())
    }

    #[test]
    fn should_be_able_to_remove_entry_from_archive() -> Result<(), Box<dyn Error>> {
        let mut archive = MemoryArchive::from(vec![]);

        let entry = archive.add_entry("first", &vec![1,2,3])?;
        archive.add_entry("second", &vec![3,31,5,6,23,3])?;
        archive.add_entry("third", &vec![9,8,7])?;

        archive.remove_entry(&entry)?;

        Ok(())
    }
}