use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, FnArg, GenericArgument, ItemFn, ItemForeignMod, PathArguments};

/// `plugin_fn` is used to define an Extism callable function to export
///
/// It should be added to a function you would like to export, the function should
/// accept a parameter that implements `extism_pdk::FromBytes` and return a
/// `extism_pdk::FnResult` that contains a value that implements
/// `extism_pdk::ToBytes`. This maps input and output parameters to Extism input
/// and output instead of using function arguments directly.
///
/// ## Example
///
/// ```rust
/// use extism_pdk::{FnResult, plugin_fn};
/// #[plugin_fn]
/// pub fn greet(name: String) -> FnResult<String> {
///   let s = format!("Hello, {name}");
///   Ok(s)
/// }
/// ```
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
                    core::result::Result::Ok(x) => x,
                    core::result::Result::Err(rc) => {
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
                    core::result::Result::Ok(x) => x,
                    core::result::Result::Err(rc) => {
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

/// `shared_fn` is used to define a function that will be exported by a plugin but is not directly
/// callable by an Extism runtime. These functions can be used for runtime linking and mocking host
/// functions for tests. If direct access to Wasm native parameters is needed, then a bare
/// `extern "C" fn` should be used instead.
///
/// All arguments should implement `extism_pdk::ToBytes` and the return value should implement
/// `extism_pdk::FromBytes`, if `()` or `SharedFnResult<()>` then no value will be returned.
/// ## Example
///
/// ```rust
/// use extism_pdk::{SharedFnResult, shared_fn};
/// #[shared_fn]
/// pub fn greet2(greeting: String, name: String) -> SharedFnResult<String> {
///   let s = format!("{greeting}, {name}");
///   Ok(name)
/// }
/// ```
#[proc_macro_attribute]
pub fn shared_fn(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);

    if !matches!(function.vis, syn::Visibility::Public(..)) {
        panic!("extism_pdk::shared_fn expects a public function");
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
                    panic!("Receiver argument (self) cannot be used in extism_pdk::shared_fn")
                }
                FnArg::Typed(t) => &t.ty,
            };
            let arg = Ident::new(&format!("arg{i}"), Span::call_site());
            (
                quote! { #arg: extism_pdk::MemoryPointer<#t> },
                quote! { #arg.get()? },
            )
        })
        .unzip();

    if name == "main" {
        panic!(
            "export_pdk::shared_fn must not be applied to a `main` function. To fix, rename this to something other than `main`."
        )
    }

    let (no_result, raw_output) = match output {
        syn::ReturnType::Default => (true, quote! {}),
        syn::ReturnType::Type(_, t) => {
            let mut is_unit = false;
            if let syn::Type::Path(p) = t.as_ref() {
                if let Some(t) = p.path.segments.last() {
                    if t.ident != "SharedFnResult" {
                        panic!("extism_pdk::shared_fn expects a function that returns extism_pdk::SharedFnResult");
                    }
                    match &t.arguments {
                        PathArguments::AngleBracketed(args) => {
                            if args.args.len() == 1 {
                                match &args.args[0] {
                                    GenericArgument::Type(syn::Type::Tuple(t)) => {
                                        if t.elems.is_empty() {
                                            is_unit = true;
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                        _ => (),
                    }
                } else {
                    panic!("extism_pdk::shared_fn expects a function that returns extism_pdk::SharedFnResult");
                }
            };
            if is_unit {
                (true, quote! {})
            } else {
                (false, quote! {-> u64 })
            }
        }
    };

    if no_result {
        quote! {
            #[no_mangle]
            pub #constness #unsafety extern "C" fn #name(#(#raw_inputs,)*) {
                #constness #unsafety fn inner #generics(#inputs) -> extism_pdk::SharedFnResult<()> {
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
                    core::result::Result::Ok(mem) => {
                        mem.offset()
                    },
                    core::result::Result::Err(rc) => {
                        panic!("{}", rc.to_string());
                    }
                }
            }
        }
        .into()
    }
}

/// `host_fn` is used to import a host function from an `extern` block
///
/// ## Rust 1.82+ / Edition 2024
///
/// Starting with Rust 1.82, extern blocks can be marked `unsafe` and functions
/// within can be marked `safe`. When using `unsafe extern "ExtismHost"` blocks,
/// you can use `safe fn` to indicate that a host function is safe to call:
///
/// ```rust,ignore
/// #[host_fn]
/// unsafe extern "ExtismHost" {
///     // Safe to call - generates a safe wrapper function
///     safe fn get_config(key: String) -> String;
///
///     // Unsafe to call (implicit) - generates unsafe wrapper
///     fn dangerous_operation(data: Vec<u8>) -> Vec<u8>;
/// }
/// ```
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
    if item.abi.name.is_none() || item.abi.name.as_ref().unwrap().value() != "ExtismHost" {
        panic!("Expected `extern \"ExtismHost\"` or `unsafe extern \"ExtismHost\"` block");
    }

    // Track if this is an `unsafe extern` block (Rust 1.82+)
    let is_unsafe_extern = item.unsafety.is_some();
    let functions = item.items;

    let mut gen = quote!();

    for function in functions {
        // Handle regular ForeignItem::Fn (normal fn or unsafe fn)
        if let syn::ForeignItem::Fn(ref function) = function {
            // In non-unsafe extern blocks, all functions are unsafe
            // In unsafe extern blocks, unmarked fn is implicitly unsafe
            let wrapper = generate_host_fn_wrapper(&namespace, function, false);
            gen = quote! {
                #gen
                #wrapper
            };
            continue;
        }

        // Handle ForeignItem::Verbatim which syn uses for `safe fn` in unsafe extern blocks
        if let syn::ForeignItem::Verbatim(ref tokens) = function {
            if is_unsafe_extern {
                if let Some(wrapper) = parse_safe_fn_verbatim(&namespace, tokens) {
                    gen = quote! {
                        #gen
                        #wrapper
                    };
                    continue;
                }
            }
            // If we can't parse it, ignore or panic
            panic!("Unsupported item in extern block");
        }
    }

    gen.into()
}

/// Generates a wrapper function for a host function
fn generate_host_fn_wrapper(
    namespace: &str,
    function: &syn::ForeignItemFn,
    is_safe_fn: bool,
) -> proc_macro2::TokenStream {
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
                            .push(quote!(
                                extism_pdk::ManagedMemory::from(extism_pdk::ToMemory::to_memory(&&#i)?).offset()
                            ));
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

    // For safe functions, we generate a safe wrapper that uses unsafe internally
    // For unsafe functions, we generate an unsafe wrapper
    if is_safe_fn {
        if output_is_ptr {
            quote! {
                #impl_block

                #vis fn #name #generics (#original_inputs) -> core::result::Result<#output, extism_pdk::Error> {
                    // SAFETY: The caller of the macro has asserted this host function is safe
                    // by marking it with `safe fn` in an `unsafe extern` block.
                    let res = unsafe { extism_pdk::Memory::from(#impl_name(#(#into_inputs),*)) };
                    <#output as extism_pdk::FromBytes>::from_bytes(&res.to_vec())
                }
            }
        } else {
            quote! {
                #impl_block

                #vis fn #name #generics (#original_inputs) -> core::result::Result<#output, extism_pdk::Error> {
                    // SAFETY: The caller of the macro has asserted this host function is safe
                    // by marking it with `safe fn` in an `unsafe extern` block.
                    let res = unsafe { #impl_name(#(#into_inputs),*) };
                    core::result::Result::Ok(res)
                }
            }
        }
    } else if output_is_ptr {
        quote! {
            #impl_block

            #vis unsafe fn #name #generics (#original_inputs) -> core::result::Result<#output, extism_pdk::Error> {
                let res = extism_pdk::Memory::from(#impl_name(#(#into_inputs),*));
                <#output as extism_pdk::FromBytes>::from_bytes(&res.to_vec())
            }
        }
    } else {
        quote! {
            #impl_block

            #vis unsafe fn #name #generics (#original_inputs) -> core::result::Result<#output, extism_pdk::Error> {
                let res = #impl_name(#(#into_inputs),*);
                core::result::Result::Ok(res)
            }
        }
    }
}

/// Attempts to parse a `safe fn` from verbatim tokens
/// Returns Some(wrapper) if successful, None if the tokens don't represent a safe fn
fn parse_safe_fn_verbatim(namespace: &str, tokens: &proc_macro2::TokenStream) -> Option<proc_macro2::TokenStream> {
    use syn::parse::{Parse, Parser};

    // Try to parse: [visibility] safe fn name(args) [-> ReturnType];
    let parser = |input: syn::parse::ParseStream| -> syn::Result<syn::ForeignItemFn> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let vis: syn::Visibility = input.parse()?;

        // Check for `safe` keyword
        let safe_ident: syn::Ident = input.parse()?;
        if safe_ident != "safe" {
            return Err(syn::Error::new(safe_ident.span(), "expected `safe`"));
        }

        // Parse `fn`
        let fn_token: syn::token::Fn = input.parse()?;

        // Parse the rest of the signature
        let ident: syn::Ident = input.parse()?;
        let generics: syn::Generics = input.parse()?;

        let content;
        let paren_token = syn::parenthesized!(content in input);
        let inputs = content.parse_terminated(syn::FnArg::parse, syn::Token![,])?;

        let output: syn::ReturnType = input.parse()?;

        let where_clause: Option<syn::WhereClause> = input.parse()?;
        let mut generics = generics;
        generics.where_clause = where_clause;

        let semi_token: syn::Token![;] = input.parse()?;

        Ok(syn::ForeignItemFn {
            attrs,
            vis,
            sig: syn::Signature {
                constness: None,
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token,
                ident,
                generics,
                paren_token,
                inputs,
                variadic: None,
                output,
            },
            semi_token,
        })
    };

    match parser.parse2(tokens.clone()) {
        Ok(function) => {
            // It's a safe fn, generate a safe wrapper
            Some(generate_host_fn_wrapper(namespace, &function, true))
        }
        Err(_) => None,
    }
}
