#[cfg(features = "macros")]
pub use wit_bindgen_wasm3_macros::{export, import};

#[cfg(features = "macros")]
pub mod _internal {
    pub use ::anyhow;
}
