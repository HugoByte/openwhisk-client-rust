mod api;
mod client;
pub use api::{Action, Exec, HttpMethods, KeyValue, Rule, RuleResponse, Service, Trigger};
pub use client::{OpenWhisk, OpenwhiskClient, WasmClient, WskProperties};

#[cfg(not(target_arch = "wasm32"))]
pub use client::NativeClient;
