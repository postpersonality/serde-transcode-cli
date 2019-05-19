extern crate assert_cmd;
extern crate assert_fs;
extern crate predicates;

use std::error::Error;
use std::process::Command;
use std::fs::{ copy };
use std::fmt;
use predicates::prelude::*;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use assert_fs::TempDir;


fn get_cmd() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

#[test]
fn file_not_specified() -> Result<(), Box<Error>> {
    let mut cmnd = get_cmd();
    cmnd.assert()
        .failure()
        .stderr(predicate::str::contains("No input file specified"));

    Ok(())
}


#[derive(Debug)]
struct AppError {
    stderr: Vec<u8>,
}

impl Error for AppError {
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.stderr))
    }
}

fn test(i_fixture: &str, o_format: &str) -> Result<(), Box<Error>> {
    let tmp_dir = TempDir::new()?;
    let input_file = tmp_dir.child(i_fixture);
    let output_file = tmp_dir.child("output");
    let input_fixture_path = format!("./tests/assets/{}", i_fixture);
    copy(input_fixture_path, tmp_dir.path().join(i_fixture))?;

    let mut cmnd = get_cmd();
    cmnd.arg(input_file.path())
        .arg("-o")
        .arg(output_file.path())
        .arg("-f")
        .arg(o_format);

    // Catch errors
    let ca = cmnd.assert();
    let result = ca.get_output();
    if !result.status.success() {
        return Err(Box::new(AppError{ stderr: result.stderr.clone() }));
    }

    let output_fixture_file = format!("./tests/assets/{}.{}", i_fixture, o_format);
    output_file.assert(predicate::path::eq_file(output_fixture_file));

    tmp_dir.close()?;
    Ok(())
}

#[test]
fn json_json() -> Result<(), Box<Error>> {
    test("test.json", "json")
}

#[test]
fn json_cbor() -> Result<(), Box<Error>> {
    test("test.json", "cbor")
}

#[test]
fn json_toml() -> Result<(), Box<Error>> {
    test("test.json", "toml")
}

#[test]
fn json_yaml() -> Result<(), Box<Error>> {
    test("test.json", "yaml")
}

#[test]
fn cbor_json() -> Result<(), Box<Error>> {
    test("test.cbor", "json")
}

#[test]
fn cbor_toml() -> Result<(), Box<Error>> {
    test("test.cbor", "toml")
}

#[test]
fn cbor_yaml() -> Result<(), Box<Error>> {
    test("test.cbor", "yaml")
}
