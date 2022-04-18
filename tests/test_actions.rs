use openwhisk_rust::{WskProperties, NativeClient};

#[test]
fn test_list_actions_native_client() {

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

   let actions =  client.actions().list().unwrap();

   println!("{:?}",actions);
}