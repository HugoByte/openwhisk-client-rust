use openwhisk_rust::{NativeClient, WskProperties};

#[test]
fn test_list_namespaces_native_client() {
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

    let namespaces = client.namespaces().list().unwrap();

    assert_eq!(vec!["guest"], namespaces);
}