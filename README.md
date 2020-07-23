# secret-value

is a minimalistic crate for ensuring that no secret value was displayed or logged.

**Security notice:** this crate doesn't provide any kind of protection from direct memory access!

## Usage

Simply wrap up your type in `Secret` and that's it. You may access inner type via `Secret::inner` method.

## Serde support

This library supports derive of serde traits but it's disabled by default.
Use `features = ["serde"]` to enable it. By default `Serialize` does not
leak the inner value, a reversible serialization might be settled with
`#[serde(serialize_with = "insecure_serialize")]` field attribute.
