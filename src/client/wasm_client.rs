use super::common::{Context, WskProperties};
use crate::api::{ActionService, NamespaceService, RuleService, Service, TriggerService};
use derive_new::new;
use http::request;
use serde_json::Value;

#[derive(Debug, Clone)]
struct Client {}

#[derive(Debug, Clone)]
pub struct WasmClient {
    pub context: Context,
    client: Client,
    actions: ActionService<Client>,
    triggers: TriggerService<Client>,
    rules: RuleService<Client>,
    namespaces: NamespaceService<Client>,
}

impl Service for Client {
    type Output = request::Builder;

    fn new_request(&self, method: String, url: String, user_auth: Option<(String,String)>,body: Option<Value>) -> Result<Self::Output,String> {
        todo!()
    }

    fn invoke_request(&self, request: Self::Output) -> Result<Value, String> {
        todo!()
    }

    
}
