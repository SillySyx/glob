A lightweight binary archive used to read and write data.


## Features
* Read and write data entries to either memory or files
* Only stores the data entry name and content, no unnecessary metadata or schemas is saved
* Reads large files fast


## Installation
This library has not been published to cargo as a package, to use it you can add it as a submodule for your project.

Add it as a submodule
```
git submodule add https://github.com/SillySyx/glob.git
```

Add it to your dependencies
```
glob = { path = "./glob" }
```


## Examples

```
use std::error::Error;
use glob::MemoryArchive;

fn main() -> Result<(), Box<dyn Error>>{
    let text = String::from("hello world"); 
    let data = text.as_bytes();

    let mut archive = MemoryArchive::from(vec![]);
    archive.add_entry("entry name", &data)?;

    let entry = archive.find_entry("entry name")?;
    let data = archive.read_entry_data(&entry)?;

    let text = String::from_utf8(data)?;

    Ok(())
}
```