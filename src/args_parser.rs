extern crate getopts;

use getopts::Options;
use std::env;


#[derive(Debug)]
pub struct FileFormat {
    pub file: String,
    pub format: SupportedFormats,
}

#[derive(Debug)]
pub struct TranscodeParams {
    pub input: FileFormat,
    pub output: FileFormat,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum SupportedFormats {
    Json,
    Json5,
    Yaml,
    Toml,
    Cbor,
}

impl SupportedFormats {
    pub fn from_string(str: &String) -> Result<SupportedFormats, String> {
        match str.to_lowercase().as_ref() {
            "json" => Ok(SupportedFormats::Json),
            "json5" => Ok(SupportedFormats::Json5),
            "yaml" => Ok(SupportedFormats::Yaml),
            "toml" => Ok(SupportedFormats::Toml),
            "cbor" => Ok(SupportedFormats::Cbor),
            _ => Err("Unsupported format specified.".to_string()),
        }
    }
}

fn print_help(opts: Options) {
    let brief = format!("Usage: {} INPUT_FILE [options]", env!("CARGO_PKG_NAME"));
    print!("{}\n", opts.usage(&brief));
}

pub fn parse() -> Result<Option<TranscodeParams>, String> {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("i", "input", "set output format (json|json5|yaml|toml|cbor) [default: auto-detect from input file extension]", "INPUT_FORMAT");
    opts.optopt("f", "format", "set output format (json|yaml|toml|cbor) [default: json]", "OUTPUT_FORMAT");
    opts.optopt("o", "output", "set output file name", "OUTPUT_FILE");
    opts.optflag("h", "help", "print this help menu");

    let parsed_args = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => return Err(e.to_string()),
    };
    if parsed_args.opt_present("h") {
        print_help(opts);
        return Ok(None);
    }
    let input_file = if !parsed_args.free.is_empty() {
        parsed_args.free[0].clone()
    } else {
        print_help(opts);
        return Err("No input file specified.".to_string());
    };
    let input_format_str = match parsed_args.opt_str("i") {
        Some(f) => f.to_lowercase(),
        None => input_file.split('.').next_back().expect("Cannot detect input format, please specify it via -i").to_string(),
    };
    let input_format = match SupportedFormats::from_string(&input_format_str) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let output_format_str = match parsed_args.opt_str("f") {
        Some(f) => f.to_lowercase(),
        None => "json".to_string(),
    };
    let output_format = match SupportedFormats::from_string(&output_format_str) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let output_file = match parsed_args.opt_str("o") {
        Some(o) => o.to_lowercase(),
        None => format!("{}.{}", input_file, output_format_str),
    };

    Ok(Some(TranscodeParams{
        input: FileFormat{
            file: input_file,
            format: input_format,
        },
        output: FileFormat{
            file: output_file,
            format: output_format,
        },
    }))
}
