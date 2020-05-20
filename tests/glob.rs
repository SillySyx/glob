use serde::{Deserialize, Serialize};

use glob::*;

#[derive(Deserialize, Serialize)]
struct TestStruct {
    value: String,
}

#[test]
fn should_be_possible_to_read_from_blob_as_bytes() -> Result<(), Box<dyn std::error::Error>> {
    let glob = Glob::new("tests/bytes_data");
    let data = glob.read("resource/name")?;

    let expected_data = vec![1,2,3];

    assert!(data == expected_data);

    Ok(())
}

#[test]
fn should_be_possible_to_read_from_blob_as_struct() -> Result<(), Box<dyn std::error::Error>> {
    let glob = Glob::new("tests/struct_data");
    let data: TestStruct = glob.read_as("resource/name")?;

    let expected_struct = TestStruct {
        value: String::from("test"),
    };

    assert!(data.value == expected_struct.value);

    Ok(())
}

#[test]
fn should_be_possible_to_write_bytes_to_blob() -> Result<(), Box<dyn std::error::Error>> {
    remove_file("tests/bytes_data");

    let glob = Glob::new("tests/bytes_data");
    let data = vec![1,2,3];

    glob.write("resource/name", data)?;

    Ok(())
}

#[test]
fn should_be_possible_to_write_struct_to_blob() -> Result<(), Box<dyn std::error::Error>> {
    remove_file("tests/struct_data");

    let glob = Glob::new("tests/struct_data");
    let data = TestStruct {
        value: String::from("test"),
    };

    glob.write_as("resource/name", data)?;

    Ok(())
}

fn remove_file(path: &'static str) {
    match std::fs::remove_file(path) {
        Ok(_) => {},
        Err(_) => {},
    };
}