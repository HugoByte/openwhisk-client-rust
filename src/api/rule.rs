use derive_new::new;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{HttpMethods, KeyValue, Service, NAMESPACE_ENDPOINT, RULES_ENDPOINT};
use crate::client::Context;

#[derive(new, Debug, Clone)]
pub struct RuleService<T> {
    client: T,
    context: Context,
}

#[derive(new, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Rule {
    #[serde(default)]
    pub namespace: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub annotations: Vec<KeyValue>,
    pub status: String,
    #[serde(default)]
    pub trigger: Value,
    #[serde(default)]
    pub action: Value,
    #[serde(default)]
    pub publish: bool,
    #[serde(default)]
    pub updated: i64,
}

#[derive(new, Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuleListOptions {
    pub limit: i64,
    pub skip: i64,
    pub docs: bool,
}

impl Rule {
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
    pub fn list(&self) -> Result<Vec<Rule>, String> {
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

        let request = self
            .client
            .new_request(HttpMethods::GET, url.as_str(), Some((user, pass)), None)
            .unwrap();

        match self.client.invoke_request(request) {
            Ok(x) => match serde_json::from_value(x) {
                Ok(actions) => Ok(actions),
                Err(err) => Err(format!("Failed to deserailize Rules {}", err)),
            },
            Err(x) => Err(format!("Failed to fetch the list of Rules {}", x)),
        }
    }

    pub fn insert(&self, rule: &Rule, overwrite: bool) -> Result<Rule, String> {
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

        let body = match serde_json::to_value(rule) {
            Ok(body) => body,
            Err(error) => return Err(format!("Failed deserailize body {}", error)),
        };

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
            Ok(x) => match serde_json::from_value(x) {
                Ok(actions) => Ok(actions),
                Err(err) => Err(format!("Failed to deserailize rule {}", err)),
            },
            Err(x) => Err(format!("Failed to create rule {}", x)),
        }
    }

    pub fn get(&self, rule_name: &str) -> Result<Rule, String> {
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

        let request = self
            .client
            .new_request(HttpMethods::PUT, url.as_str(), Some((user, pass)), None)
            .unwrap();

        match self.client.invoke_request(request) {
            Ok(x) => match serde_json::from_value(x) {
                Ok(actions) => Ok(actions),
                Err(err) => Err(format!("Failed to deserailize rule {}", err)),
            },
            Err(x) => Err(format!("Failed to get rule properties{}", x)),
        }
    }

    pub fn delete(&self, rule_name: &str) -> Result<Rule, String> {
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

        let request = self
            .client
            .new_request(HttpMethods::PUT, url.as_str(), Some((user, pass)), None)
            .unwrap();

        match self.client.invoke_request(request) {
            Ok(x) => match serde_json::from_value(x) {
                Ok(actions) => Ok(actions),
                Err(err) => Err(format!("Failed to deserailize rule {}", err)),
            },
            Err(x) => Err(format!("Failed to get rule properties{}", x)),
        }
    }

    pub fn setstate(&self, rule_name: &str, state: &str) -> Result<Rule, String> {
        let state = state.to_lowercase();

        if state != "active" && state != "inactive" {
            return Err(format!("Invalid setstate options"));
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

            let setstate = Rule::set_status(state);

            let body = match serde_json::to_value(setstate) {
                Ok(body) => body,
                Err(error) => return Err(format!("Failed deserailize body {}", error)),
            };

            let request = self
                .client
                .new_request(
                    HttpMethods::POST,
                    url.as_str(),
                    Some((user, pass)),
                    Some(body),
                )
                .unwrap();

            match self.client.invoke_request(request) {
                Ok(x) => match serde_json::from_value(x) {
                    Ok(actions) => Ok(actions),
                    Err(err) => Err(format!("Failed to deserailize rule {}", err)),
                },
                Err(x) => Err(format!("Failed to SetState for Rule {}", x)),
            }
        }
    }
}
