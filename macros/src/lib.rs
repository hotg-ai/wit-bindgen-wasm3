use std::{fmt::Display, path::Path};

use proc_macro::{Span, TokenStream};
use syn::{Error, LitStr};
use wit_bindgen_gen_core::{wit_parser::Interface, Direction, Files, Generator};
use wit_bindgen_gen_wasm3::Wasm3;

/// Use functions defined by a WebAssembly module.
///
/// # Examples
///
/// Given the following WIT file:
///
/// ```text
#[doc = include_str!("../../wit-files/rune/rune-v1.wit")]
/// ```
///
/// You would load the WebAssembly module and call `start()` like this:
///
/// ```rust,no_run
/// use wasm3::Environment;
///
/// wit_bindgen_wasm3_macros::import!("../wit-files/rune/rune-v1.wit");
///
/// use guest::{Guest, Metadata}; // Bring the generated types into scope
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let env = Environment::new()?;
///
/// let stack_slots = 1024 * 60;
/// let runtime = env.create_runtime(stack_slots)?;
///
/// let wasm = std::fs::read("./my.wasm")?;
/// let guest_functions = Guest::instantiate(&runtime, wasm)?;
///
/// let metadata: Metadata = guest_functions.metadata()?;
/// println!("Loaded {} v{}", metadata.name, metadata.version);
/// # Ok(())
/// # }
/// ```
#[proc_macro]
pub fn import(input: TokenStream) -> TokenStream {
    let path = syn::parse_macro_input!(input as syn::LitStr);
    run(&path, Direction::Import).unwrap_or_else(|e| e.to_compile_error().into())
}

/// Expose host functions to a WebAssembly module.
///
/// # Examples
///
/// Given the following WIT file:
///
/// ```text
#[doc = include_str!("../../wit-files/rune/runtime-v1.wit")]
/// ```
///
/// You would load the WebAssembly module and call `start()` like this:
///
/// ```rust
/// use wasm3::{Environment, Module, Runtime};
///
/// wit_bindgen_wasm3_macros::export!("../wit-files/rune/runtime-v1.wit");
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let env = Environment::new()?;
/// let stack_slots = 1024 * 60;
/// let runtime = env.create_runtime(stack_slots)?;
///
/// let wasm = std::fs::read("./my.wasm")?;
/// let module = Module::parse(&env, wasm)?;
///
/// # Ok(())
/// # }
/// ```
#[proc_macro]
pub fn export(input: TokenStream) -> TokenStream {
    let path = syn::parse_macro_input!(input as syn::LitStr);
    run(&path, Direction::Export).unwrap_or_else(|e| e.to_compile_error().into())
}

fn run(input: &LitStr, direction: Direction) -> Result<TokenStream, Error> {
    // Note: to be deterministic, resolve all files relative to the package
    // root
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("Unable to read $CARGO_MANIFEST_DIR");
    let path = Path::new(&manifest_dir).join(input.value());

    let interface = Interface::parse_file(&path)
        .with_context(|| format!("Unable to parse \"{}\"", path.display()))?;

    let mut files = Files::default();

    Wasm3::default().generate_one(&interface, direction, &mut files);

    let (_filename, contents) = files
        .iter()
        .next()
        .with_context(|| "No bindings were generated")?;
    let contents = std::str::from_utf8(contents)
        .with_context(|| "Unable to parse the generated bindings as UTF-8")?;

    contents
        .parse()
        .with_context(|| "Unable to parse the generated bindings as Rust")
}

trait IntoResult<T> {
    fn with_context<F, C>(self, context_func: F) -> Result<T, Error>
    where
        F: FnOnce() -> C,
        C: Display;
}

impl<T, E: Display> IntoResult<T> for Result<T, E> {
    fn with_context<F, C>(self, context_func: F) -> Result<T, Error>
    where
        F: FnOnce() -> C,
        C: Display,
    {
        self.map_err(|e| {
            Error::new(
                Span::call_site().into(),
                format!("{}: {}", context_func(), e),
            )
        })
    }
}

impl<T> IntoResult<T> for Option<T> {
    fn with_context<F, C>(self, context_func: F) -> Result<T, Error>
    where
        F: FnOnce() -> C,
        C: Display,
    {
        self.ok_or_else(|| Error::new(Span::call_site().into(), format!("{}", context_func())))
    }
}
