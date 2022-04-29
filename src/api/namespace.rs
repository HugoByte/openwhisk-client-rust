use derive_new::new;

use crate::{client::Context} ;
use super::{HttpMethods, Service, NAMESPACE_ENDPOINT};

#[derive(new, Debug, Clone)]
pub struct NamespaceService<T> {
    client: T,
    context: Context,
}

impl<T> NamespaceService<T>
where
    T: Service,
{
    pub fn list(&self) -> Result<Vec<String>, String> {
        let url = format!("{}/api/v1/{}/", self.context.host(), NAMESPACE_ENDPOINT);

        let user_auth = self.context.auth();
        let user = user_auth.0;
        let pass = user_auth.1;

        let request = self
            .client
            .new_request(HttpMethods::GET, url.as_str(), Some((user, pass)), None)
            .unwrap();

        match self.client.invoke_request(request) {
            Ok(x) => match serde_json::from_value(x) {
                Ok(actions) => Ok(actions),
                Err(err) => Err(format!("Failed to deserailize {}", err)),
            },
            Err(x) => Err(format!("Failed to fetch the list of namespaces {}", x)),
        }
    }
}
