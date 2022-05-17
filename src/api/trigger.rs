use super::{HttpMethods, Limits, Service, TRIGGERS_ENDPOINT};
use crate::client::Context; 
use super::NAMESPACE_ENDPOINT;
use derive_new::new;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(new,Default, Debug, Clone)]
pub struct TriggerService<T> {
    client: T,
    context: Context,
}

#[derive(Debug, Deserialize, Serialize, Clone,Default)]
pub struct Trigger {
   #[serde(default)]
    pub namespace: String,
   #[serde(default)]
    pub name: String,
   #[serde(default)]
    pub version: String,
   #[serde(default)]
    pub publish: bool,
   #[serde(default)]
    pub updated: i64,
    pub annotations: Vec<KeyValue>,
    #[serde(default)]
    pub parameters: Vec<KeyValue>,

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
    pub limit: i64,
    pub skip: i64,
    pub docs: bool,
}

impl<T> TriggerService<T>
where
    T: Service,
{
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
