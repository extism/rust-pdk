use quote::quote;
use syn::{parse_macro_input, ItemFn, ItemStruct};

/// `function` is used to define a function that will be exported by a plugin
///
/// It should be added to a function you would like to export, the function should
/// accept a parameter that implements `extism_pdk::FromBytes` and return a
/// `extism_pdk::PluginResult` that contains a value that implements
/// `extism_pdk::ToMemory`.
#[proc_macro_attribute]
pub fn function(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let function = parse_macro_input!(item as ItemFn);

    if !matches!(function.vis, syn::Visibility::Public(..)) {
        panic!("extism_pdk::function expects a public function");
    }

    let name = &function.sig.ident;
    let constness = &function.sig.constness;
    let unsafety = &function.sig.unsafety;
    let inputs = &function.sig.inputs;
    let output = &function.sig.output;
    let block = &function.block;

    if inputs.len() != 1 {
        panic!("extism_pdk::function expects a function with one argument, `()` may be used if no input is needed");
    }

    quote! {
        #[no_mangle]
        pub #constness #unsafety extern "C" fn #name() -> i32 {
            fn inner(#inputs) #output {
                #block
            }

            let input = extism_pdk::unwrap!(extism_pdk::input());
            let output = extism_pdk::unwrap!(inner(input));
            unwrap!(extism_pdk::output(output));
            0
        }
    }
    .into()
}

struct Args {
    arg: Vec<syn::Path>,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let args =
            syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated(input)?;
        Ok(Args {
            arg: args.into_iter().collect(),
        })
    }
}

/// `encoding` is used to add a new serde encoder/decoder. It accepts two parameters:
/// 1) path to serialization function
/// 2) path to deserialization function
///
/// ```rust,ignore
/// #[encoding(serde_json::to_vec, serde_json::from_slice)]]
/// pub struct Json;
/// ```
#[proc_macro_attribute]
pub fn encoding(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    let args = parse_macro_input!(attr as Args);

    if args.arg.len() != 2 {
        panic!("extism_pdk::encoding expects 2 arguments (encoding function and decoding function) but got {}", args.arg.len())
    }

    let vis = item.vis;
    let name = &item.ident;

    let encode = &args.arg[0];
    let decode = &args.arg[1];

    quote! {
        #vis struct #name<T>(pub T);

        impl<T: serde::de::DeserializeOwned> extism_pdk::FromBytes for #name<T> {
            fn from_bytes(d: Vec<u8>) -> Result<Self, extism_pdk::Error> {
                let x = #decode(&d)?;
                Ok(#name(x))
            }
        }

        impl<T: serde::Serialize> extism_pdk::ToMemory for #name<T> {
            fn to_memory(&self) -> Result<extism_pdk::Memory, extism_pdk::Error> {
                let x = #encode(&self.0)?;
                Ok(extism_pdk::Memory::from_bytes(x))
            }
        }
    }
    .into()
}
