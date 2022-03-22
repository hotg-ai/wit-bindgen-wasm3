#[macro_use]
#[cfg(test)]
extern crate pretty_assertions;

use std::{collections::HashMap, path::Path};

use wit_bindgen_gen_core::{wit_parser::Interface, Files, Generator};
use wit_bindgen_gen_wasm3::Opts;

#[test]
// TODO: Re-enable this once our bindings are generated as expected so we can do
// snapshot testing while refactoring.
#[ignore]
fn single_function() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("single-function.wit");
    let single_function = Interface::parse_file(&path).unwrap();
    let mut files = Files::default();

    Opts {
        rustfmt: true,
        ..Default::default()
    }
    .build()
    .generate_all(&[single_function], &[], &mut files);

    let files: HashMap<_, _> = files.iter().collect();
    assert_eq!(files.len(), 1);
    let generated = std::str::from_utf8(files["bindings.rs"]).unwrap();
    assert_eq!(
        generated,
        include_str!("./data/single-function.generated.rs")
    );
}
