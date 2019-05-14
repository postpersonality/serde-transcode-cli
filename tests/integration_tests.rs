extern crate assert_cmd;
extern crate predicates;

use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn file_doesnt_exist() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::new("./target/debug/serde-transcode-cli");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No input file specified"));

    Ok(())
}

// use tempfile::NamedTempFile;
// use std::io::{self, Write};

// #[test]
// fn find_content_in_file() -> Result<(), Box<std::error::Error>> {
//     let mut file = NamedTempFile::new()?;
//     writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

//     let mut cmd = Command::main_binary()?;
//     cmd.arg("test")
//         .arg(file.path());
//     cmd.assert()
//         .success()
//         .stdout(predicate::str::contains("test\nAnother test"));

//     Ok(())
// }
