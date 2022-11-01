use quote::quote;
use syn::{parse_macro_input, ItemFn, ItemStruct};

#[proc_macro_attribute]
pub fn function(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let function = parse_macro_input!(item as ItemFn);

    if !matches!(function.vis, syn::Visibility::Public(..)) {
        panic!("plugin_fn expects a public function");
    }

    let name = &function.sig.ident;
    let constness = &function.sig.constness;
    let unsafety = &function.sig.unsafety;
    let inputs = &function.sig.inputs;
    let output = &function.sig.output;
    let block = &function.block;

    quote! {
        #[no_mangle]
        pub #constness #unsafety extern "C" fn #name() -> i32 {
            fn inner(#inputs) #output {
                #block
            }

            let mut host = extism_pdk::Host::new();
            let input = extism_pdk::unwrap!(host.input());
            let output = extism_pdk::unwrap!(inner(&mut host, input));
            let output = unwrap!(output.output());
            host.set_output_memory(&output.keep());
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

#[proc_macro_attribute]
pub fn encoding(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    let args = parse_macro_input!(attr as Args);

    let vis = item.vis;
    let name = &item.ident;

    let encode = &args.arg[0];
    let decode = &args.arg[1];

    quote! {
        #vis struct #name<T>(pub T);

        impl<T: serde::de::DeserializeOwned> extism_pdk::Input for #name<T> {
            fn input(d: Vec<u8>) -> Result<Self, extism_pdk::Error> {
                let x = #decode(&d)?;
                Ok(#name(x))
            }
        }

        impl<T: serde::Serialize> extism_pdk::Output for #name<T> {
            fn output(&self) -> Result<extism_pdk::Memory, extism_pdk::Error> {
                let x = #encode(&self.0)?;
                Ok(extism_pdk::Memory::from_bytes(x))
            }
        }
    }
    .into()
}
