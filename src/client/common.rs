use derive_new::new;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::Debug;

use crate::api::{HttpMethods, Service};
use http::StatusCode;
use reqwest::blocking::Client;
use serde_json::Value;

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

pub trait OpenWhisk {
    type Output;
    fn new_whisk_client(insecure: Option<bool>) -> Self::Output;
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

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn is_secure(&self) -> bool {
        self.insecure
    }

    pub fn auth(&self) -> (&str, &str) {
        (&self.username, &self.password)
    }

    pub fn host(&self) -> &str {
        &self.host
    }
}
pub struct NativeClient(Client);

impl OpenWhisk for NativeClient {
    type Output = NativeClient;
    fn new_whisk_client(insecure: Option<bool>) -> Self::Output {
        match insecure {
            Some(x) => match x {
                true => NativeClient(
                    reqwest::blocking::Client::builder()
                        .danger_accept_invalid_certs(x)
                        .timeout(None)
                        .build()
                        .unwrap(),
                ),
                false => NativeClient(
                    reqwest::blocking::Client::builder()
                        .timeout(None)
                        .build()
                        .unwrap(),
                ),
            },
            None => todo!(),
        }
    }
}

impl Service for NativeClient {
    type Output = reqwest::blocking::RequestBuilder;

    fn new_request(
        &self,
        method: HttpMethods,
        url: &str,
        use_auth: Option<(&str, &str)>,
        body: Option<Value>,
    ) -> Result<Self::Output, String> {
        let body = body.unwrap_or(serde_json::json!({}));

        match use_auth {
            Some(auth) => {
                let user = auth.0;
                let pass = auth.1;

                match method {
                    HttpMethods::GET => return Ok(self.0.get(url).basic_auth(user, Some(pass))),
                    HttpMethods::POST => {
                        return Ok(self.0.post(url).basic_auth(user, Some(pass)).json(&body))
                    }
                    HttpMethods::PUT => {
                        return Ok(self.0.put(url).basic_auth(user, Some(pass)).json(&body))
                    }
                    HttpMethods::DELETE => {
                        return Ok(self.0.delete(url).basic_auth(user, Some(pass)).json(&body))
                    }
                    _ => Err(format!("Falied to create request")),
                }
            }
            None => match method {
                HttpMethods::GET => return Ok(self.0.get(url)),
                HttpMethods::POST => return Ok(self.0.post(url).json(&body)),
                HttpMethods::PUT => return Ok(self.0.put(url).json(&body)),
                HttpMethods::DELETE => return Ok(self.0.delete(url).json(&body)),
                _ => Err(format!("Falied to create request")),
            },
        }
    }

    fn invoke_request(&self, request: Self::Output) -> Result<Value, String> {
        if let Ok(response) = request.send() {
            return match response.status() {
                StatusCode::OK => Ok(response.json().unwrap()),
                _ => Err(format!("failed to invoke request {}", response.status())),
            };
        };
        Err(format!("failed to invoke request"))
    }
}

impl Clone for NativeClient {
    fn clone(&self) -> Self {
        NativeClient(self.0.clone())
    }

    fn clone_from(&mut self, _source: &Self) {
        NativeClient(self.0.clone());
    }
}
