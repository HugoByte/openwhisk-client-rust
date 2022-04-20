use serde_json::Value;

use super::HttpMethods;

pub trait Service {
    type Output;
    fn new_request(&self, method: HttpMethods, url: &str, user_auth: Option<(&str,&str)>,body: Option<Value>) -> Result<Self::Output,String>;
    fn invoke_request(&self, request: Self::Output) -> Result<Value, String>;
}
