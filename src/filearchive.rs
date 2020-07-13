use std::fs::File;
use std::error::Error;

use crate::entry::Entry;
use crate::archive::{Archive, find_entry_in_archive, read_entries_from_archive, read_entry_data, remove_entry_from_archive, write_entry_to_archive};

pub struct FileArchive {
    file: File,
}

impl FileArchive {
    pub fn from(file: File) -> Self {
        Self {
            file,
        }
    }
}

impl Archive for FileArchive {
    fn add_entry(&mut self, name: &str, data: &[u8]) -> Result<Entry, Box<dyn Error>> {
        write_entry_to_archive(&mut self.file, name, data)
    }

    fn read_entries(&mut self) -> Result<Vec<Entry>, Box<dyn Error>> {
        read_entries_from_archive(&mut self.file)
    }

    fn read_entry_data(&mut self, entry: &Entry) -> Result<Vec<u8>, Box<dyn Error>> {
        read_entry_data(&mut self.file, entry)
    }

    fn find_entry(&mut self, name: &str) -> Result<Entry, Box<dyn Error>> {
        find_entry_in_archive(&mut self.file, name)
    }

    fn remove_entry(&mut self, entry: &Entry) -> Result<(), Box<dyn Error>> {
        remove_entry_from_archive(&mut self.file, entry)
    }
}