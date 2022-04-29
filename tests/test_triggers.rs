use openwhisk_rust::{Action, Exec, KeyValue, OpenwhiskClient, WskProperties,NativeClient, Trigger};


#[test]
fn test_list_triggers_native_client() {
    let wsk_properties = WskProperties::new(
        "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
         "https://localhost:31001".to_string(), 
         "v1".to_string(), 
         true,
         "guest".to_string(), 
         true,
         false
    );

    let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

    

    let triggers = serde_json::to_value(client.triggers().list().unwrap()).unwrap();
    let expected: String = serde_json::to_string(&triggers).unwrap();
    assert!(expected.contains("trigger"));
}

#[test]
fn test_create_trigger_native_client() {
    let wsk_properties = WskProperties::new(
        "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
         "https://localhost:31001".to_string(), 
         "v1".to_string(), 
         true,
         "guest".to_string(), 
         true,
         false
    );

    let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

    let trigger = Trigger {
        
        name: "trigger1".to_string(),
        
        annotations: vec![KeyValue {
            key: "fedd".to_string(),
            value: serde_json::json!("feed"),
        }],
        parameters: vec![KeyValue {
            key: "fedd".to_string(),
            value: serde_json::json!("feed"),
        }],
        ..Default::default()
        
    };

    let result = client.triggers().insert(&trigger, true).unwrap();

    let triggers = serde_json::to_value(result).unwrap();
    let expected: String = serde_json::to_string(&triggers).unwrap();
    assert!(expected.contains("trigger"));
}


#[test]
fn test_delete_trigger_native_client() {
    let wsk_properties = WskProperties::new(
        "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
         "https://localhost:31001".to_string(), 
         "v1".to_string(), 
         true,
         "guest".to_string(), 
         true,
         false
    );

    let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

    

    let triggers = serde_json::to_value(client.triggers().delete("trigger1").unwrap()).unwrap();
    let expected: String = serde_json::to_string(&triggers).unwrap();
    assert!(expected.contains("trigger"));
}