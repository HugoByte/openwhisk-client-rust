use openwhisk_client_rust::{NativeClient, OpenwhiskClient, WskProperties};
pub mod helper;
use crate::helper::get;

#[async_std::test]
async fn test_list_namespaces_native_client() {
    let server = get().await;
    let wsk_properties = WskProperties::new(
        "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP".to_string(),
        server.uri(),
        "guest".to_string(),
    ).set_bypass_cerificate_check(true);

    let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

    let namespaces = client.namespaces().list().unwrap();

    assert_eq!(vec!["guest"], namespaces);
}
