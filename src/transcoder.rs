extern crate serde;
extern crate serde_transcode;
extern crate serde_json;
extern crate serde_cbor;
extern crate serde_yaml;
extern crate toml;

use super::args_parser::{ TranscodeParams, SupportedFormats::* };
use std::fs::{ self, File };
use std::io::{ Write, BufReader, BufWriter };
use std::process::Command;

const ERR_READ_FILE: &str = "Unable to read file";
const ERR_WRITE_FILE: &str = "Unable to write file";
const ERR_TRANSCODE: &str = "Unable to transcode";

pub fn transcode(params: TranscodeParams) {

    let p = std::path::Path::new(&params.output.file).parent().unwrap();
    println!("path: {}", p.display());

    let o = Command::new("ls").arg("-al").arg(p.to_str().unwrap()).output().expect("path");
    let c = String::from_utf8_lossy(o.stdout.as_ref());
    println!("path info: {}", c);
    let c = String::from_utf8_lossy(o.stderr.as_ref());
    println!("path err: {}", c);
    let o = Command::new("ls").arg("-al").arg(&params.input.file).output().expect("ifile");
    let c = String::from_utf8_lossy(o.stdout.as_ref());
    println!("ifile info: {}", c);
    let o = Command::new("ls").arg("-al").arg(&params.output.file).output().expect("ofile");
    let c = String::from_utf8_lossy(o.stdout.as_ref());
    println!("ofile info: {}", c);
    let c = String::from_utf8_lossy(o.stderr.as_ref());
    println!("ofile err: {}", c);

    println!("i info: {}", &params.input.file);
    println!("o info: {}", &params.output.file);


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
