#[macro_use]
#[cfg(test)]
extern crate pretty_assertions;

use std::{collections::HashMap, path::Path};

use wit_bindgen_gen_core::{wit_parser::Interface, Files, Generator};
use wit_bindgen_gen_wasm3::Opts;

#[test]
fn single_function() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("single-function.wit");
    let single_function = Interface::parse_file(&path).unwrap();
    let mut files = Files::default();

    Opts::default()
        .build()
        .generate_all(&[single_function], &[], &mut files);

    let files: HashMap<_, _> = files.iter().collect();
    assert_eq!(files.len(), 1);
    let generated = std::str::from_utf8(files["bindings.rs"]).unwrap();
    println!("=========\n{}=======", generated);
    assert_eq!(generated, "asd");
}
