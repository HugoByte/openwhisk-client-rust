# Openwhisk Client Rust

The OpenWhisk Rust Client is a Rust library for interacting with the OpenWhisk API. It enables developers to easily access OpenWhisk features from their Rust applications.

## Prerequisites

Ensure the latest version of Rust is installed and configured on your system.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
openwhisk-client-rust = { version = "0.1.6", default-features = false }
```
Then, run `cargo build` to download and compile the OpenWhisk Rust Client.

## Usage
To start using the OpenWhisk Rust Client, you need to configure the client with your OpenWhisk credentials and settings.

### Configuration

Create a `WskProperties` instance with your API host, auth token, namespace, and connection security preference:

```rust
let wsk_properties = WskProperties::new(
			"<Openwhisk_Auth_Token>",
			"<Openwhisk_API_Host>",
			"<Namespace>"
 );
```
- **Openwhisk_Auth_Token**: This is the authentication token used to authenticate requests to the OpenWhisk API. You can obtain this token from the OpenWhisk console by going to the "API Key" section under the "Namespace" tab. You should replace `<Openwhisk_Auth_Token>` in the code snippet with the actual token string.
- **Openwhisk_API_Host**: This is the hostname or IP address of the OpenWhisk API endpoint. You can find the endpoint URL in the OpenWhisk console by going to the "Endpoints" section under the "Namespace" tab. You should replace `<Openwhisk_API_Host>` in the code snippet with the actual endpoint URL.
- **Namespace**: This is the name of the OpenWhisk namespace that you want to interact with. You can find your namespace name in the OpenWhisk console under the "Namespace" tab. You should replace `<Namespace>` in the code snippet with the actual namespace name.

### Examples

Here are a few examples of how to use the OpenWhisk Rust Client:

- **Invoke a deployed Action:**

```rust
use openwhisk_client_rust::{NativeClient, OpenwhiskClient, WskProperties};

let wsk_properties = WskProperties::new(
			"<Openwhisk_Auth_Token>",
			"<Openwhisk_API_Host>",
			"<Namespace>"
	  );

let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

client.actions().invoke("action_name",serde_json::json!({"key":"value"}),true,true).unwrap();

```

- **Get a list of available triggers**

```rust
use openwhisk_client_rust::{NativeClient, OpenwhiskClient, WskProperties};

let wsk_properties = WskProperties::new(
			"<Openwhisk_Auth_Token>",
			"<Openwhisk_API_Host>",
			"<Namespace>"
	  );

let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

let triggers = client.triggers().list().unwrap();
```

- **Get a list of available rules**

```rust
use openwhisk_client_rust::{NativeClient, OpenwhiskClient, Rule, RuleResponse, WskProperties};

let wsk_properties = WskProperties::new(
			"<Openwhisk_Auth_Token>",
			"<Openwhisk_API_Host>",
			"<Namespace>"
	  );
	  
let client = OpenwhiskClient::<NativeClient>::new(Some(&wsk_properties));

let properties = client.rules().list().unwrap();
```

## Testing
Run the test suite using

```bash
cargo test
```

## Contributions

We welcome contributions to improve the library, add new features, and fix bugs. To contribute, follow these steps:

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
