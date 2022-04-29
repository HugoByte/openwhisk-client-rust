use crate::api::{ActionService, NamespaceService, RuleService, TriggerService};
use super::{
    common::{Context, WskProperties},
    OpenWhisk,
};

#[derive(Debug, Clone)]
pub struct OpenwhiskClient<T> {
    pub context: Context,
    pub client: T,
    actions: ActionService<T>,
    triggers: TriggerService<T>,
    rules: RuleService<T>,
    namespaces: NamespaceService<T>,
}

impl<T: Clone> OpenwhiskClient<T>
where
    T: OpenWhisk + OpenWhisk<Output = T>,
{
    pub fn new(config: Option<&WskProperties>) -> Self {
        let context = Context::new(config);
        let client = T::new_whisk_client(Some(context.is_secure()));
        let actions = ActionService::new(client.clone(), context.clone());
        let triggers = TriggerService::new(client.clone(), context.clone());
        let rules = RuleService::new(client.clone(), context.clone());
        let namespaces = NamespaceService::new(client.clone(), context.clone());
        Self {
            client,
            context,
            actions,
            triggers,
            rules,
            namespaces,
        }
    }

    pub fn actions(&self) -> &ActionService<T> {
        &self.actions
    }

    pub fn triggers(&self) -> &TriggerService<T> {
        &self.triggers
    }

    pub fn rules(&self) -> &RuleService<T> {
        &self.rules
    }

    pub fn namespaces(&self) -> &NamespaceService<T> {
        &self.namespaces
    }
}
