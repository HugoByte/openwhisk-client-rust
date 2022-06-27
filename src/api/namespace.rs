use derive_new::new;

use super::{HttpMethods, Service, NAMESPACE_ENDPOINT};
use crate::client::Context;

/// Representation of Namespace Service
#[derive(new, Default, Debug, Clone)]
pub struct NamespaceService<T> {
    /// A Namespace service must have a client to handle http request
    client: T,
    /// A Namespace service uses the context which sets openwhisk properties
    context: Context,
}

impl<T> NamespaceService<T>
where
    T: Service,
{
    /// The list function gets inputs from the struct and returns the list of namespaces available
    pub fn list(&self) -> Result<Vec<String>, String> {
        let url = format!("{}/api/v1/{}/", self.context.host(), NAMESPACE_ENDPOINT);

        let user_auth = self.context.auth();
        let user = user_auth.0;
        let pass = user_auth.1;

        let request = match self.client.new_request(
            Some(HttpMethods::GET),
            url.as_str(),
            Some((user, pass)),
            None,
        ) {
            Ok(request) => request,
            Err(error) => return Err(format!("{}", error)),
        };

        match self.client.invoke_request(request) {
            Ok(x) => match serde_json::from_value(x) {
                Ok(namespaces) => Ok(namespaces),
                Err(err) => Err(format!("Failed to deserailize {}", err)),
            },
            Err(x) => Err(format!("Failed to fetch the list of available namespaces {}", x)),
        }
    }
}
