use derive_new::new;

use crate::client::Context;

#[derive(new, Debug, Clone)]
pub struct NamespaceService<T>{
    client : T,
    context: Context
}