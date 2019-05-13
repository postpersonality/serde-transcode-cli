# serde-transcode-cli &emsp; [![Build Status]][travis]

[Build Status]: https://api.travis-ci.org/serde-rs/serde.svg?branch=master
[travis]: https://travis-ci.org/serde-rs/serde

File format conversion CLI utility

```
Utility for file format conversion (via rust serde)
Version 0.1.0

Usage: serde-transcode-cli INPUT_FILE [options]

Options:
    -i, --input INPUT_FORMAT
                        set output format (json|toml|cbor) [default:
                        auto-detect from input file extension]
    -f, --format OUTPUT_FORMAT
                        set output format (json|yaml|toml|cbor) [default:
                        json]
    -o, --output OUTPUT_FILE
                        set output file name
    -h, --help          print this help menu
```
