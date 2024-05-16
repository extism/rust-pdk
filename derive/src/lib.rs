use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, ItemForeignMod};

/// `plugin_fn` is used to define an Extism callable function to export
///
/// It should be added to a function you would like to export, the function should
/// accept a parameter that implements `extism_pdk::FromBytes` and return a
/// `extism_pdk::FnResult` that contains a value that implements
/// `extism_pdk::ToBytes`.
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

    let no_args = inputs.is_empty();

    if name == "main" {
        panic!(
            "extism_pdk::plugin_fn must not be applied to a `main` function. To fix, rename this to something other than `main`."
        )
    }

    match output {
        syn::ReturnType::Default => panic!(
            "extism_pdk::plugin_fn expects a return value, `()` may be used if no output is needed"
        ),
        syn::ReturnType::Type(_, t) => {
            if let syn::Type::Path(p) = t.as_ref() {
                if let Some(t) = p.path.segments.last() {
                    if t.ident != "FnResult" {
                        panic!("extism_pdk::plugin_fn expects a function that returns extism_pdk::FnResult");
                    }
                } else {
                    panic!("extism_pdk::plugin_fn expects a function that returns extism_pdk::FnResult");
                }
            }
        }
    }

    if no_args {
        quote! {
            #[no_mangle]
            pub #constness #unsafety extern "C" fn #name() -> i32 {
                #constness #unsafety fn inner #generics() #output {
                    #block
                }

                let output = match inner() {
                    Ok(x) => x,
                    Err(rc) => {
                        let err = format!("{:?}", rc.0);
                        let mut mem = extism_pdk::Memory::from_bytes(&err).unwrap();
                        unsafe {
                            extism_pdk::extism::error_set(mem.offset());
                        }
                        return rc.1;
                    }
                };
                extism_pdk::unwrap!(extism_pdk::output(&output));
                0
            }
        }
        .into()
    } else {
        quote! {
            #[no_mangle]
            pub #constness #unsafety extern "C" fn #name() -> i32 {
                #constness #unsafety fn inner #generics(#inputs) #output {
                    #block
                }

                let input = extism_pdk::unwrap!(extism_pdk::input());
                let output = match inner(input) {
                    Ok(x) => x,
                    Err(rc) => {
                        let err = format!("{:?}", rc.0);
                        let mut mem = extism_pdk::Memory::from_bytes(&err).unwrap();
                        unsafe {
                            extism_pdk::extism::error_set(mem.offset());
                        }
                        return rc.1;
                    }
                };
                extism_pdk::unwrap!(extism_pdk::output(&output));
                0
            }
        }
        .into()
    }
}

/// `export_fn` is used to define a function that will be exported by a plugin, but is not directly
/// callable by an Extism runtime. These functions can be used for runtime linking and mocking host
/// functions for tests. If direct access to Wasm native parameters is needed, then a bare
/// `extern "C" fn` should be used instead.
///
/// All arguments should implement `extism_pdk::ToBytes` and the return value should implement
/// `extism_pdk::FromBytes`
#[proc_macro_attribute]
pub fn export_fn(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);

    if !matches!(function.vis, syn::Visibility::Public(..)) {
        panic!("extism_pdk::export_fn expects a public function");
    }

    let name = &function.sig.ident;
    let constness = &function.sig.constness;
    let unsafety = &function.sig.unsafety;
    let generics = &function.sig.generics;
    let inputs = &mut function.sig.inputs;
    let output = &mut function.sig.output;
    let block = &function.block;

    let (raw_inputs, raw_args): (Vec<_>, Vec<_>) = inputs
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let t = match x {
                FnArg::Receiver(_) => {
                    panic!("Receiver argument (self) cannot be used in extism_pdk::export_fn")
                }
                FnArg::Typed(t) => &t.ty,
            };
            let arg = Ident::new(&format!("arg{i}"), Span::call_site());
            (
                quote! { #arg: extism_pdk::memory::Pointer<#t> },
                quote! { #arg.get()? },
            )
        })
        .unzip();

    if name == "main" {
        panic!(
            "export_pdk::export_fn must not be applied to a `main` function. To fix, rename this to something other than `main`."
        )
    }

    let (no_result, raw_output) = match output {
        syn::ReturnType::Default => (true, quote! {}),
        syn::ReturnType::Type(_, t) => {
            if let syn::Type::Path(p) = t.as_ref() {
                if let Some(t) = p.path.segments.last() {
                    if t.ident != "ExportResult" {
                        panic!("extism_pdk::export_fn expects a function that returns extism_pdk::ExportResult");
                    }
                } else {
                    panic!("extism_pdk::export_fn expects a function that returns extism_pdk::ExportResult");
                }
            };
            (false, quote! {-> u64 })
        }
    };

    if no_result {
        quote! {
            #[no_mangle]
            pub #constness #unsafety extern "C" fn #name(#(#raw_inputs,)*) {
                #constness #unsafety fn inner #generics(#inputs) -> extism_pdk::ExportResult<()> {
                    #block
                }


                let r = || inner(#(#raw_args,)*);
                if let Err(rc) = r() {
                    panic!("{}", rc.to_string());
                }
            }
        }
        .into()
    } else {
        quote! {
            #[no_mangle]
            pub #constness #unsafety extern "C" fn #name(#(#raw_inputs,)*) #raw_output {
                #constness #unsafety fn inner #generics(#inputs) #output {
                    #block
                }

                let r = || inner(#(#raw_args,)*);
                match r().and_then(|x| extism_pdk::Memory::new(&x)) {
                    Ok(mem) => {
                        mem.offset()
                    },
                    Err(rc) => {
                        panic!("{}", rc.to_string());
                    }
                }
            }
        }
        .into()
    }
}

/// `host_fn` is used to import a host function from an `extern` block
#[proc_macro_attribute]
pub fn host_fn(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let namespace = if let Ok(ns) = syn::parse::<syn::LitStr>(attr) {
        ns.value()
    } else {
        "extism:host/user".to_string()
    };

    let item = parse_macro_input!(item as ItemForeignMod);
    if item.abi.name.is_none() || item.abi.name.unwrap().value() != "ExtismHost" {
        panic!("Expected `extern \"ExtismHost\"` block");
    }
    let functions = item.items;

    let mut gen = quote!();

    for function in functions {
        if let syn::ForeignItem::Fn(function) = function {
            let name = &function.sig.ident;
            let original_inputs = function.sig.inputs.clone();
            let output = &function.sig.output;

            let vis = &function.vis;
            let generics = &function.sig.generics;
            let mut into_inputs = vec![];
            let mut converted_inputs = vec![];

            let (output_is_ptr, converted_output) = match output {
                syn::ReturnType::Default => (false, quote!(())),
                syn::ReturnType::Type(_, _) => (true, quote!(u64)),
            };

            for input in &original_inputs {
                match input {
                    syn::FnArg::Typed(t) => {
                        let mut input = t.clone();
                        input.ty = Box::new(syn::Type::Verbatim(quote!(u64)));
                        converted_inputs.push(syn::FnArg::Typed(input));
                        match &*t.pat {
                            syn::Pat::Ident(i) => {
                                into_inputs
                                    .push(quote!(extism_pdk::ToMemory::to_memory(&&#i)?.offset()));
                            }
                            _ => panic!("invalid host function argument"),
                        }
                    }
                    _ => panic!("self arguments are not permitted in host functions"),
                }
            }

            let impl_name = syn::Ident::new(&format!("{name}_impl"), name.span());
            let link_name = name.to_string();
            let link_name = link_name.as_str();

            let impl_block = quote! {
                #[link(wasm_import_module = #namespace)]
                extern "C" {
                    #[link_name = #link_name]
                    fn #impl_name(#(#converted_inputs),*) -> #converted_output;
                }
            };

            let output = match output {
                syn::ReturnType::Default => quote!(()),
                syn::ReturnType::Type(_, ty) => quote!(#ty),
            };

            if output_is_ptr {
                gen = quote! {
                    #gen

                    #impl_block

                    #vis unsafe fn #name #generics (#original_inputs) -> Result<#output, extism_pdk::Error> {
                        let res = extism_pdk::Memory::from(#impl_name(#(#into_inputs),*));
                        <#output as extism_pdk::FromBytes>::from_bytes(&res.to_vec())
                    }
                };
            } else {
                gen = quote! {
                    #gen

                    #impl_block

                    #vis unsafe fn #name #generics (#original_inputs) -> Result<#output, extism_pdk::Error> {
                        let res = #impl_name(#(#into_inputs),*);
                        Ok(res)
                    }
                };
            }
        }
    }

    gen.into()
}
