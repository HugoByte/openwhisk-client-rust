use crate::client::Context;
use derive_new::new;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::traits::Service;

static ACTION_ENDPOINT: &str = "actions";

enum HttpMethods {
    GET,
    PUT,
    POST,
    DELETE,
}

impl HttpMethods {
    fn to_string(&self) -> String {
        match self {
            HttpMethods::GET => "get".to_string(),
            HttpMethods::PUT => "put".to_string(),
            HttpMethods::POST => "post".to_string(),
            HttpMethods::DELETE => "delete".to_string(),
        }
    }
}

#[derive(new, Debug, Deserialize, Serialize, Clone)]
pub struct ActionService<T> {
    client: T,
    context: Context,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Action {
    namespace: String,
    name: String,
    version: String,
    exec: Exec,
    error: String,
    publish: bool,
    updated: i64,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Exec {
    kind: String,
    coode: String,
    image: String,
    init: String,
    main: String,
    components: Vec<String>,
    binary: bool,
}

impl<T> ActionService<T>
where
    T: Service,
{
    pub fn list(&self) -> Result<Value, String> {
        let url = format!(
            "{}/api/v1/namespaces/{}/{}",
            self.context.host(),
            self.context.namespace(),
            ACTION_ENDPOINT
        );

        let user_auth = self.context.auth().clone();
        let user = user_auth.0.to_owned();
        let pass = user_auth.1.to_owned();

        let request = self
            .client
            .new_request(HttpMethods::GET.to_string(), url, Some((user, pass)), None)
            .unwrap();

        match self.client.invoke_request(request) {
            Ok(x) => Ok(x),
            Err(x) => Err(format!("Failed to fetch the list of actions {}", x)),
        }
    }
}
