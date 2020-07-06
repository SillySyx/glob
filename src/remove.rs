use crate::{Glob};

use std::{
    error::Error,
};

pub trait RemoveGlob {
    fn remove(&self, name: &str) -> Result<(), Box<dyn Error>>;
}

impl RemoveGlob for Glob {
    fn remove(&self, _name: &str) -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }
}