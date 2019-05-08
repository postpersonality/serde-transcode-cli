extern crate serde;
extern crate serde_transcode;
extern crate serde_json;
extern crate serde_cbor;
extern crate serde_yaml;
extern crate toml;
extern crate json5;

use super::args_parser::{ TranscodeParams, SupportedFormats::* };
use std::{ fs, fs::File };
use std::io::{ Write, BufReader, BufWriter };

pub fn transcode(params: TranscodeParams) {
    match (params.input.format, params.output.format) {
        (Json, Json) => {
            let reader = BufReader::new(File::open(params.input.file).expect("Unable to read file"));
            let writer = BufWriter::new(File::create(params.output.file).expect("Unable to write file"));

            let mut deserializer = serde_json::Deserializer::from_reader(reader);
            let mut serializer = serde_json::Serializer::new(writer);
            serde_transcode::transcode(&mut deserializer, &mut serializer).expect("Unable to transcode");

            serializer.into_inner().flush().unwrap();
        },
        (Json, Cbor) => {
            let reader = BufReader::new(File::open(params.input.file).expect("Unable to read file"));
            let writer = BufWriter::new(File::create(params.output.file).expect("Unable to write file"));

            let mut deserializer = serde_json::Deserializer::from_reader(reader);
            let mut serializer = serde_cbor::Serializer::new(writer);
            serde_transcode::transcode(&mut deserializer, &mut serializer).expect("Unable to transcode");

            serializer.into_inner().flush().unwrap();
        }
        (Cbor, Json) => {
            let reader = BufReader::new(File::open(params.input.file).expect("Unable to read file"));
            let writer = BufWriter::new(File::create(params.output.file).expect("Unable to write file"));

            let mut deserializer = serde_cbor::Deserializer::from_reader(reader);
            let mut serializer = serde_json::Serializer::new(writer);
            serde_transcode::transcode(&mut deserializer, &mut serializer).expect("Unable to transcode");

            serializer.into_inner().flush().unwrap();
        }
        (Json, Toml) => {
            let reader = BufReader::new(File::open(params.input.file).expect("Unable to read file"));
            let mut output_contents = String::new();

            let mut deserializer = serde_json::Deserializer::from_reader(reader);
            let mut serializer = toml::Serializer::new(&mut output_contents);
            serde_transcode::transcode(&mut deserializer, &mut serializer).expect("Unable to transcode");

            fs::write(params.output.file, output_contents).expect("Unable to write file");
        }
        (Toml, Json) => {
            let input_contents = fs::read_to_string(params.input.file).expect("Unable to read file");
            let writer = BufWriter::new(File::create(params.output.file).expect("Unable to write file"));

            let mut deserializer = toml::Deserializer::new(&input_contents);
            let mut serializer = serde_json::Serializer::new(writer);
            serde_transcode::transcode(&mut deserializer, &mut serializer).expect("Unable to transcode");

            serializer.into_inner().flush().unwrap();
        }
        (Cbor, Toml) => {
            let reader = BufReader::new(File::open(params.input.file).expect("Unable to read file"));
            let mut output_contents = String::new();

            let mut deserializer = serde_cbor::Deserializer::from_reader(reader);
            let mut serializer = toml::Serializer::new(&mut output_contents);
            serde_transcode::transcode(&mut deserializer, &mut serializer).expect("Unable to transcode");

            fs::write(params.output.file, output_contents).expect("Unable to write file");
        }
        (Toml, Cbor) => {
            let input_contents = fs::read_to_string(params.input.file).expect("Unable to read file");
            let writer = BufWriter::new(File::create(params.output.file).expect("Unable to write file"));

            let mut deserializer = toml::Deserializer::new(&input_contents);
            let mut serializer = serde_cbor::Serializer::new(writer);
            serde_transcode::transcode(&mut deserializer, &mut serializer).expect("Unable to transcode");

            serializer.into_inner().flush().unwrap();
        }
        (Json, Yaml) => {
            let reader = BufReader::new(File::open(params.input.file).expect("Unable to read file"));
            let writer = BufWriter::new(File::create(params.output.file).expect("Unable to write file"));

            let mut deserializer = serde_json::Deserializer::from_reader(reader);
            serde_yaml::to_writer(
                writer,
                &serde_transcode::Transcoder::new(&mut deserializer)).expect("Unable to transcode");
        }
        (Cbor, Yaml) => {
            let reader = BufReader::new(File::open(params.input.file).expect("Unable to read file"));
            let writer = BufWriter::new(File::create(params.output.file).expect("Unable to write file"));

            let mut deserializer = serde_cbor::Deserializer::from_reader(reader);
            serde_yaml::to_writer(
                writer,
                &serde_transcode::Transcoder::new(&mut deserializer)).expect("Unable to transcode");
        }
        (Toml, Yaml) => {
            let input_contents = fs::read_to_string(params.input.file).expect("Unable to read file");
            let writer = BufWriter::new(File::create(params.output.file).expect("Unable to write file"));

            let mut deserializer = toml::Deserializer::new(&input_contents);
            serde_yaml::to_writer(
                writer,
                &serde_transcode::Transcoder::new(&mut deserializer)).expect("Unable to transcode");
        }
        (_, _) => panic!("Not implemented"),
    }
}
