use super::common::{whisk_errors, OpenWhisk, WhiskError};
use crate::api::{HttpMethods, Service};
use http::StatusCode;
use reqwest::blocking::Client;
use serde_json::Value;

/// A Client to make Requests with.
#[derive(Debug, Default)]
pub struct NativeClient(Client);

impl OpenWhisk for NativeClient {
    /// NativeClient - Http Client (Here client is Reqwest Client)
    type Output = NativeClient;
    /// Creates New WhiskClient
    ///
    /// # Arguments
    /// * `insecure` - Option of Bool to specify connection type
    fn new_whisk_client(insecure: Option<bool>) -> Self::Output {
        match insecure {
            Some(x) => match x {
                true => NativeClient(
                    reqwest::blocking::Client::builder()
                        .danger_accept_invalid_certs(x)
                        .timeout(None)
                        .build()
                        .unwrap(),
                ),
                false => NativeClient(
                    reqwest::blocking::Client::builder()
                        .timeout(None)
                        .build()
                        .unwrap(),
                ),
            },
            None => todo!(),
        }
    }
}

impl Service for NativeClient {
    type Output = reqwest::blocking::RequestBuilder;

    ///
    /// Creates New Request and Returns  `reqwest::blocking::RequestBuilder`
    ///
    /// # Arguments
    /// * `method`   - Option of HTTPMethods
    /// * `url`      - API Host url
    /// * `use_auth` - Option of tuple conatining Username and Password
    /// * `body`     - Option of value which can have parameters necessary for the body of request
    ///
    fn new_request(
        &self,
        method: Option<HttpMethods>,
        url: &str,
        use_auth: Option<(&str, &str)>,
        body: Option<Value>,
    ) -> Result<Self::Output, String> {
        let body = body.unwrap_or_else(|| serde_json::json!({}));

        match use_auth {
            Some(auth) => {
                let user = auth.0;
                let pass = auth.1;

                match method {
                    Some(http_method) => match http_method {
                        HttpMethods::GET => Ok(self.0.get(url).basic_auth(user, Some(pass))),
                        HttpMethods::POST => {
                            Ok(self.0.post(url).basic_auth(user, Some(pass)).json(&body))
                        }
                        HttpMethods::PUT => {
                            Ok(self.0.put(url).basic_auth(user, Some(pass)).json(&body))
                        }
                        HttpMethods::DELETE => {
                            Ok(self.0.delete(url).basic_auth(user, Some(pass)).json(&body))
                        }
                    },
                    None => Err("Falied to create request".to_string()),
                }
            }
            None => match method {
                Some(http_method) => match http_method {
                    HttpMethods::GET => Ok(self.0.get(url)),
                    HttpMethods::POST => Ok(self.0.post(url).json(&body)),
                    HttpMethods::PUT => Ok(self.0.put(url).json(&body)),
                    HttpMethods::DELETE => Ok(self.0.delete(url).json(&body)),
                },
                None => Err("Falied to create request".to_string()),
            },
        }
    }

    ///
    /// To invoke request and get response out of request execution
    ///
    /// # Arguments
    ///
    /// * `request` - Http request with url,auth and body
    ///
    ///
    ///
    fn invoke_request(&self, request: Self::Output) -> Result<Value, String> {
        match request.send() {
            Ok(response) => match response.status() {
                StatusCode::OK => Ok(response.json().unwrap_or_default()),
                _ => {
                    let code = response.status();
                    let error: WhiskError = response.json().unwrap();

                    Err(whisk_errors(code, error.error))
                }
            },
            Err(error) => Err(format!("{}", error)),
        }
    }
}

impl Clone for NativeClient {
    fn clone(&self) -> Self {
        NativeClient(self.0.clone())
    }

    #[allow(clippy::unnecessary_operation)]
    fn clone_from(&mut self, _source: &Self) {
        NativeClient(self.0.clone());
    }
}
