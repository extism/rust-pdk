use quote::quote;
use syn::{parse_macro_input, ItemFn, ItemForeignMod, ItemStruct};

/// `plugin_fn` is used to define a function that will be exported by a plugin
///
/// It should be added to a function you would like to export, the function should
/// accept a parameter that implements `extism_pdk::FromBytes` and return a
/// `extism_pdk::FnResult` that contains a value that implements
/// `extism_pdk::ToMemory`.
#[proc_macro_attribute]
pub fn plugin_fn(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);

    if !matches!(function.vis, syn::Visibility::Public(..)) {
        panic!("extism_pdk::plugin_fn expects a public function");
    }

    let name = &function.sig.ident;
    let constness = &function.sig.constness;
    let unsafety = &function.sig.unsafety;
    let generics = &function.sig.generics;
    let inputs = &mut function.sig.inputs;
    let output = &mut function.sig.output;
    let block = &function.block;

    if inputs.is_empty() {
        panic!("extism_pdk::plugin_fn expects a function with one argument, `()` may be used if no input is needed");
    }

    if name == "main" {
        panic!(
            "extism_pdk::plugin_fn must not be applied to a `main` function. To fix, rename this to something other than `main`."
        )
    }

    match output {
        syn::ReturnType::Default => panic!(
            "extism_pdk::plugin_fn expects a return value, `()` may be used if no output is needed"
        ),
        syn::ReturnType::Type(_, t) => match t.as_ref() {
            syn::Type::Path(p) => {
                if let Some(t) = p.path.segments.last() {
                    if t.ident != "FnResult" {
                        panic!("extism_pdk::plugin_fn expects a function that returns extism_pdk::FnResult");
                    }
                } else {
                    panic!("extism_pdk::plugin_fn expects a function that returns extism_pdk::FnResult");
                }
            }
            _ => (),
        },
    }

    quote! {
        #[no_mangle]
        pub #constness #unsafety extern "C" fn #name() -> i32 {
            fn inner #generics(#inputs) #output {
                #block
            }

            let input = extism_pdk::unwrap!(extism_pdk::input());
            let output = extism_pdk::unwrap!(inner(input));
            let status = output.status();
            unwrap!(extism_pdk::output(output));
            0
        }
    }
    .into()
}

/// `host_fn` is used to define a host function that will be callable from within a plugin
#[proc_macro_attribute]
pub fn host_fn(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as ItemForeignMod);
    if item.abi.name.is_none() || item.abi.name.unwrap().value() != "ExtismHost" {
        panic!("Expected `extern \"ExtismHost\"` block");
    }
    let functions = item.items;

    let mut gen = quote!();

    for function in functions {
        if let syn::ForeignItem::Fn(function) = function {
            let name = &function.sig.ident;
            // let constness = &function.sig.constness;
            // let unsafety = &function.sig.unsafety;
            // let generics = &function.sig.generics;
            let original_inputs = function.sig.inputs.clone();
            // let inputs = function.sig.inputs.into_iter();
            let output = &function.sig.output;

            let mut into_inputs = vec![];
            let mut converted_inputs = vec![];

            let converted_output = match output {
                syn::ReturnType::Default => quote!(#output),
                syn::ReturnType::Type(_, ty) => match &**ty {
                    syn::Type::Path(p) => {
                        if let Some(ident) = p.path.get_ident() {
                            if ident == "Memory" {
                                quote!(u64)
                            } else {
                                quote!(#output)
                            }
                        } else if p
                            .path
                            .segments
                            .iter()
                            .map(|x| x.ident.to_string())
                            .collect::<Vec<String>>()
                            == vec!["extism_pdk", "Memory"]
                        {
                            quote!(u64)
                        } else {
                            quote!(#output)
                        }
                    }
                    _ => quote!(#output),
                },
            };

            for input in &original_inputs {
                match input {
                    syn::FnArg::Typed(t) => {
                        match &*t.ty {
                            syn::Type::Path(p) => {
                                if let Some(ident) = p.path.get_ident() {
                                    if ident == "Memory" {
                                        let mut input = t.clone();
                                        input.ty = Box::new(syn::Type::Verbatim(quote!(u64)));
                                        converted_inputs.push(syn::FnArg::Typed(input));
                                    } else {
                                        converted_inputs.push(input.clone());
                                    }
                                } else if p
                                    .path
                                    .segments
                                    .iter()
                                    .map(|x| x.ident.to_string())
                                    .collect::<Vec<String>>()
                                    == vec!["extism_pdk", "Memory"]
                                {
                                    let mut input = t.clone();
                                    input.ty = Box::new(syn::Type::Verbatim(quote!(u64)));
                                    converted_inputs.push(syn::FnArg::Typed(input));
                                } else {
                                    converted_inputs.push(input.clone());
                                }
                            }
                            _ => converted_inputs.push(input.clone()),
                        }
                        match &*t.pat {
                            syn::Pat::Ident(i) => {
                                into_inputs.push(quote!(#i.into()));
                            }
                            _ => panic!("invalid host function argument"),
                        }
                    }
                    _ => panic!("self arguments are not permitted in host functions"),
                }
            }

            let impl_name = syn::Ident::new(&format!("{name}_impl"), name.span());

            gen = quote! {
                #gen

                extern "C" {
                    fn #impl_name(#(#converted_inputs),*) -> #converted_output;
                }

                #[no_mangle]
                pub unsafe fn #name(#original_inputs) #output {
                    #impl_name(#(#into_inputs),*).into()
                }
            };
        }
    }

    gen.into()
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
