use std::fs;
use std::path::Path;

use protoc_rust::Customize;

const PROTO_OUTPUT_DIR: &str = "core/protos/src/autogenerated";

pub fn autogenerate() {
    // dumb vector hack because https://bit.ly/2RJcIH1
    let input_files: Vec<String> = fs::read_dir(Path::new("protos/protos"))
        .expect("could not read protos directory")
        .map(|dir_entry| {
            dir_entry.expect("unable to get entry")
                .path()
                .display()
                .to_string()
        })
        .collect();
    let input_files: Vec<&str> = input_files.iter()
        .map(|x| x.as_ref())
        .collect();
    protoc_rust::run(protoc_rust::Args {
        out_dir: PROTO_OUTPUT_DIR,
        input: input_files.as_slice(),
        includes: &["protos"],
        customize: Customize {
            expose_oneof: Some(true),
            ..Default::default()
        },
    }).expect("protoc");
}
