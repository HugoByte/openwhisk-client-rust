use derive_new::new;

#[derive(new, Debug, Clone)]
pub struct NamespaceService<T>{
    client : T,
}