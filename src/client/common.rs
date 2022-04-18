use derive_new::new;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::Debug;

#[derive(new, Debug, Clone)]
pub struct WskProperties {
    pub auth_token: String,
    pub host: String,
    pub version: String,
    pub insecure: bool,
    pub namespace: String,
    pub verbose: bool,
    pub debug: bool,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Context {
    host: String,
    namespace: String,
    insecure: bool,
    username: String,
    password: String,
    version: String,
}

impl Context {
    pub fn new(wskprops: Option<&WskProperties>) -> Context {
        let api_key = if env::var("__OW_API_KEY").is_ok() {
            env::var("__OW_API_KEY").unwrap()
        } else {
            match wskprops {
                Some(wskprops) => wskprops.auth_token.clone(),
                None => "test:test".to_string(),
            }
        };
        let auth: Vec<&str> = api_key.split(":").collect();
        let host = if env::var("__OW_API_HOST").is_ok() {
            env::var("__OW_API_HOST").unwrap()
        } else {
            match wskprops {
                Some(wskprops) => wskprops.host.clone(),
                None => "host.docker.internal".to_string(),
            }
        };
        let namespace = if env::var("__OW_NAMESPACE").is_ok() {
            env::var("__OW_NAMESPACE").unwrap()
        } else {
            match wskprops {
                Some(wskprops) => wskprops.namespace.clone(),
                None => "guest".to_string(),
            }
        };

        let connectiontype = match wskprops {
            Some(config) => config.insecure.clone(),
            None => false,
        };

        let version = match wskprops {
            Some(config) => config.version.clone(),
            None => "v1".to_string(),
        };

        Context {
            host: host,
            namespace: namespace,
            insecure: connectiontype,
            username: auth[0].to_string(),
            password: auth[1].to_string(),
            version: version,
        }
    }

    pub fn namespace(&self) -> &String {
        &self.namespace
    }

    pub fn is_secure(&self) -> bool {
        self.insecure
    }

    pub fn auth(&self) -> (&String, &String) {
        (&self.username, &self.password)
    }

    pub fn host(&self) -> &String {
        &self.host
    }
}
