extern crate assert_cmd;
extern crate assert_fs;
extern crate predicates;

use std::error::Error;
use std::process::Command;
use std::fs::read_to_string;
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

fn test(i_fixture: &str, o_format: &str) -> Result<(), Box<Error>> {
    let tmp_dir = TempDir::new()?;
    let input_file = tmp_dir.child(i_fixture);
    let input_fixture = read_to_string(format!("./tests/assets/{}", i_fixture))?;
    input_file.write_str(&*input_fixture)?;
    let output_file = tmp_dir.child("output");

    let mut cmnd = get_cmd();
    cmnd.arg(input_file.path())
        .arg("-o")
        .arg(output_file.path())
        .arg("-f")
        .arg(o_format);
    cmnd.assert()
        .success();

    let output_fixture_file = format!("./tests/assets/{}.{}", i_fixture, o_format);
    output_file.assert(predicate::path::eq_file(output_fixture_file));

    tmp_dir.close()?;
    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<Error>> {
    test("test.json", "json")
}
