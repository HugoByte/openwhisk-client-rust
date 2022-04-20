use openwhisk_rust::{WskProperties,NativeClient,Trigger, KeyValue};


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
    
    let client = NativeClient::new(Some(&wsk_properties));

   let triggers =  client.triggers().list().unwrap();

   println!("{:?}",triggers);
}

#[test]
fn test_create_trigger_native_client(){
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

    let trigger = Trigger{
        namespace: "guest".to_string(),
        name: "shanith".to_string(),
        version: "2.0".to_string(),
        publish: true,
        updated: 5,
        annotations: vec![KeyValue{ key: "fedd".to_string(), value: serde_json::from_str("feed").unwrap() }],
        parameters: vec![KeyValue{ key: "fedd".to_string(), value: serde_json::from_str("feed").unwrap() }],
        limits: Default::default(),
    };

    

    let result = client.triggers().insert(&trigger, true).unwrap();

    println!("{:?}",result);

}