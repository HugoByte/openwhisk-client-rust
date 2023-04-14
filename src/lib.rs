mod api;
mod client;
pub use api::{
    Action, ActionList, Exec, HttpMethods, KeyValue, Limits, Rule, RuleResponse, Service, Trigger,
};
pub use client::{OpenWhisk, OpenwhiskClient, WskProperties};

#[cfg(not(target_arch = "wasm32"))]
pub use client::NativeClient;

#[cfg(target_arch = "wasm32")]
pub use client::WasmClient;