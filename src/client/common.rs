use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::Debug;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WhiskError {
    pub code: String,
    pub error: String,
}

/// Representation of OpenWhisk Properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WskProperties {
    /// Auth_token - `:` separated username and password
    pub auth_token: String,
    /// Api Host to interact with OpenWhisk API
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
    /// Debug - Toggle to enable it
    #[serde(default = "bool::default")]
    pub debug: bool,
}

fn default() -> String {
    "v1".to_string()
}

/// Context used to set Whisk properties
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Context {
    /// API Host Url
    host: String,
    /// Namespace where actions, triggers, rules exist
    namespace: String,
    /// Toggle to make secure and insecure connection (Set true or false)
    insecure: bool,
    /// Username for Authentication which is set by auth_token
    username: String,
    /// Password for Authentication which is set by auth_token
    password: String,
    /// Version
    version: String,
}

impl WskProperties {
    /// New Creates OpenWhisk properties
    ///
    /// # Arguments
    /// * `auth_token`  - The authorization token
    /// * `host`        - The API url
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
    /// "namespace".to_string()
    /// );
    ///
    /// ```
    pub fn new(auth_token: String, host: String, namespace: String) -> Self {
        Self {
            auth_token,
            host,
            insecure: false,
            namespace,
            version: default(),
            ..Default::default()
        }
    }

    /// To set Debug
    ///
    /// # Arguments
    /// * `debug`   - Bool to toggle debug
    ///
    /// # Example
    ///
    /// ```
    /// use openwhisk_rust::WskProperties;
    ///
    /// let wsk_property = WskProperties::new(
    /// "your:auth_token".to_string(),
    /// "host".to_string(),
    /// "namespace".to_string()
    /// ).set_debug(false);
    ///
    /// ```
    pub fn set_debug(mut self, debug: bool) -> Self {
        self.debug = debug;

        self
    }

    /// To set Verbose
    ///
    /// # Arguments
    /// * `verbose` - Bool to toggle verbose
    ///
    /// # Example
    ///
    /// ```
    /// use openwhisk_rust::WskProperties;
    ///
    /// let wsk_property = WskProperties::new(
    /// "your:auth_token".to_string(),
    /// "host".to_string(),
    /// "namespace".to_string()
    /// ).set_verbose(false);
    ///
    /// ```
    pub fn set_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;

        self
    }

    /// To set Version
    ///
    /// # Arguments
    /// * `version` - Version of Wsk properties
    ///
    /// # Example
    ///
    /// ```
    /// use openwhisk_rust::WskProperties;
    ///
    /// let wsk_property = WskProperties::new(
    /// "your:auth_token".to_string(),
    /// "host".to_string(),
    /// "namespace".to_string()
    /// ).set_version("v2".to_string());
    ///
    /// ```
    pub fn set_version(mut self, version: String) -> Self {
        self.version = version;

        self
    }

    /// To set client to bypass cerificate check primarily useful if you are using the OpenWhisk API over HTTPS and the API endpoint is using a self-signed or invalid certificate.
    ///
    /// # Arguments
    /// * `bypass`   - Bool to toggle bypass cerificate check
    ///
    /// # Example
    ///
    /// ```
    /// use openwhisk_rust::WskProperties;
    ///
    /// let new_wsk_property = WskProperties::new(
    /// "your:auth_token".to_string(),
    /// "host".to_string(),
    /// "namespace".to_string()
    /// ).set_bypass_cerificate_check(true);
    ///
    /// ```
    pub fn set_bypass_cerificate_check(mut self, bypass: bool) -> Self {
        self.insecure = bypass;

        self
    }
}

/// Trait OpenWhisk
pub trait OpenWhisk {
    type Output;
    /// Creates a new OpenWhisk client
    fn new_whisk_client(insecure: Option<bool>) -> Self::Output;
}

impl Context {
    /// Creates and returns context based on the Whisk Properties supplied
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
        let auth: Vec<&str> = api_key.split(':').collect();
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

        let connection_type = match wskprops {
            Some(config) => config.insecure,
            None => false,
        };

        let version = match wskprops {
            Some(config) => config.version.clone(),
            None => "v1".to_string(),
        };

        Context {
            host,
            namespace,
            insecure: connection_type,
            username: auth[0].to_string(),
            password: auth[1].to_string(),
            version,
        }
    }

    /// Returns namespace value
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

pub fn whisk_errors(code: StatusCode, message: String) -> String {
    format!(": Error -> [ Status :{}, Message : {} ]", code, message)
}
