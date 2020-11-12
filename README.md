# deno_json_op

This is a crate for easing the building of deno plugins by providing a simple
macro which automatically converts the first element in the `ZeroCopy` to a
`serde_json::Value` and the `Result<_, _>` to a json object.

## Usage

```rust
use deno_core::error::AnyError;

use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;

use deno_core::serde_json::Value;
use deno_core::serde_json::json;

use deno_json_op::json_op;

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
  interface.register_op("test", op_test);
}

#[json_op]
fn op_test(_interface: &mut dyn Interface, val: Value, _zero_copy: &mut [ZeroCopyBuf]) -> Result<Value, AnyError> {
  Ok(json!({
    "hello": val
  }))
}
```

## Other

### Related

- [calcite](https://github.com/Srinivasa314/calcite) - similar project

### Contribution

Pull request, issues and feedback are very welcome. Code style is formatted with
`deno fmt` and commit messages are done following Conventional Commits spec.

### Licence

Copyright 2020-present, the denosaurs team. All rights reserved. MIT license.
