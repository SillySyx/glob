pub struct GlobEntry {
    pub start_index: u64,
    pub name_length: u32,
    pub data_length: u64,
}

impl GlobEntry {
    pub fn new() -> Self {
        Self {
            start_index: 0,
            name_length: 0,
            data_length: 0,
        }
    }
}