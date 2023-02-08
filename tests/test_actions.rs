pub mod helper;

use openwhisk_rust::{
    Action, ActionList, Exec, KeyValue, NativeClient, OpenwhiskClient, WskProperties,
};

use crate::helper::{delete, get, put};

#[async_std::test]
async fn test_list_actions_native_client() {
    let server = get().await;

    let wsk_properties = WskProperties::new(
        "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
        server.uri(),
         true,
         "guest".to_string(), 
    );

    let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

    let result = serde_json::to_value(client.actions().list().unwrap()).unwrap();

    let actual = serde_json::to_value(vec![ActionList {
        name: "cars".to_string(),
        namespace: "guest".to_string(),
    }])
    .unwrap();

    assert_eq!(actual, result)
}

#[async_std::test]
async fn test_get_action_property_native_client() {
    let server = get().await;

    let wsk_properties = WskProperties::new(
        "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
         server.uri(), 
          true, 
         "guest".to_string(), 
    );

    let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));
    let actions = serde_json::to_value(client.actions().get("cars", false).unwrap()).unwrap();

    let expected: String = serde_json::to_string(&actions).unwrap();
    assert!(expected.contains("cars"));
}

#[async_std::test]
async fn test_delete_action_native_client() {
    let server = delete().await;
    let wsk_properties = WskProperties::new(
        "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
         server.uri(), 
         true,
         "guest".to_string(), 
    );

    let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

    client.actions().delete("cars").unwrap();

    let actions = serde_json::to_value(client.actions().list().unwrap()).unwrap();
    let expected: String = serde_json::to_string(&actions).unwrap();
    assert!(!expected.contains("cars"));
}

#[async_std::test]
async fn test_create_action() {
    let bas64_data = base64::encode("code");

    let action = Action {
        namespace: "guest".to_string(),
        name: "cars".to_string(),
        version: "0.0.1".to_string(),
        limits: Default::default(),
        exec: Exec {
            kind: "rust:1.34".to_string(),
            code: bas64_data,
            image: "openwhisk/action-rust-v1.34".to_string(),
            init: "".to_string(),
            main: "".to_string(),
            components: vec![],
            binary: true,
        },
        error: "".to_string(),
        publish: true,
        updated: 0,
        annotations: vec![KeyValue {
            key: "feed".to_string(),
            value: serde_json::json!({}),
        }],
    };

    let server = put(Some(action.clone())).await;

    let wsk_properties = WskProperties::new(
        "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
         server.uri(), 
         true,
         "guest".to_string(), 
    );

    let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

    client.actions().insert(&action, true).unwrap();

    let actions = serde_json::to_value(client.actions().list().unwrap()).unwrap();
    let expected: String = serde_json::to_string(&actions).unwrap();

    assert!(expected.contains("cars"));
}
