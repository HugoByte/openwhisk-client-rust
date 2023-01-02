# Openwhisk Client Rust

## Description

This project Openwhisk-client-Rust is a Rust client library to access the Openwhisk API.
This Project comes with Native and Wasm Support.

## Setup

### Prerequisites

This Project requies latest verion of rust to be installed and configured.

## Configuration

Configure your Openwhisk client by setting the whisk properties.

```
let wsk_properties = WskProperties::new(
			"<Openwhisk_Auth_Token>",
			"<Openwhisk_API_Hosy>",
			 <insecure(Bool: for secure or insecure connection )>,
			 "<Namespace>"
 );
```

## Usage

### Examples

- `Invoke a deployed Action`

```
 let wsk_properties = WskProperties::new(
			"<Openwhisk_Auth_Token>",
			"<Openwhisk_API_Hosy>",
			 <Bool(true/false)>,
			 "<Namespace>"
	  );
 let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

 client.actions().invoke("your action name",serde_json::json!({"key":"value"}),true,true).unwrap();

```

- ` Get list of available triggers`

```
 let wsk_properties = WskProperties::new(
			"<Openwhisk_Auth_Token>",
			"<Openwhisk_API_Hosy>",
			 <Bool(true/false)>,
			 "<Namespace>"
	  );
 let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

 let triggers = client.triggers().list().unwrap();
```

- ` Get the properties of the rule`

```
 let wsk_properties = WskProperties::new(
			"<Openwhisk_Auth_Token>",
			"<Openwhisk_API_Hosy>",
			 <Bool(true/false)>,
			 "<Namespace>"
	  );
 let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));
 let properties = client.rules().list().unwrap();

```

## Contributions

Contributions welcome particularly for enhancement of this library and also adding new functionality which helps in seamless interaction with Openwhisk Apis in rust applications.

### Follow these steps for contributing

- Fork the repository
- Clone your fork
- Create a new branch
- Make changes in local repo
- commit and push your changes to forked repo
- Begin and create a pull request
- Once pull request is review and accepted code changes will be merged to main branch
- You will get a notification email once the changes have been merged

## References

Special thanks to [Openwhisk-go-client](https://github.com/apache/openwhisk-client-go) for inspiring us to develop Openwhisk Client in rust

## License

Licensed under [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)
