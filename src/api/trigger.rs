use derive_new::new;


#[derive(new, Debug, Clone)]
pub struct TriggerService<T>{
    client : T,
}