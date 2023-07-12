use derive_new::new;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{HttpMethods, KeyValue, Service, NAMESPACE_ENDPOINT, RULES_ENDPOINT};
use crate::client::Context;

/// Representation of rule Service
#[derive(new, Default, Debug, Clone)]
pub struct RuleService<T> {
    /// A rule service must have a client to handle http request
    client: T,
    /// A rule service uses the context which sets openwhisk properties
    context: Context,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Rule {
    pub name: String,
    pub trigger: String,
    pub action: String,
    pub status: String,
}

impl Rule {
    fn body(namespace: String, rule: &Rule) -> Result<Value, serde_json::Error> {
        let trigger = format!("/{}/{}/", namespace, rule.trigger);

        let action = format!("/{}/{}/", namespace, rule.action);

        serde_json::to_value(Rule {
            name: rule.name.clone(),
            trigger,
            action,
            status: rule.status.clone(),
        })
    }
}

/// Representation of Rule
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuleResponse {
    /// A rule must have a namspace where it exists
    #[serde(default)]
    pub namespace: String,
    /// A rule must have a name to represent it
    #[serde(default)]
    pub name: String,
    /// A action must have a versioning
    #[serde(default)]
    pub version: String,
    /// Keyvalue pair for annotate rules
    #[serde(default)]
    pub annotations: Vec<KeyValue>,
    /// The execution status of the rule
    #[serde(default)]
    pub status: String,
    /// A rule must have a trigger mapped to it
    #[serde(default)]
    pub trigger: Value,
    /// A rule must have an action to pass the trigger
    #[serde(default)]
    pub action: Value,
    /// Toggle to publish rule
    #[serde(default)]
    pub publish: bool,
    /// Updated version count of actions
    #[serde(default)]
    pub updated: i64,
}

/// Representation of rules list options
#[derive(new, Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuleListOptions {
    /// The limit for the required rules.
    pub limit: i64,
    /// The counts to be skipped.
    pub skip: i64,
    /// Toggle to get documents.
    pub docs: bool,
}

impl RuleResponse {
    fn set_status(state: String) -> Self {
        Self {
            status: state,
            ..Default::default()
        }
    }
}

impl<T> RuleService<T>
where
    T: Service,
{
    /// Returns a list of Rules
    pub fn list(&self) -> Result<Vec<RuleResponse>, String> {
        let url = format!(
            "{}/api/v1/{}/{}/{}",
            self.context.host(),
            NAMESPACE_ENDPOINT,
            self.context.namespace(),
            RULES_ENDPOINT,
        );

        let auth = self.context.auth();
        let user = auth.0;
        let pass = auth.1;

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
                Ok(rules) => Ok(rules),
                Err(err) => Err(format!("Failed to deserailize Rules {}", err)),
            },
            Err(x) => Err(format!("Failed to fetch the list of Rules {}", x)),
        }
    }

    /// Inserts a rule
    ///
    /// # Arguments
    /// * `rule` - The rule ro be inserted
    /// * `overwrite`  - Toggle to get overwrtite an existing rule
    ///  
    pub fn insert(&self, rule: &Rule, overwrite: bool) -> Result<RuleResponse, String> {
        let url = format!(
            "{}/api/v1/{}/{}/{}/{}?overwrite={}",
            self.context.host(),
            NAMESPACE_ENDPOINT,
            self.context.namespace(),
            RULES_ENDPOINT,
            rule.name,
            overwrite
        );

        let auth = self.context.auth();
        let user = auth.0;
        let pass = auth.1;

        let body = match Rule::body(self.context.namespace().to_string(), rule) {
            Ok(body) => body,
            Err(error) => return Err(format!("Failed deserailize body {}", error)),
        };

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
                Err(err) => Err(format!("Failed to deserailize rule {}", err)),
            },
            Err(x) => Err(format!("Failed to create rule {}", x)),
        }
    }

    /// To get the properties of the rule
    ///
    /// # Arguments
    /// * `rule_name` - String slice that holds rule name
    ///
    pub fn get(&self, rule_name: &str) -> Result<RuleResponse, String> {
        let url = format!(
            "{}/api/v1/{}/{}/{}/{}",
            self.context.host(),
            NAMESPACE_ENDPOINT,
            self.context.namespace(),
            RULES_ENDPOINT,
            rule_name
        );

        let auth = self.context.auth();
        let user = auth.0;
        let pass = auth.1;

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
                Err(err) => Err(format!("Failed to deserailize rule {}", err)),
            },
            Err(x) => Err(format!("Failed to get rule properties{}", x)),
        }
    }

    /// Deletes an already existing rule
    ///
    /// # Arguments
    /// * `rule_name` - String slice that holds rule name
    ///
    pub fn delete(&self, rule_name: &str) -> Result<RuleResponse, String> {
        let url = format!(
            "{}/api/v1/{}/{}/{}/{}",
            self.context.host(),
            NAMESPACE_ENDPOINT,
            self.context.namespace(),
            RULES_ENDPOINT,
            rule_name
        );

        let auth = self.context.auth();
        let user = auth.0;
        let pass = auth.1;

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
                Err(err) => Err(format!("Failed to deserailize rule {}", err)),
            },
            Err(x) => Err(format!("Failed to get rule properties{}", x)),
        }
    }

    /// Sets the state of the rule
    ///
    /// # Arguments
    /// * `rule_name` - String slice that holds rule name
    /// * 'state' - Execution state of the rule    
    ///
    pub fn set_state(&self, rule_name: &str, state: &str) -> Result<String, String> {
        let state = state.to_lowercase();

        if state != "active" && state != "inactive" {
            Err("Invalid setstate options".to_string())
        } else {
            let url = format!(
                "{}/api/v1/{}/{}/{}/{}",
                self.context.host(),
                NAMESPACE_ENDPOINT,
                self.context.namespace(),
                RULES_ENDPOINT,
                rule_name
            );

            let auth = self.context.auth();
            let user = auth.0;
            let pass = auth.1;

            let setstate = RuleResponse::set_status(state);

            let body = match serde_json::to_value(setstate) {
                Ok(body) => body,
                Err(error) => return Err(format!("Failed deserailize body {}", error)),
            };

            let request = match self.client.new_request(
                Some(HttpMethods::POST),
                url.as_str(),
                Some((user, pass)),
                Some(body),
            ) {
                Ok(request) => request,
                Err(error) => return Err(error),
            };

            match self.client.invoke_request(request) {
                Ok(_x) => Ok("The rule is updated".to_string()),
                Err(x) => Err(format!("Failed to SetState for Rule {}", x)),
            }
        }
    }
}
