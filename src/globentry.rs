#[derive(Debug)]
pub struct GlobEntry {
    pub name: String,
    pub data: Option<Vec<u8>>,
}

impl GlobEntry {
    pub fn new(name: &str, data: Option<Vec<u8>>) -> Self {
        Self {
            name: name.to_owned(),
            data,
        }
    }
}