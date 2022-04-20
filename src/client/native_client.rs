use crate::api::{
    ActionService, HttpMethods, NamespaceService, RuleService, Service, TriggerService,
};
use http::StatusCode;
use reqwest::blocking::Client;
use serde_json::Value;

use super::common::{Context, WskProperties};

#[derive(Debug, Clone)]
pub struct NativeClient {
    pub context: Context,
    pub client: Client,
    actions: ActionService<Client>,
    triggers: TriggerService<Client>,
    rules: RuleService<Client>,
    namespaces: NamespaceService<Client>,
}

impl NativeClient {
    pub fn new(config: Option<&WskProperties>) -> Self {
        let context = Context::new(config);
        let client = http_client(context.is_secure());
        let actions = ActionService::new(client.clone(), context.clone());
        let triggers = TriggerService::new(client.clone(), context.clone());
        let rules = RuleService::new(client.clone(), context.clone());
        let namespaces = NamespaceService::new(client.clone(), context.clone());
        Self {
            client,
            context,
            actions,
            triggers,
            rules,
            namespaces,
        }
    }

    pub fn actions(&self) -> &ActionService<Client> {
        &self.actions
    }

    pub fn triggers(&self) -> &TriggerService<Client>{
        &self.triggers
    }

    pub fn rules(&self) -> &RuleService<Client>{
        &self.rules
    }

    pub fn namespaces(&self) -> &NamespaceService<Client>{
        &self.namespaces
    }
}

fn http_client(insecure: bool) -> Client {
    let client = match insecure {
        true => reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(insecure)
            .timeout(None)
            .build()
            .unwrap(),
        false => reqwest::blocking::Client::builder()
            .timeout(None)
            .build()
            .unwrap(),
    };

    return client;
}

impl Service for Client {
    type Output = reqwest::blocking::RequestBuilder;

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
                    HttpMethods::GET => return Ok(self.get(url).basic_auth(user, Some(pass))),
                    HttpMethods::POST => {
                        return Ok(self.post(url).basic_auth(user, Some(pass)).json(&body))
                    }
                    HttpMethods::PUT => {
                        return Ok(self.put(url).basic_auth(user, Some(pass)).json(&body))
                    }
                    HttpMethods::DELETE => {
                        return Ok(self.delete(url).basic_auth(user, Some(pass)).json(&body))
                    }
                    _ => Err(format!("Falied to create request")),
                }
            }
            None => match method {
                HttpMethods::GET => return Ok(self.get(url)),
                HttpMethods::POST => return Ok(self.post(url).json(&body)),
                HttpMethods::PUT => return Ok(self.put(url).json(&body)),
                HttpMethods::DELETE => return Ok(self.delete(url).json(&body)),
                _ => Err(format!("Falied to create request")),
            },
        }
    }

    fn invoke_request(&self, request: Self::Output) -> Result<Value, String> {
        if let Ok(response) = request.send() {
            return match response.status() {
                StatusCode::OK => Ok(response.json().unwrap()),
                _ => Err(format!("failed to invoke request {}",response.status())),
            };
        };
        Err(format!("failed to invoke request"))
    }
}
