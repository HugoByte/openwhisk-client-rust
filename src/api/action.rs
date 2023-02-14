use crate::client::Context;
use derive_new::new;
use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};

use super::{traits::Service, HttpMethods, KeyValue, Limits, ACTION_ENDPOINT, NAMESPACE_ENDPOINT};

/// Representation of Action Service
#[derive(new, Debug, Default, Deserialize, Serialize, Clone)]
pub struct ActionService<T> {
    /// A action service must have a client to handle http request
    client: T,
    /// A action service uses the context which sets openwhisk properties
    context: Context,
}

/// Represenation of Action
#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Action {
    /// A action must have a namspace where it exists
    #[serde(default)]
    pub namespace: String,
    /// A action must have a name to represent it
    #[serde(default)]
    pub name: String,
    /// A action must have a versioning
    #[serde(default)]
    pub version: String,
    /// A action can take concurrrent limit
    #[serde(default)]
    pub limits: Limits,
    /// A action must have Exec properties
    pub exec: Exec,
    /// A action must have error to handle error created
    #[serde(default)]
    pub error: String,
    /// Toggle to publish action
    #[serde(default)]
    pub publish: bool,
    /// Updated version count of actions
    #[serde(default)]
    pub updated: i64,
    /// Keyvalue pair for annotate Actions
    pub annotations: Vec<KeyValue>,
}

/// Actions Execucatble properties
#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Exec {
    /// Action's Kind
    #[serde(default)]
    pub kind: String,
    /// Action's Code
    #[serde(default)]
    pub code: String,
    /// Action's Image
    #[serde(default)]
    pub image: String,
    /// Action's Init method
    #[serde(default)]
    pub init: String,
    /// Action's Main method
    #[serde(default)]
    pub main: String,
    /// Action's components
    #[serde(default)]
    pub components: Vec<String>,
    /// Toogled to true Action will be of binary
    #[serde(default)]
    pub binary: bool,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct ActionList {
    pub name: String,
    pub namespace: String,
}

impl<T> ActionService<T>
where
    T: Service,
{
    /// Returns a list of Actions
    pub fn list(&self) -> Result<Vec<ActionList>, String> {
        let url = format!(
            "{}/api/v1/{}/{}/{}",
            self.context.host(),
            NAMESPACE_ENDPOINT,
            self.context.namespace(),
            ACTION_ENDPOINT
        );

        let user_auth = self.context.auth();
        let user = user_auth.0;
        let pass = user_auth.1;

        let request = match self.client.new_request(
            Some(HttpMethods::GET),
            url.as_str(),
            Some((user, pass)),
            None,
        ) {
            Ok(request) => request,
            Err(error) => return Err(error),
        };

        match self.client.invoke_request(request) {
            Ok(x) => {
                let actions: Result<Vec<Action>, Error> = serde_json::from_value(x);
                match actions {
                    Ok(actions) => {
                        let mut result = Vec::new();
                        for action in actions.into_iter() {
                            let actionlist = ActionList {
                                name: action.name,
                                namespace: action.namespace,
                            };

                            result.push(actionlist)
                        }

                        Ok(result)
                    }
                    Err(error) => Err(format!("Failed to deserailize actions {}", error)),
                }
            }

            Err(error) => Err(format!("Failed to fetch the list of actions {}", error)),
        }
    }

    ///
    /// Returns Properties of action by using action name
    ///
    /// # Arguments
    /// * `action_name` - String slice that holds action name
    /// * `fetch_code`  - Toggle to get code for the action
    ///

    pub fn get(&self, action_name: &str, fetch_code: bool) -> Result<Action, String> {
        let url = format!(
            "{}/api/v1/{}/{}/{}/{}?code={}",
            self.context.host(),
            NAMESPACE_ENDPOINT,
            self.context.namespace(),
            ACTION_ENDPOINT,
            action_name,
            fetch_code
        );

        let user_auth = self.context.auth();
        let user = user_auth.0;
        let pass = user_auth.1;

        let request = match self.client.new_request(
            Some(HttpMethods::GET),
            url.as_str(),
            Some((user, pass)),
            None,
        ) {
            Ok(request) => request,
            Err(error) => return Err(error),
        };

        match self.client.invoke_request(request) {
            Ok(x) => match serde_json::from_value(x) {
                Ok(actions) => Ok(actions),
                Err(error) => Err(format!("Failed to deserailize actions {}", error)),
            },
            Err(error) => Err(format!("Failed to get action properties {}", error)),
        }
    }

    ///
    /// Delete Action and returns deleted Action by using action name
    ///
    /// # Arguments
    /// * `action_name` - String slice that holds action name
    ///
    pub fn delete(&self, action_name: &str) -> Result<Action, String> {
        let url = format!(
            "{}/api/v1/{}/{}/{}/{}?code=false",
            self.context.host(),
            NAMESPACE_ENDPOINT,
            self.context.namespace(),
            ACTION_ENDPOINT,
            action_name,
        );

        let user_auth = self.context.auth();
        let user = user_auth.0;
        let pass = user_auth.1;

        let request = match self.client.new_request(
            Some(HttpMethods::DELETE),
            url.as_str(),
            Some((user, pass)),
            None,
        ) {
            Ok(request) => request,
            Err(error) => return Err(error),
        };

        match self.client.invoke_request(request) {
            Ok(x) => match serde_json::from_value(x) {
                Ok(actions) => Ok(actions),
                Err(err) => Err(format!("Failed to deserailize actions {}", err)),
            },
            Err(x) => Err(format!("Failed to get action properties {}", x)),
        }
    }

    ///
    /// Insert Action and returns new action created
    ///
    /// # Arguments
    /// * `action`    - String slice that holds action name
    /// * `overwrite` - Bool toggle overwite of action if it present already
    ///
    pub fn insert(&self, action: &Action, overwrite: bool) -> Result<Action, String> {
        let url = format!(
            "{}/api/v1/{}/{}/{}/{}?overwrite={}",
            self.context.host(),
            NAMESPACE_ENDPOINT,
            self.context.namespace(),
            ACTION_ENDPOINT,
            action.name,
            overwrite,
        );

        let user_auth = self.context.auth();
        let user = user_auth.0;
        let pass = user_auth.1;

        let body = serde_json::to_value(action).unwrap();

        let request = match self.client.new_request(
            Some(HttpMethods::PUT),
            url.as_str(),
            Some((user, pass)),
            Some(body),
        ) {
            Ok(request) => request,
            Err(error) => return Err(error),
        };

        match self.client.invoke_request(request) {
            Ok(x) => match serde_json::from_value(x) {
                Ok(actions) => Ok(actions),
                Err(err) => Err(format!("Failed to deserailize actions {}", err)),
            },
            Err(x) => Err(format!("Failed to get action properties {}", x)),
        }
    }

    ///
    /// Invoke Action and returns action result
    ///
    /// # Arguments
    /// * `action_name` - String slice that holds action name
    /// * `payload`     - Params that action takes for exection
    /// * `blocking`    - Toggle to block action execution until it returns result
    /// * `result`      - Toggled only action result is returned
    ///
    pub fn invoke(
        &self,
        action_name: &str,
        payload: Value,
        blocking: bool,
        result: bool,
    ) -> Result<Value, String> {
        let url = format!(
            "{}/api/v1/{}/{}/{}/{}?blocking={}&result={}",
            self.context.host(),
            NAMESPACE_ENDPOINT,
            self.context.namespace(),
            ACTION_ENDPOINT,
            action_name,
            blocking,
            result
        );
        let user_auth = self.context.auth();
        let user = user_auth.0;
        let pass = user_auth.1;

        let request = match self.client.new_request(
            Some(HttpMethods::POST),
            url.as_str(),
            Some((user, pass)),
            Some(payload),
        ) {
            Ok(request) => request,
            Err(error) => return Err(error),
        };

        match self.client.invoke_request(request) {
            Ok(x) => match serde_json::from_value(x) {
                Ok(actions) => Ok(actions),
                Err(err) => Err(format!("Failed to deserailize actions {}", err)),
            },
            Err(x) => Err(format!("Failed to invoke action {}", x)),
        }
    }
}
