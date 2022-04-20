use crate::{client::Context, KeyValue};
use derive_new::new;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{traits::Service, HttpMethods, Limits, ACTION_ENDPOINT};

#[derive(new, Debug, Deserialize, Serialize, Clone)]
pub struct ActionService<T> {
    client: T,
    context: Context,
}

#[derive(Debug, Deserialize, Serialize, Clone,PartialEq)]
pub struct Action {
    #[serde(default)]
    pub namespace: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing)]
    pub version: String,
    #[serde(skip_serializing)]
    pub limits: Limits,
    pub exec: Exec,
    #[serde(default)]
    #[serde(skip_serializing)]
    pub error: String,
    #[serde(skip_serializing)]
    pub publish: bool,
    #[serde(skip_serializing)]
    pub updated: i64,
    pub annotations: Vec<KeyValue>,
}
#[derive(Debug, Deserialize, Serialize, Clone,PartialEq)]
pub struct Exec {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub image: String,
    #[serde(default)]
    pub init: String,
    #[serde(default)]
    pub main: String,
    #[serde(default)]
    pub components: Vec<String>,
    #[serde(default)]
    pub binary: bool,
}

impl<T> ActionService<T>
where
    T: Service,
{
    pub fn list(&self) -> Result<Vec<Action>, String> {
        let url = format!(
            "{}/api/v1/namespaces/{}/{}",
            self.context.host(),
            self.context.namespace(),
            ACTION_ENDPOINT
        );

        let user_auth = self.context.auth();
        let user = user_auth.0;
        let pass = user_auth.1;

        let request = self
            .client
            .new_request(HttpMethods::GET, url.as_str(), Some((user, pass)), None)
            .unwrap();

        match self.client.invoke_request(request) {
            Ok(x) => match serde_json::from_value(x) {
                Ok(actions) => Ok(actions),
                Err(err) => Err(format!("Failed to deserailize actions {}", err)),
            },
            Err(x) => Err(format!("Failed to fetch the list of actions {}", x)),
        }
    }

    pub fn get(&self, action_name: &str, fetch_code: bool) -> Result<Action, String> {
        let url = format!(
            "{}/api/v1/namespaces/{}/{}/{}?code={}",
            self.context.host(),
            self.context.namespace(),
            ACTION_ENDPOINT,
            action_name,
            fetch_code
        );

        let user_auth = self.context.auth();
        let user = user_auth.0;
        let pass = user_auth.1;

        let request = self
            .client
            .new_request(HttpMethods::GET, url.as_str(), Some((user, pass)), None)
            .unwrap();

        match self.client.invoke_request(request) {
            Ok(x) =>  match serde_json::from_value(x) {
                Ok(actions) => Ok(actions),
                Err(err) => Err(format!("Failed to deserailize actions {}", err)),
            },
            Err(x) => Err(format!("Failed to get action properties {}", x)),
        }
    }

    pub fn delete(&self, action_name: &str) -> Result<Action, String> {
        let url = format!(
            "{}/api/v1/namespaces/{}/{}/{}?code=false",
            self.context.host(),
            self.context.namespace(),
            ACTION_ENDPOINT,
            action_name,
        );

        let user_auth = self.context.auth();
        let user = user_auth.0;
        let pass = user_auth.1;

        let request = self
            .client
            .new_request(HttpMethods::DELETE, url.as_str(), Some((user, pass)), None)
            .unwrap();

        match self.client.invoke_request(request) {
            Ok(x) => match serde_json::from_value(x) {
                Ok(actions) => Ok(actions),
                Err(err) => Err(format!("Failed to deserailize actions {}", err)),
            },
            Err(x) => Err(format!("Failed to get action properties {}", x)),
        }
    }

    pub fn insert(&self, action: &Action, overwrite: bool) -> Result<Action, String> {
        let url = format!(
            "{}/api/v1/namespaces/{}/{}/{}?overwrite={}",
            self.context.host(),
            self.context.namespace(),
            ACTION_ENDPOINT,
            action.name,
            overwrite,
        );

        let user_auth = self.context.auth().clone();
        let user = user_auth.0;
        let pass = user_auth.1;

        let body = serde_json::to_value(&action).unwrap();
        println!("{}",body);

        let request = self
            .client
            .new_request(
                HttpMethods::PUT,
                url.as_str(),
                Some((user, pass)),
                Some(body),
            )
            .unwrap();

        match self.client.invoke_request(request) {
            Ok(x) =>  match serde_json::from_value(x) {
                Ok(actions) => Ok(actions),
                Err(err) => Err(format!("Failed to deserailize actions {}", err)),
            },
            Err(x) => Err(format!("Failed to get action properties {}", x)),
        }
    }
    pub fn invoke(
        &self,
        action_name: &str,
        payload: Value,
        blocking: bool,
        result: bool,
    ) -> Result<Action, String> {
        let url = format!(
            "{}/api/v1/namespaces/{}/{}/{}?blocking={}&result={}",
            self.context.host(),
            self.context.namespace(),
            ACTION_ENDPOINT,
            action_name,
            blocking,
            result
        );
        let user_auth = self.context.auth();
        let user = user_auth.0;
        let pass = user_auth.1;

        let request = self
            .client
            .new_request(
                HttpMethods::POST,
                url.as_str(),
                Some((user, pass)),
                Some(payload),
            )
            .unwrap();

        match self.client.invoke_request(request) {
            Ok(x) => match serde_json::from_value(x) {
                Ok(actions) => Ok(actions),
                Err(err) => Err(format!("Failed to deserailize actions {}", err)),
            },
            Err(x) => Err(format!("Failed to get action properties {}", x)),
        }
    }
}
