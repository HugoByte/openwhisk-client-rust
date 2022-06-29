use super::{
    common::{Context, WskProperties},
    OpenWhisk,
};
use crate::api::{ActionService, NamespaceService, RuleService, TriggerService};

/// Representation of Openwhisk Client
#[derive(Debug, Default, Clone)]
pub struct OpenwhiskClient<T> {
    /// Openwhisk client context (Properties set for the openwhisk)
    pub context: Context,
    /// Client represents the http client (OpenwhiskClient takes generic type for http client)
    pub client: T,
    /// Action endpoint to access Openwhisk API
    actions: ActionService<T>,
    /// triggers endpoint to access Openwhisk API
    triggers: TriggerService<T>,
    /// rules endpoint to access Openwhisk API
    rules: RuleService<T>,
    /// namespace endpoint to access Openwhisk API
    namespaces: NamespaceService<T>,
}

impl<T: Clone> OpenwhiskClient<T>
where
    T: OpenWhisk + OpenWhisk<Output = T>,
{
    /// To set Openwhisk config for library to interact with Openwhisk API's
    ///
    /// # Arguments
    /// * `config` - Can be None or Openwhisk Properties defined by User
    ///    when None is supplied poperties are set by environment
    ///
    /// # Example
    /// ```
    /// use openwhisk_rust::{NativeClient, OpenwhiskClient, WskProperties};
    /// // setting openwhisk props with user Input
    /// let new_wsk_props = WskProperties::new(
    ///         "your:auth_token".to_string(),
    ///         "host".to_string(),
    ///         true,
    ///         "namespace".to_string()
    ///  );
    ///
    /// // creating new client from using the propety
    ///
    /// let client = OpenwhiskClient::<NativeClient>::new(Some(&new_wsk_props));
    /// // use initilalised client to interact with openwhisk API
    ///
    /// ```
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

    /// To Access action endpoints from the Openwhisk Client using this method
    ///
    /// Returns ActionService
    ///
    /// This can be used to call underlying action service methods
    ///
    /// * `list`    - Lists all the actions in the namesapce
    ///
    /// # Example
    /// ```
    /// use openwhisk_rust::{NativeClient, OpenwhiskClient, WskProperties};
    /// // setting openwhisk props with user Input
    /// let new_wsk_props = WskProperties::new(
    ///         "your:auth_token".to_string(),
    ///         "host".to_string(),
    ///         true,
    ///         "namespace".to_string()
    ///  );
    ///
    /// // creating new client from using the propety
    ///
    /// let client = OpenwhiskClient::<NativeClient>::new(Some(&new_wsk_props));
    /// // use initilalised client to interact with openwhisk API
    ///
    /// // Lists the actions deployed in the openwhisk
    /// let actions = client.actions().list().unwrap();
    /// ```
    ///
    /// * `get`     - Get the action property based on the action name provided as paramter
    ///
    /// # Example
    ///
    /// ```
    /// use openwhisk_rust::{NativeClient, OpenwhiskClient, WskProperties};
    /// // setting openwhisk props with user Input
    /// let new_wsk_props = WskProperties::new(
    ///         "your:auth_token".to_string(),
    ///         "host".to_string(),
    ///         true,
    ///         "namespace".to_string()
    ///  );
    ///
    /// // creating new client from using the propety
    ///
    /// let client = OpenwhiskClient::<NativeClient>::new(Some(&new_wsk_props));
    /// // use initilalised client to interact with openwhisk API
    ///
    /// // get the action properties which is deployed in the openwhisk
    /// let action_property = client.actions().get("action_name").unwrap();
    /// ```
    ///  
    /// * `delete`  - Delete action based on the action name
    ///
    /// # Example
    ///
    /// ```
    /// use openwhisk_rust::{NativeClient, OpenwhiskClient, WskProperties};
    /// // setting openwhisk props with user Input
    /// let new_wsk_props = WskProperties::new(
    ///         "your:auth_token".to_string(),
    ///         "host".to_string(),
    ///         true,
    ///         "namespace".to_string()
    ///  );
    ///
    /// // creating new client from using the propety
    ///
    /// let client = OpenwhiskClient::<NativeClient>::new(Some(&new_wsk_props));
    /// // use initilalised client to interact with openwhisk API
    ///
    /// // deletes the action  which is deployed in the openwhisk
    /// let action = client.actions().delete("action_name").unwrap();
    ///
    /// ```
    ///
    /// * `insert`  - Insert action and returns new action created
    /// # Example
    ///
    /// ```
    /// use openwhisk_rust::{NativeClient, OpenwhiskClient, WskProperties};
    /// // setting openwhisk props with user Input
    /// let new_wsk_props = WskProperties::new(
    ///         "your:auth_token".to_string(),
    ///         "host".to_string(),
    ///         true,
    ///         "namespace".to_string()
    ///  );
    ///
    /// // creating new client from using the propety
    ///
    /// let client = OpenwhiskClient::<NativeClient>::new(Some(&new_wsk_props));
    /// // use initilalised client to interact with openwhisk API
    ///
    /// // insert the action and deploys in the openwhisk
    /// let action = client.actions().insert(action,true,true).unwrap();
    ///
    /// ```
    /// * `invoke`  - Invoke action based on the action name and payload for the actions
    ///  
    /// # Example
    ///
    /// ```
    /// use openwhisk_rust::{NativeClient, OpenwhiskClient, WskProperties};
    /// // setting openwhisk props with user Input
    /// let new_wsk_props = WskProperties::new(
    ///         "your:auth_token".to_string(),
    ///         "host".to_string(),
    ///         true,
    ///         "namespace".to_string()
    ///  );
    ///
    /// // creating new client from using the propety
    ///
    /// let client = OpenwhiskClient::<NativeClient>::new(Some(&new_wsk_props));
    /// // use initilalised client to interact with openwhisk API
    ///
    /// // invoke the action deployed in the openwhisk
    /// let action_result = client.actions().invoke("action_name","action_payload",true,true).unwrap();
    ///
    /// ```
    pub fn actions(&self) -> &ActionService<T> {
        &self.actions
    }

    /// To Access trigger endpoints from the Openwhisk Client using this method
    ///
    /// Returns TriggerService
    /// This can be used to call underlying action service methods
    ///
    /// * `list`    - Lists all the triggers in the namesapce
    ///
    /// # Example
    /// ```
    /// use openwhisk_rust::{NativeClient, OpenwhiskClient, WskProperties};
    /// // setting openwhisk props with user Input
    /// let new_wsk_props = WskProperties::new(
    ///         "your:auth_token".to_string(),
    ///         "host".to_string(),
    ///         true,
    ///         "namespace".to_string()
    ///  );
    ///
    /// // creating new client from using the propety
    ///
    /// let client = OpenwhiskClient::<NativeClient>::new(Some(&new_wsk_props));
    /// // use initilalised client to interact with openwhisk API
    ///
    /// // Lists the triggers deployed in the openwhisk
    /// let triggers = client.triggers().list().unwrap();
    /// ```
    ///
    /// * `get`     - Get the trigger property based on the trigger name provided as paramter
    ///
    /// # Example
    ///
    /// ```
    /// use openwhisk_rust::{NativeClient, OpenwhiskClient, WskProperties};
    /// // setting openwhisk props with user Input
    /// let new_wsk_props = WskProperties::new(
    ///         "your:auth_token".to_string(),
    ///         "host".to_string(),
    ///         true,
    ///         "namespace".to_string()
    ///  );
    ///
    /// // creating new client from using the propety
    ///
    /// let client = OpenwhiskClient::<NativeClient>::new(Some(&new_wsk_props));
    /// // use initilalised client to interact with openwhisk API
    ///
    /// // get the trigger properties which is deployed in the openwhisk
    /// let trigger_property = client.triggers().get("trigger_name").unwrap();
    /// ```
    ///  
    /// * `delete`  - Delete trigger based on the trigger name
    ///
    /// # Example
    ///
    /// ```
    /// use openwhisk_rust::{NativeClient, OpenwhiskClient, WskProperties};
    /// // setting openwhisk props with user Input
    /// let new_wsk_props = WskProperties::new(
    ///         "your:auth_token".to_string(),
    ///         "host".to_string(),
    ///         true,
    ///         "namespace".to_string()
    ///  );
    ///
    /// // creating new client from using the propety
    ///
    /// let client = OpenwhiskClient::<NativeClient>::new(Some(&new_wsk_props));
    /// // use initilalised client to interact with openwhisk API
    ///
    /// // deletes the trigger  which is deployed in the openwhisk
    /// let trigger = client.triggers().delete("trigger_name").unwrap();
    ///
    /// ```
    ///
    /// * `insert`  - Insert trigger and returns new trigger created
    /// # Example
    ///
    /// ```
    /// use openwhisk_rust::{NativeClient, OpenwhiskClient, WskProperties};
    /// // setting openwhisk props with user Input
    /// let new_wsk_props = WskProperties::new(
    ///         "your:auth_token".to_string(),
    ///         "host".to_string(),
    ///         true,
    ///         "namespace".to_string()
    ///  );
    ///
    /// // creating new client from using the propety
    ///
    /// let client = OpenwhiskClient::<NativeClient>::new(Some(&new_wsk_props));
    /// // use initilalised client to interact with openwhisk API
    ///
    /// // insert the trigger and deploys in the openwhisk
    /// let trigger = client.triggers().insert(trigger,true).unwrap();
    ///
    /// ```
    ///
    /// * `fire`  - Fires a trigger to an action
    ///  
    /// # Example
    ///
    /// ```
    /// use openwhisk_rust::{NativeClient, OpenwhiskClient, WskProperties};
    /// // setting openwhisk props with user Input
    /// let new_wsk_props = WskProperties::new(
    ///         "your:auth_token".to_string(),
    ///         "host".to_string(),
    ///         true,
    ///         "namespace".to_string()
    ///  );
    ///
    /// // creating new client from using the propety
    ///
    /// let client = OpenwhiskClient::<NativeClient>::new(Some(&new_wsk_props));
    /// // use initilalised client to interact with openwhisk API
    ///
    /// // Fires a trigger to an action
    /// let trigger = client.actions().fire("trigger_name",value).unwrap();
    ///
    /// ```

    pub fn triggers(&self) -> &TriggerService<T> {
        &self.triggers
    }

    /// To Access action endpoints from the Openwhisk Client using this method
    ///
    /// Returns RuleService
    /// This can be used to call underlying action service methods
    ///
    /// * `list`    - Lists all the rules in the namesapce
    ///
    /// # Example
    /// ```
    /// use openwhisk_rust::{NativeClient, OpenwhiskClient, WskProperties};
    /// // setting openwhisk props with user Input
    /// let new_wsk_props = WskProperties::new(
    ///         "your:auth_token".to_string(),
    ///         "host".to_string(),
    ///         true,
    ///         "namespace".to_string()
    ///  );
    ///
    /// // creating new client from using the propety
    ///
    /// let client = OpenwhiskClient::<NativeClient>::new(Some(&new_wsk_props));
    /// // use initilalised client to interact with openwhisk API
    ///
    /// // Lists the rules deployed in the openwhisk
    /// let rules = client.rules().list().unwrap();
    /// ```
    ///
    /// * `get`     - Get the rule property based on the rule name provided as paramter
    ///
    /// # Example
    ///
    /// ```
    /// use openwhisk_rust::{NativeClient, OpenwhiskClient, WskProperties};
    /// // setting openwhisk props with user Input
    /// let new_wsk_props = WskProperties::new(
    ///         "your:auth_token".to_string(),
    ///         "host".to_string(),
    ///         true,
    ///         "namespace".to_string()
    ///  );
    ///
    /// // creating new client from using the propety
    ///
    /// let client = OpenwhiskClient::<NativeClient>::new(Some(&new_wsk_props));
    /// // use initilalised client to interact with openwhisk API
    ///
    /// // get the rule properties which is deployed in the openwhisk
    /// let rule_property = client.rules().get("rule_name").unwrap();
    /// ```
    ///  
    /// * `delete`  - Delete rule based on the rule name
    ///
    /// # Example
    ///
    /// ```
    /// use openwhisk_rust::{NativeClient, OpenwhiskClient, WskProperties};
    /// // setting openwhisk props with user Input
    /// let new_wsk_props = WskProperties::new(
    ///         "your:auth_token".to_string(),
    ///         "host".to_string(),
    ///         true,
    ///         "namespace".to_string()
    ///  );
    ///
    /// // creating new client from using the propety
    ///
    /// let client = OpenwhiskClient::<NativeClient>::new(Some(&new_wsk_props));
    /// // use initilalised client to interact with openwhisk API
    ///
    /// // deletes the rule  which is deployed in the openwhisk
    /// let rule = client.rules().delete("rule_name").unwrap();
    ///
    /// ```
    ///
    /// * `setstate`  - Sets the state of the rule
    ///  
    /// # Example
    ///
    /// ```
    /// use openwhisk_rust::{NativeClient, OpenwhiskClient, WskProperties};
    /// // setting openwhisk props with user Input
    /// let new_wsk_props = WskProperties::new(
    ///         "your:auth_token".to_string(),
    ///         "host".to_string(),
    ///         true,
    ///         "namespace".to_string()
    ///  );
    ///
    /// // creating new client from using the propety
    ///
    /// let client = OpenwhiskClient::<NativeClient>::new(Some(&new_wsk_props));
    /// // use initilalised client to interact with openwhisk API
    ///
    /// // set state for the rule deployed in the openwhisk
    /// let rule = client.rules().setstate("rule_name","state").unwrap();
    ///
    /// ```
    pub fn rules(&self) -> &RuleService<T> {
        &self.rules
    }

    /// To Access namespace endpoints from the Openwhisk Client using this method
    ///
    /// Returns NamespaceService
    ///
    /// This can be used to call underlying action service methods
    ///
    /// * `list`    - Lists all the actions in the namesapce
    ///
    /// # Example
    /// ```
    /// use openwhisk_rust::{NativeClient, OpenwhiskClient, WskProperties};
    /// // setting openwhisk props with user Input
    /// let new_wsk_props = WskProperties::new(
    ///         "your:auth_token".to_string(),
    ///         "host".to_string(),
    ///         true,
    ///         "namespace".to_string()
    ///  );
    ///
    /// // creating new client from using the propety
    ///
    /// let client = OpenwhiskClient::<NativeClient>::new(Some(&new_wsk_props));
    /// // use initilalised client to interact with openwhisk API
    ///
    /// // Lists the namespaces available in the openwhisk
    /// let namespaces = client.namespaces().list().unwrap();
    /// ```
    ///
    pub fn namespaces(&self) -> &NamespaceService<T> {
        &self.namespaces
    }
}
