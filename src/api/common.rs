use serde::{Deserialize, Serialize};

pub static ACTION_ENDPOINT: &str = "actions";
pub static TRIGGERS_ENDPOINT: &str = "triggers";
pub static RULES_ENDPOINT: &str = "rules";
pub static NAMESPACE: &str = "namespaces";

#[derive(Debug, Deserialize, Serialize, Clone,Default,PartialEq)]
pub struct Limits {
    #[serde(default)]
    pub timeout: i64,
    #[serde(default)]
    pub memory: i64,
    #[serde(default)]
    pub logsize: i64,
    #[serde(default)]
    pub concurrency: i64,
}

pub enum HttpMethods {
    GET,
    PUT,
    POST,
    DELETE,
}