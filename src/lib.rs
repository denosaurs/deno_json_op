extern crate proc_macro;
use proc_macro::TokenStream;

use syn::parse_macro_input;
use syn::ItemFn;

use quote::format_ident;
use quote::quote;

#[proc_macro_attribute]
pub fn json_op(_args: TokenStream, item: TokenStream) -> TokenStream {
  let func = parse_macro_input!(item as ItemFn);

  let ItemFn {
    attrs,
    vis,
    sig,
    block,
  } = func;

  let is_async = sig.asyncness.is_some();
  let name = &sig.ident;

  let impl_name = format_ident!("__impl_{}", name);
  let mut impl_sig = sig.clone();
  impl_sig.ident = impl_name.clone();

  let impl_func = ItemFn {
    attrs,
    vis,
    sig: impl_sig,
    block,
  };

  let body = if is_async {
    // TODO
    // quote! {
    //   let fut = #impl_name(val, rest)
    //     .then(|res| async {
    //       let res = match res {
    //         Ok(val) => deno_core::serde_json::json!({ "ok": val }),
    //         Err(err) => deno_core::serde_json::json!({
    //           "err": err.to_string()
    //         }),
    //       };
    //       deno_core::serde_json::to_vec(&res).unwrap().into_boxed_slice()
    //     });
    //   deno_core::Op::Async(fut.boxed())
    // }

    panic!("async functions are not supported yet");
  } else {
    quote! {
      let res = match #impl_name(val, rest) {
        Ok(val) => deno_core::serde_json::json!({
          "ok": val
        }),
        Err(err) => deno_core::serde_json::json!({
          "err": err.to_string()
        }),
      };

      deno_core::Op::Sync(deno_core::serde_json::to_vec(&res).unwrap().into_boxed_slice())
    }
  };

  (quote! {
    #impl_func

    fn #name (_interface: &mut dyn deno_core::plugin_api::Interface, zero_copy: &mut [deno_core::ZeroCopyBuf]) -> Op {
      let val = deno_core::serde_json::from_slice(&zero_copy[0]).unwrap();
      let rest = &mut zero_copy[1..];

      #body
    }
  }).into()
}
