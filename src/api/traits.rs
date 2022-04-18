use serde_json::Value;

pub trait Service {
    type Output;
    fn new_request(&self, method: String, url: String, user_auth: Option<(String,String)>,body: Option<Value>) -> Result<Self::Output,String>;
    fn invoke_request(&self, request: Self::Output) -> Result<Value, String>;
}
