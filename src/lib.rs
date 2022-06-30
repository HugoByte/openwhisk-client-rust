mod api;
mod client;
pub use api::{Action, Exec, HttpMethods, KeyValue, Rule, Service, Trigger};
pub use client::{ OpenWhisk, OpenwhiskClient, WskProperties,WasmClient};

#[cfg(not(target_arch = "wasm32"))]
pub use client::NativeClient;
