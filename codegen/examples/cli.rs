use std::path::PathBuf;

use structopt::StructOpt;
use wit_bindgen_gen_core::{wit_parser::Interface, Files, Generator};
use wit_bindgen_gen_wasm3::Opts;

fn main() {
    let Args {
        imports,
        exports,
        out_dir,
    } = Args::from_args();

    let imports = imports
        .iter()
        .map(|path| Interface::parse_file(path))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let exports = exports
        .iter()
        .map(|path| Interface::parse_file(path))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut files = Files::default();
    let options = Opts {
        rustfmt: true,
        ..Opts::default()
    };

    options.build().generate_all(&imports, &exports, &mut files);

    for (filename, bindings) in files.iter() {
        let filename = out_dir.join(filename);
        if let Some(parent) = filename.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }

        std::fs::write(&filename, bindings).unwrap();
    }
}

#[derive(StructOpt)]
struct Args {
    #[structopt(short, long = "import")]
    imports: Vec<PathBuf>,
    #[structopt(short, long = "export")]
    exports: Vec<PathBuf>,
    #[structopt(short, long, default_value = ".")]
    out_dir: PathBuf,
}
