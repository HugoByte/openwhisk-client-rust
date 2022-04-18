use derive_new::new;

#[derive(new, Debug, Clone)]
pub struct RuleService<T>{
    client : T,
}

