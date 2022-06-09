use super::{HttpMethods, Limits, Service, TRIGGERS_ENDPOINT};
use crate::client::Context; 
use super::NAMESPACE_ENDPOINT;
use derive_new::new;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Representation of Trigger Service
#[derive(new,Default, Debug, Clone)]
pub struct TriggerService<T> {
    /// A trigger service must have a client to handle http request
    client: T,
    /// A trigger service uses the context which sets openwhisk properties
    context: Context,
}

/// Represenation of Trigger 
#[derive(Debug, Deserialize, Serialize, Clone,Default)]
pub struct Trigger {
    /// The namespace name
   #[serde(default)]
    pub namespace: String,
    /// The trigger name
   #[serde(default)]
    pub name: String,
    /// Version
   #[serde(default)]
    pub version: String,
    /// Publish to true or flase
   #[serde(default)]
    pub publish: bool,
    /// Number of times the trigger has been updated
   #[serde(default)]
    pub updated: i64,
    /// Annotations used
    pub annotations: Vec<KeyValue>,
    /// Parameters required
    #[serde(default)]
    pub parameters: Vec<KeyValue>,
    /// Trigger rate Limits  
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    pub limits: Limits,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct KeyValue {
    pub key: String,
    pub value: Value,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TriggerListOptions {
    /// The limit for the trigger
    pub limit: i64,
    /// Skip count
    pub skip: i64,
    /// And the document is required or not should be passed as parameters
    pub docs: bool,
}

impl<T> TriggerService<T>
where
    T: Service,
{
    /// Returns a list of Triggers
    pub fn list(&self) -> Result<Vec<Trigger>, String> {
        let url = format!(
            "{}/api/v1/{}/{}/{}",
            self.context.host(),
            NAMESPACE_ENDPOINT,
            self.context.namespace(),
            TRIGGERS_ENDPOINT
        );

        let user_auth = self.context.auth();
        let user = user_auth.0;
        let pass = user_auth.1;

        let request = self
            .client
            .new_request(HttpMethods::GET, url.as_str(), Some((user, pass)), None)
            .unwrap();

        match self.client.invoke_request(request) {
            Ok(value) => match serde_json::from_value(value) {
                Ok(result) => Ok(result),
                Err(error) => Err(format!("Failed deserailize triggers {}", error)),
            },

            Err(error) => Err(format!("Failed to fetch the list of triggers {}", error)),
        }
    }

    /// Inserts a trigger
    /// 
    /// # Arguments
    /// * `trigger` - The trigger ro be inserted
    /// * `overwrite`  - Toggle to get overwrtite an existing trigger 
    /// 
    pub fn insert(&self, trigger: &Trigger, overwrite: bool) -> Result<Trigger, String> {
        let url = format!(
            "{}/api/v1/{}/{}/{}/{}?overwrite={}",
            self.context.host(),
            NAMESPACE_ENDPOINT,
            self.context.namespace(),
            TRIGGERS_ENDPOINT,
            trigger.name,
            overwrite,
        );

        let user_auth = self.context.auth();
        let user = user_auth.0;
        let pass = user_auth.1;

        let body = match serde_json::to_value(trigger) {
            Ok(value) => value,
            Err(err) => return Err(format!("failed to serialize body {}", err)),
        };

        let request = match self.client.new_request(
            HttpMethods::PUT,
            url.as_str(),
            Some((user, pass)),
            Some(body),
        ) {
            Ok(request) => request,
            Err(err) => return Err(format!("failed to create request {}", err)),
        };

        let trigger: Trigger = match self.client.invoke_request(request) {
            Ok(response) => match serde_json::from_value(response) {
                Ok(trigger) => trigger,
                Err(err) => return Err(format!("falied to deserilaize {}", err)),
            },
            Err(err) => return Err(format!("falied to insert trigger {}", err)),
        };

        Ok(trigger)
    }

    /// To get the properties of the trigger
    /// 
    /// # Arguments
    /// * `trigger_name` - String slice that holds trigger name
    /// 
    pub fn get(&self, trigger_name: &str) -> Result<Trigger, String> {
        let url = format!(
            "{}/api/v1/{}/{}/{}/{}",
            self.context.host(),
            NAMESPACE_ENDPOINT,
            self.context.namespace(),
            TRIGGERS_ENDPOINT,
            trigger_name
        );

        let user_auth = self.context.auth();
        let user = user_auth.0;
        let pass = user_auth.1;

        let request =
            match self
                .client
                .new_request(HttpMethods::GET, url.as_str(), Some((user, pass)), None)
            {
                Ok(request) => request,
                Err(err) => return Err(format!("falied to create request {}", err)),
            };

        let trigger: Trigger = match self.client.invoke_request(request) {
            Ok(response) => match serde_json::from_value(response) {
                Ok(trigger) => trigger,
                Err(err) => return Err(format!("falied to deserilaize {}", err)),
            },
            Err(err) => return Err(format!("falied to insert trigger {}", err)),
        };

        Ok(trigger)
    }

    /// Deletes an already existing trigger
    /// # Arguments
    /// * `trigger_name` - String slice that holds trigger name
    /// 
    pub fn delete(&self, trigger_name: &str) -> Result<Trigger, String> {
        let url = format!(
            "{}/api/v1/{}/{}/{}/{}",
            self.context.host(),
            NAMESPACE_ENDPOINT,
            self.context.namespace(),
            TRIGGERS_ENDPOINT,
            trigger_name
        );
        let user_auth = self.context.auth();
        let user = user_auth.0;
        let pass = user_auth.1;

        let request = match self.client.new_request(
            HttpMethods::DELETE,
            url.as_str(),
            Some((user, pass)),
            None,
        ) {
            Ok(request) => request,
            Err(err) => return Err(format!("falied to create request {}", err)),
        };

        let trigger: Trigger = match self.client.invoke_request(request) {
            Ok(response) => match serde_json::from_value(response) {
                Ok(trigger) => trigger,
                Err(err) => return Err(format!("falied to deserilaize {}", err)),
            },
            Err(err) => return Err(format!("falied to insert trigger {}", err)),
        };

        Ok(trigger)
    }

    /// Fires a trigger to an action
    /// 
    ///  # Arguments
    /// * `trigger_name` - String slice that holds trigger name
    /// * `payload` - payload is the result of the action
    /// 
    pub fn fire(&self, trigger_name: &str, payload: Value) -> Result<Trigger, String> {
        let url = format!(
            "{}/api/v1/{}/{}/{}/{}",
            self.context.host(),
            NAMESPACE_ENDPOINT,
            self.context.namespace(),
            TRIGGERS_ENDPOINT,
            trigger_name
        );

        let user_auth = self.context.auth();
        let user = user_auth.0;
        let pass = user_auth.1;

        let request = match self.client.new_request(
            HttpMethods::POST,
            url.as_str(),
            Some((user, pass)),
            Some(payload),
        ) {
            Ok(request) => request,
            Err(err) => return Err(format!("falied to create request {}", err)),
        };

        let trigger: Trigger = match self.client.invoke_request(request) {
            Ok(response) => match serde_json::from_value(response) {
                Ok(trigger) => trigger,
                Err(err) => return Err(format!("falied to deserilaize {}", err)),
            },
            Err(err) => return Err(format!("falied to insert trigger {}", err)),
        };

        Ok(trigger)
    }
}
