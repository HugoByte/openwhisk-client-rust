use openwhisk_rust::{KeyValue, NativeClient, OpenwhiskClient, Trigger, WskProperties};

pub mod helper;
use crate::helper::{delete, get, put};

#[async_std::test]
async fn test_list_triggers_native_client() {
    let server = get().await;
    let wsk_properties = WskProperties::new(
        "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
        server.uri(),
          true, 
         "guest".to_string(), 
    );

    let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

    let triggers = serde_json::to_value(client.triggers().list().unwrap()).unwrap();
    let expected: Vec<Trigger> = serde_json::from_value(triggers).unwrap();
    assert_eq!(expected[0].name, "trigger".to_string());
}

#[async_std::test]
async fn test_create_trigger_native_client() {
    let server = put(None).await;
    let wsk_properties = WskProperties::new(
        "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
        server.uri(),
          true, 
         "guest".to_string(), 
    );

    let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

    let trigger = Trigger {
        name: "trigger".to_string(),

        annotations: vec![KeyValue {
            key: "fedd".to_string(),
            value: serde_json::json!("feed"),
        }],
        parameters: vec![KeyValue {
            key: "fedd".to_string(),
            value: serde_json::json!("feed"),
        }],

        version: "0.0.1".to_string(),
        ..Default::default()
    };

    let result = client.triggers().insert(&trigger, true).unwrap();

    let triggers = serde_json::to_value(result).unwrap();
    let expected: String = serde_json::to_string(&triggers).unwrap();
    assert!(expected.contains("trigger"));
}

#[async_std::test]
async fn test_delete_trigger_native_client() {
    let server = delete().await;
    let wsk_properties = WskProperties::new(
        "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
         server.uri(), 
          true, 
         "guest".to_string(), 
    );

    let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

    client.triggers().delete("trigger").unwrap();
    let triggers = serde_json::to_value(client.triggers().list().unwrap()).unwrap();
    let expected: Vec<Trigger> = serde_json::from_value(triggers).unwrap();

    assert!(expected.is_empty())
}
