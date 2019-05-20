mod args_parser;
// mod transcoder;

#[cfg(test)]
mod args_parser_tests;

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}\nVersion {}\n", env!("CARGO_PKG_DESCRIPTION"), env!("CARGO_PKG_VERSION"));

    let params_maybe = args_parser::parse(env::args().collect());
    let params = match params_maybe {
        Ok(Some(p)) => p,
        Ok(None) => return Ok(()),
        Err(e) => return Err(e.into()),
    };

    panic!("{}", &params);
    // transcoder::transcode(params);

    // Ok(())
}
