use chesterfield::sync::Database;
use derive_new::new;
use reqwest;
use reqwest::StatusCode;
use serde_json::{Error, Value};
use std::env;

#[derive(new, Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub host: String,
    pub name: String,
    pub namespace: String,
    pub insecure: bool,
}

pub struct Context {
    pub host: String,
    pub name: String,
    pub namespace: String,
    db: Database,
    user: String,
    pass: String,
    pub insecure: bool,
}

fn client(insecure: bool) -> reqwest::blocking::Client {
    if insecure == true {
        return reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(insecure)
            .timeout(None)
            .build()
            .unwrap();
    } else {
        return reqwest::blocking::Client::builder()
            .timeout(None)
            .build()
            .unwrap();
    }
}

fn invoke_client(
    request: reqwest::blocking::RequestBuilder,
) -> Result<reqwest::blocking::Response, reqwest::Error> {
    request.send()
}

impl Context {
    pub fn new(db: Database, config: Option<&Config>) -> Self {
        let api_key = if env::var("__OW_API_KEY").is_ok() {
            env::var("__OW_API_KEY").unwrap()
        } else {
            match config {
                Some(config) => config.api_key.clone(),
                None => "test:test".to_string(),
            }
        };
        let auth: Vec<&str> = api_key.split(":").collect();
        let host = if env::var("__OW_API_HOST").is_ok() {
            env::var("__OW_API_HOST").unwrap()
        } else {
            match config {
                Some(config) => config.host.clone(),
                None => "host.docker.internal".to_string(),
            }
        };
        let name = if env::var("__OW_ACTION_NAME").is_ok() {
            env::var("__OW_ACTION_NAME").unwrap()
        } else {
            match config {
                Some(config) => config.name.clone(),
                None => "action".to_string(),
            }
        };
        let namespace = if env::var("__OW_NAMESPACE").is_ok() {
            env::var("__OW_NAMESPACE").unwrap()
        } else {
            match config {
                Some(config) => config.namespace.clone(),
                None => "guest".to_string(),
            }
        };

        let connectiontype = match config {
            Some(config) => config.insecure.clone(),
            None => true,
        };
        Context {
            host,
            db,
            name,
            namespace,
            user: auth[0].to_string(),
            pass: auth[1].to_string(),
            insecure: connectiontype,
        }
    }
    /// To get auth deatils
    pub fn get_auth_key(&self) -> (String, String) {
        return (self.user.clone(), self.pass.clone());
    }

    /// To get list of {actions,triggers,rules} in the specified namespaces
    pub fn get_list(&self, endpoint: &str) -> Result<Value, Error> {
        let client = client(self.insecure);
        let url = format!(
            "{}/api/v1/namespaces/{}/{}",
            self.host, self.namespace, endpoint
        );
        if let Ok(response) = invoke_client(
            client
                .get(url)
                .basic_auth(self.user.clone(), Some(self.pass.clone())),
        ) {
            return match response.status() {
                StatusCode::OK => return response.json().map_err(serde::de::Error::custom),
                _ => Err(format!("error fetching {} list", endpoint))
                    .map_err(serde::de::Error::custom),
            };
        };

        Err(format!("error fetching {} list", endpoint)).map_err(serde::de::Error::custom)
    }

    pub fn create_rule(&self, name: &str, trigger: &str, action: &str) -> Result<Value, Error> {
        let client = client(self.insecure);

        let url = format!(
            "{}/api/v1/namespaces/{}/rules/{}?overwrite=true",
            self.host, self.namespace, name
        );

        if let Ok(response) = invoke_client(
            client
                .put(url.clone())
                .basic_auth(self.user.clone(), Some(self.pass.clone()))
                .json(&serde_json::json!({
                    "status": "",
                    "action": format!("/{}/{}",self.namespace, action),
                    "trigger": format!("/{}/{}",self.namespace, trigger)
                })),
        ) {
            return match response.status() {
                StatusCode::OK => return response.json().map_err(serde::de::Error::custom),
                _ => Err(format!("failed to create rule  {}  ", name))
                    .map_err(serde::de::Error::custom),
            };
        };
        Err(format!("failed to create rule {} ", name)).map_err(serde::de::Error::custom)
    }

    pub fn create_trigger(&self, name: &str, value: &Value) -> Result<Value, Error> {
        let client = client(self.insecure);

        let url = format!(
            "{}/api/v1/namespaces/{}/triggers/{}?overwrite=true",
            self.host, self.namespace, name
        );

        if let Ok(response) = invoke_client(
            client
                .put(url.clone())
                .basic_auth(self.user.clone(), Some(self.pass.clone()))
                .json(value),
        ) {
            return match response.status() {
                StatusCode::OK => return response.json().map_err(serde::de::Error::custom),
                _ => Err(format!(
                    "failed to create trigger  {} {:?} ",
                    name,
                    response.error_for_status()
                ))
                .map_err(serde::de::Error::custom),
            };
        };

        Err(format!("failed to create trigger {} ", name)).map_err(serde::de::Error::custom)
    }

    pub fn invoke_trigger(&self, name: &str, value: &Value) -> Result<Value, Error> {
        let client = client(self.insecure);
        let url = format!(
            "{}/api/v1/namespaces/{}/triggers/{}?result=true",
            self.host, self.namespace, name
        );

        if let Ok(response) = invoke_client(
            client
                .post(url.clone())
                .basic_auth(self.user.clone(), Some(self.pass.clone()))
                .json(value),
        ) {
            return match response.status() {
                StatusCode::OK => return response.json().map_err(serde::de::Error::custom),
                _ => Err(format!(
                    "failed to invoke trigger  {} {:?} ",
                    name,
                    response.error_for_status()
                ))
                .map_err(serde::de::Error::custom),
            };
        };
        Err(format!("failed to invoke trigger {} ", name)).map_err(serde::de::Error::custom)
    }

    pub fn invoke_action(&self, name: &str, value: &Value) -> Result<Value, Error> {
        let client = client(self.insecure);
        let url = format!(
            "{}/api/v1/namespaces/{}/actions/{}?result=true",
            self.host, self.namespace, name
        );

        if let Ok(response) = invoke_client(
            client
                .post(url.clone())
                .basic_auth(self.user.clone(), Some(self.pass.clone()))
                .json(value),
        ) {
            return match response.status() {
                StatusCode::OK => return response.json().map_err(serde::de::Error::custom),
                _ => Err(format!(
                    "failed to invoke actions  {} {:?} ",
                    name,
                    response.error_for_status()
                ))
                .map_err(serde::de::Error::custom),
            };
        };
        Err(format!("failed to invoke actions {} ", name)).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use chesterfield::sync::Client;

    use super::*;

    fn context() -> Context{

        let config = Config::new(
            "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
             "https://localhost:31001".to_string(), 
             "actions".to_string(),
              "guest".to_string(), 
              true
        );

        let couch_db = Client::new("http://127.0.0.1:5984");

        return  Context::new(couch_db.unwrap().database("").unwrap(), Some(&config));
    }
    #[test]
    fn get_list_of_actions_pass() {
        
        let context = context();

        let result = context.get_list("actions").unwrap();

        let actual = vec!["cars".to_string(), "carlisy".to_string()];

        for index in 0..result.as_array().unwrap().len() {
            assert_eq!(result.as_array().unwrap()[index]["name"], actual[index]);
        }
    }

    #[test]
    fn create_triggers(){
        let topic = "1234".to_string();

        let context = context();

        context.create_trigger(&topic, &serde_json::json!({})).unwrap();

        let triggers = context.get_list("triggers").unwrap();

        for index in 0..triggers.as_array().unwrap().len() {
            assert_eq!(triggers.as_array().unwrap()[index]["name"], "1234".to_string());
        }

    }

    #[test]
    fn create_rules(){

        let trigger = "1234".to_string();

        let context = context();

        context.create_rule("rule1", &trigger, "cars").unwrap();

        let rules = context.get_list("rules").unwrap();

        for index in 0..rules.as_array().unwrap().len() {
            assert_eq!(rules.as_array().unwrap()[index]["name"], "rule1".to_string());
        }

    }
}
