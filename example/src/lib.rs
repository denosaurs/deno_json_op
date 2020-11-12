use deno_core::error::AnyError;

use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;

use deno_core::futures::FutureExt;

use deno_core::serde_json::Value;
use deno_core::serde_json::json;

use deno_json_op::json_op;

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
  interface.register_op("op_sync", op_sync);
}

#[json_op]
fn op_sync(val: Value, _zero_copy: &mut [ZeroCopyBuf]) -> Result<Value, AnyError> {
  Ok(json!({
    "hello": val
  }))
}

// #[json_op]
// async fn op_sync(val: Value, _zero_copy: &mut [ZeroCopyBuf]) -> Result<Value, AnyError> {
//   Ok(json!({
//     "hello": val
//   }))
// }
