use derive_new::new;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::Debug;

use crate::api::{HttpMethods, Service};
use http::StatusCode;
use reqwest::blocking::Client;
use serde_json::Value;

/// Representation  of OpenWhisk Properties 
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WskProperties {
    /// Auth_token - `:` seperated username and passeord
    pub auth_token: String,
    /// Api Host to interact with Openwhisk API
    pub host: String,
    /// Version 
    #[serde(default = "default")]
    pub version: String,
    /// Toggle to set secure or insecure connection
    pub insecure: bool,
    /// Namespace for the endpoint
    pub namespace: String,
    /// Verbose - Toggle to enable it
    #[serde(default = "bool::default")]
    pub verbose: bool,
    /// Debug - Toggle to ennable it
    #[serde(default = "bool::default")]
    pub debug: bool,
}

fn default() -> String {
    "v1".to_string()
}

/// Context which used to set whisk properties
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Context {
    /// API Host Url
    host: String,
    /// Namespace where actions,triggrs, rules exists
    namespace: String,
    /// Toggle to make secure and inscure connection (Set true or false)
    insecure: bool,
    /// Username for Authentication which is set by auth_token
    username: String,
    /// Passwod for Authentication which is set by auth_token
    password: String,
    /// Verion
    version: String,
}


impl WskProperties {

    /// New Creates Openwhisk properties 
    /// 
    /// # Arguments 
    /// * `auth_token`  - The authorization token
    /// * `host`        - The API url
    /// * `insecure`    - Toggle for secure connection
    /// * `namespace`   - Name of the namespace
    /// 
    /// # Example
    /// 
    /// ```
    /// use openwhisk_rust::WskProperties;
    /// 
    /// let new_wsk_property = WskProperties::new(
    /// "your:auth_token".to_string(),
    /// "host".to_string(),
    /// true, 
    /// "namespace".to_string()
    /// );
    /// 
    /// ```
    pub fn new(auth_token: String, host: String, insecure: bool, namespace: String) -> Self {
        Self {
            auth_token,
            host,
            insecure,
            namespace,
            version: default(),
            ..Default::default()
        }
    }

    /// To set Verbose, Version and Debug 
    /// 
    /// # Arguments 
    /// * `debug`   - Bool to toggle debug
    /// * `verbose` - Bool to toggle verbose
    /// * `version` - Version of wsk properties
    /// 
    /// # Example
    /// 
    /// ```
    /// use openwhisk_rust::WskProperties;
    /// 
    /// let new_wsk_property = WskProperties::new(
    /// "your:auth_token".to_string(),
    /// "host".to_string(),
    /// true, 
    /// "namespace".to_string()
    /// );
    /// 
    /// new_wsk_property.set_verbose_debug_version(false,false,"v2".to_string());
    /// 
    /// ```
    pub fn set_verbose_debug_version(&self, debug: bool, verbose: bool, version: String) -> Self {
        Self {
            auth_token: self.auth_token.clone(),
            host: self.host.clone(),
            version,
            insecure: self.insecure.clone(),
            namespace: self.namespace.clone(),
            verbose,
            debug,
        }
    }
}


/// Trait Openwhisk
pub trait OpenWhisk {
    type Output;
    /// creates new openwhisk client
    fn new_whisk_client(insecure: Option<bool>) -> Self::Output;
}

impl Context {
    /// Creates and retruns context based on the Whisk Properties supplied
    /// 
    /// # Arguments
    /// * `wskprops` - Option of WhiskProperties
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
            host,
            namespace,
            insecure: connectiontype,
            username: auth[0].to_string(),
            password: auth[1].to_string(),
            version,
        }
    }

    /// Returns namspace value
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns secure value
    pub fn is_secure(&self) -> bool {
        self.insecure
    }

    /// Returns tuple containing username and password
    pub fn auth(&self) -> (&str, &str) {
        (&self.username, &self.password)
    }

    /// Returns host
    pub fn host(&self) -> &str {
        &self.host
    }
}

/// A Client to make Requests with.
#[derive(Debug, Default)]
pub struct NativeClient(Client);

impl OpenWhisk for NativeClient {
    /// NativeClient - Http Client (Here client is Reqwest Client)
    type Output = NativeClient;
    /// Creates New WhiskClient
    /// 
    /// # Arguments
    /// * `insecure` - Option of Bool to specify connection type
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

    ///
    /// Creates New Request and Returns  `reqwest::blocking::RequestBuilder`
    /// 
    /// # Arguments
    /// * `method`   - Enum of HTTPMethods
    /// * `url`      - API Host url
    /// * `use_auth` - Option of tuple conatining Username and Password
    /// * `body`     - Option of value which can have parameters necessary for the body of request
    /// 
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

    ///
    /// To invoke request and get response out of request execution
    /// 
    /// # Arguments
    /// 
    /// * `request` - Http request with url,auth and body
    /// 
    /// 
    /// 
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


