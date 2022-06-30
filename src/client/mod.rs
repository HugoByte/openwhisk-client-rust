mod common;
mod openwhisk_client;

mod wasmtime_client;
pub use common::*;
pub use openwhisk_client::OpenwhiskClient;

#[cfg(not(target_arch = "wasm32"))]
mod native_client;
#[cfg(not(target_arch = "wasm32"))]
pub use native_client::*;

pub use wasmtime_client::WasmClient;
