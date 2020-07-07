use glob::{Glob, GlobEntry};
use std::error::Error;

#[test]
fn should_be_possible_to_find_entry_from_glob() -> Result<(), Box<dyn Error>> {
    let glob = Glob::from(&[0, 0, 0, 9, 116, 101, 115, 116, 45, 110, 97, 109, 101, 0, 0, 0, 0, 0, 0, 0, 3, 0, 1, 2])?;

    let entry = glob.find(|e| e.name == "test-name")?;

    assert_eq!("test-name", entry.name);

    Ok(())
}

#[test]
fn should_replace_entry_when_adding_with_same_name() -> Result<(), Box<dyn Error>> {
    let mut glob = Glob::new();

    glob.add(GlobEntry::new("test-name", None));
    glob.add(GlobEntry::new("test-name", Some(vec![0,1,2])));

    let entry = glob.find(|e| e.name == "test-name")?;

    assert_eq!(Some(vec![0,1,2]), entry.data);

    Ok(())
}

#[test]
#[should_panic]
fn should_be_possible_to_remove_entry_from_glob() {
    let mut glob = Glob::from(&[0, 0, 0, 9, 116, 101, 115, 116, 45, 110, 97, 109, 101, 0, 0, 0, 0, 0, 0, 0, 3, 0, 1, 2]).unwrap();

    glob.remove(|entry| entry.name == "test-name");

    glob.find(|e| e.name == "test-name").expect("should fail");
}