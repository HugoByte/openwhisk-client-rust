use openwhisk_rust::{NativeClient, Rule, Trigger, WskProperties};

#[test]
fn test_list_rules_native_client() {
    let wsk_properties = WskProperties::new(
        "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
         "https://localhost:31001".to_string(), 
         "v1".to_string(), 
         true,
         "guest".to_string(), 
         true,
         false
    );

    let client = NativeClient::new(Some(&wsk_properties));

    let actions = serde_json::to_value(client.rules().list().unwrap()).unwrap();
    let expected: String = serde_json::to_string(&actions).unwrap();

    assert!(expected.contains(""));
}

#[test]
fn test_create_rule_native_clients() {
    let wsk_properties = WskProperties::new(
        "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
         "https://localhost:31001".to_string(), 
         "v1".to_string(), 
         true,
         "guest".to_string(), 
         true,
         false
    );

    let client = NativeClient::new(Some(&wsk_properties));

    let rule = Rule {
        name: "sample_rule".to_string(),
        trigger: serde_json::to_value(Trigger {
            name: "sample_trigger".to_string(),
            ..Default::default()
        })
        .unwrap(),
        action: serde_json::to_value(Rule {
            name: "sample_rule".to_string(),
            ..Default::default()
        })
        .unwrap(),
        ..Default::default()
    };
    let rule = serde_json::to_value(client.rules().insert(&rule, true).unwrap()).unwrap();
    let expected: String = serde_json::to_string(&rule).unwrap();

    assert!(expected.contains("sample_rule"));
}

#[test]
fn test_get_rule_property_native_client() {
    let wsk_properties = WskProperties::new(
        "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
         "https://localhost:31001".to_string(), 
         "v1".to_string(), 
         true,
         "guest".to_string(), 
         true,
         false
    );

    let client = NativeClient::new(Some(&wsk_properties));

    let rule = serde_json::to_value(client.rules().get("sample_rule").unwrap()).unwrap();
    let expected: String = serde_json::to_string(&rule).unwrap();

    assert!(expected.contains("sample_trigger"));
}
