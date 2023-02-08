use openwhisk_rust::{
     NativeClient, OpenwhiskClient, Rule, RuleResponse, WskProperties,
};
pub mod helper;
use crate::helper::{get, put};

#[async_std::test]
async fn test_list_rules_native_client() {
    let server = get().await;
    let wsk_properties = WskProperties::new(
         "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
         server.uri(), 
          true, 
         "guest".to_string(), 
    );

    let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

    let rules = serde_json::to_value(client.rules().list().unwrap()).unwrap();

    let expected = RuleResponse {
        name: "rule1".to_string(),
        namespace: "guest".to_string(),
        ..Default::default()
    };

    let expected = serde_json::to_value(vec![expected]).unwrap();

    assert_eq!(rules, expected)
}

#[async_std::test]
async fn test_create_rule_native_clients() {
    let server = put(None).await;
    let wsk_properties = WskProperties::new(
         "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
         server.uri(), 
          true, 
         "guest".to_string(), 
    );

    let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

    let rule = Rule {
        name: "rule1".to_string(),
        trigger: "trigger1".to_string(),
        action: "cartype".to_string(),
    };
    let rule = serde_json::to_value(client.rules().insert(&rule, true).unwrap()).unwrap();

    let expected: RuleResponse = serde_json::from_value(rule).unwrap();

    assert_eq!(expected.namespace, "guest".to_string());
}

#[async_std::test]
async fn test_get_rule_property_native_client() {
    let server = get().await;
    let wsk_properties = WskProperties::new(
         "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
         server.uri(),
          true, 
         "guest".to_string(), 
    );

    let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

    let rule = serde_json::to_value(client.rules().get("rule1").unwrap()).unwrap();

    let expected: RuleResponse = serde_json::from_value(rule).unwrap();

    assert_ne!(expected.version, "1".to_string())
}
