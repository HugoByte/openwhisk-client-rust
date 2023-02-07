use openwhisk_rust::{
    Action, ActionList, Exec, KeyValue, Limits, NativeClient, OpenwhiskClient, WskProperties,
};
use std::{fs, io::Read};
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

async fn serever() -> MockServer {
    let mock_server = MockServer::start().await;

    let action = Action {
        namespace: "guest".to_string(),
        name: "cars".to_string(),
        version: "".to_string(),
        limits: Limits {
            timeout: 1,
            memory: 2,
            logsize: 3,
            concurrency: 3,
        },
        exec: Exec {
            kind: "rust:1.34".to_string(),
            code: "bas64_data".to_string(),
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
    let r = vec![action.clone()];
    Mock::given(method("GET"))
        .and(path("/api/v1/namespaces/guest/actions"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(r),
        )
        // Mounting the mock on the mock server - it's now effective!
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v1/namespaces/guest/actions/cars"))
        .and(query_param("code", "false"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(action.clone()),
        )
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/api/v1/namespaces/guest/actions/cars"))
        .and(query_param("code", "false"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/json")
                .set_body_json(action.clone()),
        )
        .mount(&mock_server)
        .await;

    mock_server
}

#[async_std::test]
async fn test_list_actions_native_client() {
    let server = serever().await;

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
    let server = serever().await;

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
    let server = serever().await;
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

#[test]
fn test_create_action() {
    let wsk_properties = WskProperties::new(
        "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
         "https://65.20.70.146:31001".to_string(), 
         true,
         "guest".to_string(), 
    );

    let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

    let mut contents =
        fs::File::open("/home/soul/Downloads/actions/car_list.zip").expect("File not found");
    let mut file_content = Vec::new();
    contents.read_to_end(&mut file_content).unwrap();
    let bas64_data = base64::encode(file_content);

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

    client.actions().insert(&action, true).unwrap();

    let actions = serde_json::to_value(client.actions().list().unwrap()).unwrap();
    let expected: String = serde_json::to_string(&actions).unwrap();

    assert!(expected.contains("cars"));
}
