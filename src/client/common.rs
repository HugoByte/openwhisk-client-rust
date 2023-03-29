use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::Debug;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WhiskError {
    pub code: String,
    pub error: String,
}
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
            insecure: self.insecure,
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

        let connectiontype = match wskprops {
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

pub fn whisk_errors(code: StatusCode, message: String) -> String {
    format!(": Error -> [ Status :{}, Message : {} ]", code, message)
}
