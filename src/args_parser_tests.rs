use super::args_parser::{ *, SupportedFormats::* };

fn parse_args (args: &str) -> Result<Option<TranscodeParams>, String> {
    parse(format!("bin {}", args).split_whitespace().map(String::from).collect())
}


#[test]
fn parse_errors() {
    assert_eq!(
        parse_args(""),
        Err(ERR_NO_FILE.to_string()));
    assert_eq!(
        parse_args("file"),
        Err(ERR_CANNOT_DETECT.to_string()));
    assert_eq!(
        parse_args("file.unknown"),
        Err(ERR_UNSUPP_FORMAT.to_string()));
    assert_eq!(
        parse_args("file.json -f unknown"),
        Err(ERR_UNSUPP_FORMAT.to_string()));
    assert_eq!(
        parse_args("file.json -f json -i unknown"),
        Err(ERR_UNSUPP_FORMAT.to_string()));
}

#[test]
fn parse_help() {
    assert_eq!(
        parse_args("--help"),
        Ok(None));
    assert_eq!(
        parse_args("-h"),
        Ok(None));
}

#[test]
fn parse_params() {
    assert_eq!(
        parse_args("file.json -f yaml"),
        Ok(Some(TranscodeParams{
            input: FileFormat{
                file: "file.json".to_string(),
                format: Json,
            },
            output: FileFormat{
                file: "file.json.yaml".to_string(),
                format: Yaml,
            },
        }))
    );

    assert_eq!(
        parse_args("file.json -f yaml -i toml"),
        Ok(Some(TranscodeParams{
            input: FileFormat{
                file: "file.json".to_string(),
                format: Toml,
            },
            output: FileFormat{
                file: "file.json.yaml".to_string(),
                format: Yaml,
            },
        }))
    );

    assert_eq!(
        parse_args("file.json -f yaml -o output.yaml"),
        Ok(Some(TranscodeParams{
            input: FileFormat{
                file: "file.json".to_string(),
                format: Json,
            },
            output: FileFormat{
                file: "output.yaml".to_string(),
                format: Yaml,
            },
        }))
    );

    assert_eq!(
        parse_args("file.json -f yaml -i toml -o output.yaml"),
        Ok(Some(TranscodeParams{
            input: FileFormat{
                file: "file.json".to_string(),
                format: Toml,
            },
            output: FileFormat{
                file: "output.yaml".to_string(),
                format: Yaml,
            },
        }))
    );

    assert_eq!(
        parse_args("CaseTest.json -f yaml -i toml -o CaseTest.yaml"),
        Ok(Some(TranscodeParams{
            input: FileFormat{
                file: "CaseTest.json".to_string(),
                format: Toml,
            },
            output: FileFormat{
                file: "CaseTest.yaml".to_string(),
                format: Yaml,
            },
        }))
    );
}
