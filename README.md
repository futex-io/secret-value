# secret-value

is a minimalistic crate for ensuring that no secret value was displayed or logged.

**Security notice:** this crate doesn't provide any kind of protection from direct memory access!

## Usage

Simply wrap up your type in `Secret` and that's it. You may access inner type via `Secret::inner` method.

## Serde support

This library supports serde traits derive but it's not enabled by default. Use `features = ["serde"]` to enable.
