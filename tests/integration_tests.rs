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
    stderr: String,
    input_file: String,
    output_file: String,
    output_format: String,
}

impl Error for AppError {
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.stderr)
    }
}

fn test(i_fixture: &str, o_format: &str) -> Result<(), Box<Error>> {
    let tmp_dir = TempDir::new().expect("Cannot create temp directory");
    let input_file = tmp_dir.child(i_fixture);
    let output_file = tmp_dir.child("output");
    std::fs::File::create(output_file.path())?;

    let o = Command::new("whoami").output()?;
    let c = String::from_utf8_lossy(o.stdout.as_ref());
    println!("whoami info: {}", c);
    let o = Command::new("ls").arg("-al").arg(tmp_dir.path().to_str().unwrap()).output()?;
    let c = String::from_utf8_lossy(o.stdout.as_ref());
    println!("path info: {}", c);
    let o = Command::new("ls").arg("-al").arg(output_file.path().to_str().unwrap()).output()?;
    let c = String::from_utf8_lossy(o.stdout.as_ref());
    println!("file info: {}", c);

    let input_fixture_path = format!("{}/tests/assets/{}", env!("CARGO_MANIFEST_DIR"), i_fixture);
    copy(input_fixture_path, tmp_dir.path().join(i_fixture)).expect("Cannot copy fixture file");

    let mut cmnd = get_cmd();
    cmnd.arg(input_file.path())
        .arg("-o")
        .arg(output_file.path())
        .arg("-f")
        .arg(o_format);

    let ca = cmnd.assert();

    // Catch stderr
    let result = ca.get_output();
    if !result.status.success() {
        return Err(Box::new(AppError{
            stderr: String::from_utf8_lossy(&result.stderr.clone()).to_string(),
            input_file: String::from(input_file.path().to_str().unwrap()),
            output_file: String::from(output_file.path().to_str().unwrap()),
            output_format: o_format.to_string(),
        }));
    }

    let output_fixture_file = format!("{}/tests/assets/{}.{}", env!("CARGO_MANIFEST_DIR"), i_fixture, o_format);
    output_file.assert(predicate::path::eq_file(output_fixture_file));

    tmp_dir.close().expect("Cannot close temp directory");
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

#[test]
fn toml_cbor() -> Result<(), Box<Error>> {
    test("test.toml", "cbor")
}

#[test]
fn toml_json() -> Result<(), Box<Error>> {
    test("test.toml", "json")
}

#[test]
fn toml_yaml() -> Result<(), Box<Error>> {
    test("test.toml", "yaml")
}
