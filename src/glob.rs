pub struct Glob {
    pub file_path: String,
}

impl Glob {
    pub fn new(file_path: &str) -> Glob {
        Glob {
            file_path: file_path.to_owned(),
        }
    }
}
