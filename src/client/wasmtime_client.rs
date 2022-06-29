use bytes::Bytes;
use crate::api::{HttpMethods, Service};
use http::{Request,HeaderMap, StatusCode};
use super::common::{OpenWhisk,whisk_errors};
use serde_json::Value;
use wasi_experimental_http::request as wasi_request;
 
#[derive(Debug, Default)]
pub struct WasmClient{
     headers: http::HeaderMap
}
impl OpenWhisk for WasmClient {
    type Output = WasmClient;

    fn new_whisk_client(insecure: Option<bool>) -> Self::Output {
        let mut header_map = HeaderMap::new();
        match insecure {
            Some(x) => match x {
                true => header_map.insert("upgrade-insecure-requests", "true".parse().unwrap()),
                false => header_map.insert("upgrade-insecure-requests", "false".parse().unwrap())
            },
            None => todo!(),
        };
        return WasmClient{
            headers : header_map
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
        match user_auth {
            Some(auth) => {
                let user = auth.0;
                let pass = auth.1;
                let mut req =  http::request::Builder::new().header("authorization", http::HeaderValue::from_str(&("Basic ".to_owned() + user + pass)).unwrap());
                let _ = req.headers_mut().insert(&mut self.headers.clone());
                let body = Bytes::from(serde_json::to_vec(&body).unwrap());
                match method {
                    Some(http_methods) => match http_methods {
                        HttpMethods::GET =>Ok(req.method("GET").uri(url).body(Some(body)).unwrap()),
                        HttpMethods::PUT => Ok(req.method("PUT").uri(url).body(Some(body)).unwrap()),
                        HttpMethods::POST =>  Ok(req.method("POST").uri(url).body(Some(body)).unwrap()),
                        HttpMethods::DELETE =>   Ok(req.method("DELETE").uri(url).body(Some(body)).unwrap()),
                    },
                    None => todo!(),
                }
            }
            None => todo!(),
        }
    }

    fn invoke_request(&self, request:  Self::Output) -> Result<Value, String> {
       
         match wasi_request(request){
            Ok(mut response) => match response.status_code{
                StatusCode::OK => Ok(serde_json::Value::from(response.body_read_all().unwrap())),
                _ => {
                    let code = response.status_code;
                    let error = response.body_read_all().unwrap();

                    let s = match std::str::from_utf8(&error) {
                        Ok(v) => v,
                        Err(e) => todo!()
                    };

                    Err(whisk_errors(code, s.to_string()))
                }
               

            },
            Err(error) => Err(format!("{}", error)),
        }
        
    }
}
