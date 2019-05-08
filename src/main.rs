mod args_parser;
mod transcoder;


fn main() {
    println!("{}\nVersion {}\n", env!("CARGO_PKG_DESCRIPTION"), env!("CARGO_PKG_VERSION"));
    let params_maybe = args_parser::parse();
    let params = match params_maybe {
        Ok(Some(p)) => p,
        Ok(None) => return,
        Err(e) => return println!("Error: {}", e),
    };

    transcoder::transcode(params);
}
