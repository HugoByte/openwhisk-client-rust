use super::common::{whisk_errors, OpenWhisk};
use crate::api::{HttpMethods, Service};
use bytes::Bytes;
use http::{HeaderMap, Request, StatusCode};
use serde_json::{Error, Value};
use wasi_experimental_http::request as wasi_request;

#[derive(Debug, Default, Clone)]
pub struct WasmClient {
    headers: http::HeaderMap,
}
impl OpenWhisk for WasmClient {
    type Output = WasmClient;

    fn new_whisk_client(insecure: Option<bool>) -> Self::Output {
        let mut header_map = HeaderMap::new();
        match insecure {
            Some(x) => match x {
                true => header_map.insert("Upgrade-Insecure-Requests", "1".parse().unwrap()),
                false => header_map.insert("upgrade-insecure-requests", "0".parse().unwrap()),
            },
            None => todo!(),
        };
        WasmClient {
            headers: header_map,
        }
    }
}

impl Service for WasmClient {
    type Output = Request<Option<Bytes>>;

    fn new_request(
        &self,
        method: Option<HttpMethods>,
        url: &str,
        user_auth: Option<(&str, &str)>,
        body: Option<Value>,
    ) -> Result<Self::Output, String> {
        let mut req = http::request::Builder::new().header("Content-Type", "application/json");
        for (key, value) in self.headers.iter() {
            req = req.header(key, value);
        }
        let body = Bytes::from(serde_json::to_vec(&body).unwrap());
        match user_auth {
            Some(auth) => {
                let user = auth.0;
                let pass = auth.1;
                let bse64_encode = base64::encode(format!("{}:{}", user, pass));

                match method {
                    Some(http_methods) => match http_methods {
                        HttpMethods::GET => {
                            let req = req
                                .header("Authorization", format!("Basic {}", bse64_encode))
                                .method("GET")
                                .uri(url)
                                .body(Some(body));
                            match req {
                                Ok(req) => Ok(req),
                                Err(error) => Err(format!("{}", error)),
                            }
                        }
                        HttpMethods::PUT => {
                            let req = req
                                .header("Authorization", format!("Basic {}", bse64_encode))
                                .method("PUT")
                                .uri(url)
                                .body(Some(body));
                            match req {
                                Ok(req) => Ok(req),
                                Err(error) => Err(format!("{}", error)),
                            }
                        }
                        HttpMethods::POST => {
                            let req = req
                                .header("Authorization", format!("Basic {}", bse64_encode))
                                .method("POST")
                                .uri(url)
                                .body(Some(body));
                            match req {
                                Ok(req) => Ok(req),
                                Err(error) => Err(format!("{}", error)),
                            }
                        }
                        HttpMethods::DELETE => {
                            let req = req
                                .header("Authorization", format!("Basic {}", bse64_encode))
                                .method("DELETE")
                                .uri(url)
                                .body(Some(body));
                            match req {
                                Ok(req) => Ok(req),
                                Err(error) => Err(format!("{}", error)),
                            }
                        }
                    },
                    None => Err("Falied to create request".to_string()),
                }
            }
            None => match method {
                Some(http_methods) => match http_methods {
                    HttpMethods::GET => {
                        let req = req.method("GET").uri(url).body(Some(body));
                        match req {
                            Ok(req) => Ok(req),
                            Err(error) => Err(format!("{}", error)),
                        }
                    }
                    HttpMethods::PUT => {
                        let req = req.method("PUT").uri(url).body(Some(body));
                        match req {
                            Ok(req) => Ok(req),
                            Err(error) => Err(format!("{}", error)),
                        }
                    }
                    HttpMethods::POST => {
                        let req = req.method("POST").uri(url).body(Some(body));
                        match req {
                            Ok(req) => Ok(req),
                            Err(error) => Err(format!("{}", error)),
                        }
                    }
                    HttpMethods::DELETE => {
                        let req = req.method("DELETE").uri(url).body(Some(body));
                        match req {
                            Ok(req) => Ok(req),
                            Err(error) => Err(format!("{}", error)),
                        }
                    }
                },
                None => Err("Falied to create request".to_string()),
            },
        }
    }

    fn invoke_request(&self, request: Self::Output) -> Result<Value, String> {
        match wasi_request(request) {
            Ok(mut response) => match response.status_code {
                StatusCode::OK => match response.body_read_all() {
                    Ok(response) => match String::from_utf8(response) {
                        Ok(response) => {
                            let response_to_value: Result<Value, Error> =
                                serde_json::from_str(&response);
                            match response_to_value {
                                Ok(value) => Ok(value),
                                Err(error) => Err(error.to_string()),
                            }
                        }
                        Err(error) => Err(error.to_string()),
                    },
                    Err(error) => Err(format!("{}", error)),
                },
                _ => {
                    let code = response.status_code;
                    let error = response.body_read_all().unwrap();
                    let s = match std::str::from_utf8(&error) {
                        Ok(v) => v,
                        Err(error) => return Err(format!("{}", error)),
                    };

                    Err(whisk_errors(code, s.to_string()))
                }
            },
            Err(error) => Err(error.to_string()),
        }
    }
}
