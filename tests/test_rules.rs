use openwhisk_rust::{ OpenwhiskClient, WskProperties,NativeClient, Trigger, Action, Rule};
use serde_json::json;

#[test]
fn test_list_rules_native_client() {
    let wsk_properties = WskProperties::new(
         "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
         "https://65.20.70.146:31001".to_string(), 
          true, 
         "guest".to_string(), 
    );

    let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

    let rules = serde_json::to_value(client.rules().list().unwrap()).unwrap();
    let expected: String = serde_json::to_string(&rules).unwrap();

 

    assert!(expected.contains(""));
}

#[test]
fn test_create_rule_native_clients() {
    let wsk_properties = WskProperties::new(
         "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
         "https://65.20.70.146:31001".to_string(), 
          true, 
         "guest".to_string(), 
    );

    let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

    let rule = Rule  { name: "sample_rule4".to_string(), trigger: "trigger1".to_string(), action: "cartype".to_string() };
    let rule = serde_json::to_value(client.rules().insert(&rule, true).unwrap()).unwrap();

    
    let expected: String = serde_json::to_string(&rule).unwrap();

    assert!(expected.contains("sample_rule4"));
}

#[test]
fn test_get_rule_property_native_client() {
    let wsk_properties = WskProperties::new(
         "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
         "https://65.20.70.146:31001".to_string(), 
          true, 
         "guest".to_string(), 
    );

    let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

    let rule = serde_json::to_value(client.rules().get("sample_rule2").unwrap()).unwrap();
   
    let expected: String = serde_json::to_string(&rule).unwrap();

    assert!(expected.contains("sample_rule2"));
}
