extern crate serde;
extern crate serde_transcode;
extern crate serde_json;
extern crate serde_cbor;
extern crate serde_yaml;
extern crate toml;

use super::args_parser::{ TranscodeParams, SupportedFormats::* };
use std::fs::{ self, File };
use std::io::{ Write, BufReader, BufWriter };

const ERR_READ_FILE: &str = "Unable to read file";
const ERR_WRITE_FILE: &str = "Unable to write file";
const ERR_TRANSCODE: &str = "Unable to transcode";

pub fn transcode(params: TranscodeParams) {
    println!("{}", &params);
    match (params.input.format, params.output.format) {
        (Json, Json) => {
            let reader = BufReader::new(File::open(params.input.file).expect(ERR_READ_FILE));
            let writer = BufWriter::new(File::create(params.output.file).expect(ERR_WRITE_FILE));

            let mut deserializer = serde_json::Deserializer::from_reader(reader);
            let mut serializer = serde_json::Serializer::pretty(writer);
            serde_transcode::transcode(&mut deserializer, &mut serializer).expect(ERR_TRANSCODE);

            serializer.into_inner().flush().unwrap();
        },
        (Json, Cbor) => {
            let reader = BufReader::new(File::open(params.input.file).expect(ERR_READ_FILE));
            let writer = BufWriter::new(File::create(params.output.file).expect(ERR_WRITE_FILE));

            let mut deserializer = serde_json::Deserializer::from_reader(reader);
            let mut serializer = serde_cbor::Serializer::new(writer);
            serde_transcode::transcode(&mut deserializer, &mut serializer).expect(ERR_TRANSCODE);

            serializer.into_inner().flush().unwrap();
        }
        (Cbor, Json) => {
            let reader = BufReader::new(File::open(params.input.file).expect(ERR_READ_FILE));
            let writer = BufWriter::new(File::create(params.output.file).expect(ERR_WRITE_FILE));

            let mut deserializer = serde_cbor::Deserializer::from_reader(reader);
            let mut serializer = serde_json::Serializer::new(writer);
            serde_transcode::transcode(&mut deserializer, &mut serializer).expect(ERR_TRANSCODE);

            serializer.into_inner().flush().unwrap();
        }
        (Json, Toml) => {
            let reader = BufReader::new(File::open(params.input.file).expect(ERR_READ_FILE));
            let mut output_contents = String::new();

            let mut deserializer = serde_json::Deserializer::from_reader(reader);
            let mut serializer = toml::Serializer::new(&mut output_contents);
            serde_transcode::transcode(&mut deserializer, &mut serializer).expect(ERR_TRANSCODE);

            fs::write(params.output.file, output_contents).expect(ERR_WRITE_FILE);
        }
        (Toml, Json) => {
            let input_contents = fs::read_to_string(params.input.file).expect(ERR_READ_FILE);
            let writer = BufWriter::new(File::create(params.output.file).expect(ERR_WRITE_FILE));

            let mut deserializer = toml::Deserializer::new(&input_contents);
            let mut serializer = serde_json::Serializer::new(writer);
            serde_transcode::transcode(&mut deserializer, &mut serializer).expect(ERR_TRANSCODE);

            serializer.into_inner().flush().unwrap();
        }
        (Cbor, Toml) => {
            let reader = BufReader::new(File::open(params.input.file).expect(ERR_READ_FILE));
            let mut output_contents = String::new();

            let mut deserializer = serde_cbor::Deserializer::from_reader(reader);
            let mut serializer = toml::Serializer::new(&mut output_contents);
            serde_transcode::transcode(&mut deserializer, &mut serializer).expect(ERR_TRANSCODE);

            fs::write(params.output.file, output_contents).expect(ERR_WRITE_FILE);
        }
        (Toml, Cbor) => {
            let input_contents = fs::read_to_string(params.input.file).expect(ERR_READ_FILE);
            let writer = BufWriter::new(File::create(params.output.file).expect(ERR_WRITE_FILE));

            let mut deserializer = toml::Deserializer::new(&input_contents);
            let mut serializer = serde_cbor::Serializer::new(writer);
            serde_transcode::transcode(&mut deserializer, &mut serializer).expect(ERR_TRANSCODE);

            serializer.into_inner().flush().unwrap();
        }
        (Json, Yaml) => {
            let reader = BufReader::new(File::open(params.input.file).expect(ERR_READ_FILE));
            let writer = BufWriter::new(File::create(params.output.file).expect(ERR_WRITE_FILE));

            let mut deserializer = serde_json::Deserializer::from_reader(reader);
            serde_yaml::to_writer(
                writer,
                &serde_transcode::Transcoder::new(&mut deserializer)).expect(ERR_TRANSCODE);
        }
        (Cbor, Yaml) => {
            let reader = BufReader::new(File::open(params.input.file).expect(ERR_READ_FILE));
            let writer = BufWriter::new(File::create(params.output.file).expect(ERR_WRITE_FILE));

            let mut deserializer = serde_cbor::Deserializer::from_reader(reader);
            serde_yaml::to_writer(
                writer,
                &serde_transcode::Transcoder::new(&mut deserializer)).expect(ERR_TRANSCODE);
        }
        (Toml, Yaml) => {
            let input_contents = fs::read_to_string(params.input.file).expect(ERR_READ_FILE);
            let writer = BufWriter::new(File::create(params.output.file).expect(ERR_WRITE_FILE));

            let mut deserializer = toml::Deserializer::new(&input_contents);
            serde_yaml::to_writer(
                writer,
                &serde_transcode::Transcoder::new(&mut deserializer)).expect(ERR_TRANSCODE);
        }
        (_, _) => panic!("Not implemented"),
    }
}
