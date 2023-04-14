mod common;
mod openwhisk_client;

pub use common::*;
pub use openwhisk_client::OpenwhiskClient;

#[cfg(not(target_arch = "wasm32"))]
mod native_client;
#[cfg(not(target_arch = "wasm32"))]
pub use native_client::*;

#[cfg(target_arch = "wasm32")]
mod wasmtime_client;
#[cfg(target_arch = "wasm32")]
pub use wasmtime_client::WasmClient;
