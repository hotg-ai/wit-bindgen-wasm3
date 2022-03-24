#[macro_use]
#[cfg(test)]
extern crate pretty_assertions;

use std::collections::BTreeMap;

use wit_bindgen_gen_core::{wit_parser::Interface, Direction, Files, Generator};
use wit_bindgen_gen_wasm3::Opts;

fn generate(filename: &str, src: &str, direction: Direction) -> String {
    let interface = Interface::parse(filename, src).unwrap();
    let mut files = Files::default();

    let mut imports = Vec::new();
    let mut exports = Vec::new();

    match direction {
        Direction::Import => imports.push(interface),
        Direction::Export => exports.push(interface),
    }

    Opts {
        rustfmt: true,
        ..Default::default()
    }
    .build()
    .generate_all(&imports, &exports, &mut files);

    let files: BTreeMap<_, _> = files.iter().collect();
    assert_eq!(files.len(), 1);
    std::str::from_utf8(&files["bindings.rs"])
        .unwrap()
        .to_string()
}

macro_rules! integration_tests {
    ($( $name:ident : $filename:literal ),* $(,)?) => {
        mod import {
            use super::*;

            $(
                #[test]
                fn $name() {
                    let src = include_str!($filename);
                    let $name = generate($filename, src, Direction::Import);

                    insta::assert_snapshot!($name);
                }
            )*
        }

        mod export {
            use super::*;

            $(
                #[test]
                fn $name() {
                    let src = include_str!($filename);
                    let $name = generate($filename, src, Direction::Export);

                    insta::assert_snapshot!($name);
                }
            )*
        }
    };
}

integration_tests! {
    single_function: "thunk.wit",
    empty: "empty.wit",
    record: "custom-record.wit",
}
