//! # ring-lang-codegen
//!
//! Proc macros to generate [Ring](https://ring-lang.github.io/) programming language
//! extensions in Rust with zero configuration.
//!
//! ## Features
//!
//! - **Zero config** - Just use the `ring_extension!` macro, no separate config files
//! - **Auto-generated bindings** - Structs, impl blocks, and functions are automatically wrapped
//! - **Auto ring_libinit!** - Library registration is generated for you
//! - **Full IDE support** - Works with rust-analyzer, autocomplete, and type checking
//!
//! ## Quick Start
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [lib]
//! crate-type = ["cdylib"]
//!
//! [dependencies]
//! ring-lang-rs = "0.1"
//! ring-lang-codegen = "0.1"
//! ```
//!
//! ## Usage
//!
//! ```rust,ignore
//! use ring_lang_codegen::ring_extension;
//! use ring_lang_rs::*;
//!
//! ring_extension! {
//!     prefix: "mylib";  // Optional prefix for all functions
//!
//!     // Standalone functions - generates mylib_add(a, b)
//!     pub fn add(a: i32, b: i32) -> i32 {
//!         a + b
//!     }
//!
//!     pub fn greet(name: &str) -> String {
//!         format!("Hello, {}!", name)
//!     }
//!
//!     // Structs with auto-generated accessors
//!     #[derive(Default)]
//!     pub struct Counter {
//!         pub value: i64,
//!         pub name: String,
//!     }
//!
//!     // Impl blocks with methods
//!     impl Counter {
//!         pub fn new(name: &str, initial: i64) -> Self {
//!             Counter { value: initial, name: name.to_string() }
//!         }
//!
//!         pub fn increment(&mut self) {
//!             self.value += 1;
//!         }
//!
//!         pub fn get_value(&self) -> i64 {
//!             self.value
//!         }
//!     }
//! }
//! ```
//!
//! ## What Gets Generated
//!
//! | Source | Generated Ring Functions |
//! |--------|--------------------------|
//! | `pub fn add(a, b)` | `mylib_add(a, b)` |
//! | `pub struct Counter` | `mylib_counter_new()`, `mylib_counter_delete(ptr)` |
//! | `pub value: i64` field | `mylib_counter_get_value(ptr)`, `mylib_counter_set_value(ptr, v)` |
//! | `impl Counter { pub fn new() }` | Replaces default `_new` with custom constructor |
//! | `pub fn increment(&mut self)` | `mylib_counter_increment(ptr)` |
//!
//! ## Ring Usage
//!
//! ```ring
//! loadlib("libmylib.so")  # or .dll / .dylib
//!
//! ? mylib_add(10, 20)        # 30
//! ? mylib_greet("World")     # Hello, World!
//!
//! obj = mylib_counter_new("test", 0)
//! mylib_counter_increment(obj)
//! ? mylib_counter_get_value(obj)  # 1
//! mylib_counter_delete(obj)
//! ```
//!
//! ## Supported Types
//!
//! | Rust Type | Ring Type |
//! |-----------|-----------|
//! | `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`, `f32`, `f64` | Number |
//! | `bool` | Number (0/1) |
//! | `String`, `&str` | String |
//! | Structs (via pointer) | C Pointer |

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
use syn::{
    parse_macro_input, FnArg, Ident, ImplItem, ImplItemFn, Item, ItemFn, ItemImpl, ItemStruct, Pat,
    ReturnType, Token, Type, Visibility,
};

struct RingExtension {
    prefix: Option<String>,
    items: Vec<Item>,
}

impl Parse for RingExtension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut prefix = None;
        let mut items = Vec::new();

        while !input.is_empty() {
            if input.peek(Ident) {
                let ident: Ident = input.parse()?;
                if ident == "prefix" {
                    let _: Token![:] = input.parse()?;
                    let lit: syn::LitStr = input.parse()?;
                    let _: Token![;] = input.parse()?;
                    prefix = Some(lit.value());
                    continue;
                } else {
                    return Err(syn::Error::new(ident.span(), "expected 'prefix' or item"));
                }
            }
            items.push(input.parse()?);
        }

        Ok(RingExtension { prefix, items })
    }
}

/// Define a Ring module with auto-generated bindings and ring_libinit!
#[proc_macro]
pub fn ring_extension(input: TokenStream) -> TokenStream {
    let module = parse_macro_input!(input as RingExtension);

    let prefix = module.prefix.unwrap_or_default();
    let prefix_underscore = if prefix.is_empty() {
        String::new()
    } else {
        format!("{}_", prefix)
    };

    let mut structs_with_custom_new: HashSet<String> = HashSet::new();
    let mut impl_methods: HashSet<(String, String)> = HashSet::new();

    for item in &module.items {
        if let Item::Impl(i) = item {
            if let Type::Path(p) = &*i.self_ty {
                let struct_name = p.path.segments.last().unwrap().ident.to_string();
                for impl_item in &i.items {
                    if let ImplItem::Fn(method) = impl_item {
                        let method_name = method.sig.ident.to_string();
                        if method_name == "new" {
                            structs_with_custom_new.insert(struct_name.clone());
                        }
                        impl_methods.insert((struct_name.clone(), method_name));
                    }
                }
            }
        }
    }

    let mut original_items = Vec::new();
    let mut generated_code = Vec::new();
    let mut registrations: Vec<(String, syn::Ident)> = Vec::new();

    for item in module.items {
        match item {
            Item::Struct(s) => {
                let has_custom_new = structs_with_custom_new.contains(&s.ident.to_string());
                let (orig, generated, regs) =
                    process_struct(&s, &prefix_underscore, has_custom_new, &impl_methods);
                original_items.push(orig);
                generated_code.push(generated);
                registrations.extend(regs);
            }
            Item::Impl(i) => {
                let (orig, generated, regs) = process_impl(&i, &prefix_underscore);
                original_items.push(orig);
                generated_code.push(generated);
                registrations.extend(regs);
            }
            Item::Fn(f) => {
                let (orig, generated, regs) = process_function(&f, &prefix_underscore);
                original_items.push(orig);
                generated_code.push(generated);
                registrations.extend(regs);
            }
            other => {
                original_items.push(quote! { #other });
            }
        }
    }

    let libinit_entries: Vec<_> = registrations
        .iter()
        .map(|(name, fn_ident)| {
            let name_bytes = format!("{}\0", name);
            quote! { #name_bytes.as_bytes() => #fn_ident }
        })
        .collect();

    let expanded = quote! {
        #(#original_items)*
        #(#generated_code)*

        ring_libinit! {
            #(#libinit_entries),*
        }
    };

    expanded.into()
}

fn process_struct(
    s: &ItemStruct,
    prefix: &str,
    has_custom_new: bool,
    impl_methods: &HashSet<(String, String)>,
) -> (TokenStream2, TokenStream2, Vec<(String, syn::Ident)>) {
    let struct_name = &s.ident;
    let struct_name_lower = struct_name.to_string().to_lowercase();
    let type_const = format_ident!("{}_TYPE", struct_name.to_string().to_uppercase());
    let type_const_str = format!("{}\0", struct_name);

    let mut regs = Vec::new();

    let delete_fn_name = format_ident!("ring_{}{}_delete", prefix, struct_name_lower);
    let delete_ring_name = format!("{}{}_delete", prefix, struct_name_lower);
    regs.push((delete_ring_name, delete_fn_name.clone()));

    let new_code = if !has_custom_new {
        let new_fn_name = format_ident!("ring_{}{}_new", prefix, struct_name_lower);
        let new_ring_name = format!("{}{}_new", prefix, struct_name_lower);
        regs.push((new_ring_name, new_fn_name.clone()));

        quote! {
            ring_func!(#new_fn_name, |p| {
                ring_check_paracount!(p, 0);
                let obj = Box::new(#struct_name::default());
                ring_ret_cpointer!(p, Box::into_raw(obj), #type_const);
            });
        }
    } else {
        quote! {}
    };

    let mut accessors = Vec::new();
    let struct_name_str = struct_name.to_string();

    if let syn::Fields::Named(fields) = &s.fields {
        for field in &fields.named {
            if !matches!(field.vis, Visibility::Public(_)) {
                continue;
            }

            let field_name = field.ident.as_ref().unwrap();
            let field_name_str = field_name.to_string();
            let field_type = &field.ty;

            let getter_method = format!("get_{}", field_name_str);
            let setter_method = format!("set_{}", field_name_str);

            if !impl_methods.contains(&(struct_name_str.clone(), getter_method.clone()))
                && !impl_methods.contains(&(struct_name_str.clone(), field_name_str.clone()))
            {
                let getter_fn =
                    format_ident!("ring_{}{}_get_{}", prefix, struct_name_lower, field_name);
                let getter_name = format!("{}{}_get_{}", prefix, struct_name_lower, field_name);
                regs.push((getter_name, getter_fn.clone()));

                let getter_code = generate_field_getter(
                    &getter_fn,
                    struct_name,
                    &type_const,
                    field_name,
                    field_type,
                );
                accessors.push(getter_code);
            }

            if !impl_methods.contains(&(struct_name_str.clone(), setter_method)) {
                let setter_fn =
                    format_ident!("ring_{}{}_set_{}", prefix, struct_name_lower, field_name);
                let setter_name = format!("{}{}_set_{}", prefix, struct_name_lower, field_name);
                regs.push((setter_name, setter_fn.clone()));

                let setter_code = generate_field_setter(
                    &setter_fn,
                    struct_name,
                    &type_const,
                    field_name,
                    field_type,
                );
                accessors.push(setter_code);
            }
        }
    }

    let original = quote! { #s };

    let generated = quote! {
        const #type_const: &[u8] = #type_const_str.as_bytes();

        #new_code

        ring_func!(#delete_fn_name, |p| {
            ring_check_paracount!(p, 1);
            ring_check_cpointer!(p, 1);
            let ptr = ring_get_cpointer!(p, 1, #type_const);
            if !ptr.is_null() {
                unsafe { let _ = Box::from_raw(ptr as *mut #struct_name); }
            }
        });

        #(#accessors)*
    };

    (original, generated, regs)
}

fn process_impl(
    i: &ItemImpl,
    prefix: &str,
) -> (TokenStream2, TokenStream2, Vec<(String, syn::Ident)>) {
    let struct_name = match &*i.self_ty {
        Type::Path(p) => p.path.segments.last().unwrap().ident.clone(),
        _ => return (quote! { #i }, quote! {}, vec![]),
    };

    let struct_name_lower = struct_name.to_string().to_lowercase();
    let type_const = format_ident!("{}_TYPE", struct_name.to_string().to_uppercase());

    let mut regs = Vec::new();
    let mut method_wrappers = Vec::new();

    for item in &i.items {
        if let ImplItem::Fn(method) = item {
            if !matches!(method.vis, Visibility::Public(_)) {
                continue;
            }

            let method_name = &method.sig.ident;
            let method_name_str = method_name.to_string();

            if method_name_str == "new" {
                let (code, name, fn_ident) = generate_custom_new(
                    &struct_name,
                    &struct_name_lower,
                    &type_const,
                    method,
                    prefix,
                );
                method_wrappers.push(code);
                regs.push((name, fn_ident));
                continue;
            }

            let has_self = method
                .sig
                .inputs
                .iter()
                .any(|arg| matches!(arg, FnArg::Receiver(_)));

            if has_self {
                let (code, name, fn_ident) = generate_method(
                    &struct_name,
                    &struct_name_lower,
                    &type_const,
                    method,
                    prefix,
                );
                method_wrappers.push(code);
                regs.push((name, fn_ident));
            }
        }
    }

    let original = quote! { #i };
    let generated = quote! { #(#method_wrappers)* };

    (original, generated, regs)
}

fn process_function(
    f: &ItemFn,
    prefix: &str,
) -> (TokenStream2, TokenStream2, Vec<(String, syn::Ident)>) {
    let fn_name = &f.sig.ident;
    let ring_fn_name = format_ident!("ring_{}{}", prefix, fn_name);
    let ring_name = format!("{}{}", prefix, fn_name);

    let params: Vec<_> = f
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat) = arg {
                let name = if let Pat::Ident(ident) = &*pat.pat {
                    ident.ident.clone()
                } else {
                    return None;
                };
                Some((name, (*pat.ty).clone()))
            } else {
                None
            }
        })
        .collect();

    let param_count = params.len();
    let mut checks = Vec::new();
    let mut gets = Vec::new();
    let mut args = Vec::new();

    for (i, (name, ty)) in params.iter().enumerate() {
        let idx = (i + 1) as i32;
        let type_str = quote!(#ty).to_string();

        if is_number_type(&type_str) {
            checks.push(quote! { ring_check_number!(p, #idx); });
            let cast = get_number_cast(&type_str);
            gets.push(quote! { let #name = ring_get_number!(p, #idx) as #cast; });
        } else if is_string_type(&type_str) {
            checks.push(quote! { ring_check_string!(p, #idx); });
            gets.push(quote! { let #name = ring_get_string!(p, #idx); });
        } else if type_str == "bool" {
            checks.push(quote! { ring_check_number!(p, #idx); });
            gets.push(quote! { let #name = ring_get_number!(p, #idx) != 0.0; });
        } else {
            checks.push(quote! { ring_check_number!(p, #idx); });
            gets.push(quote! { let #name = ring_get_number!(p, #idx) as _; });
        }
        args.push(name.clone());
    }

    let param_count_i32 = param_count as i32;
    let return_code = generate_return_code(&f.sig.output, quote! { #fn_name(#(#args),*) });

    let original = quote! { #f };
    let generated = quote! {
        ring_func!(#ring_fn_name, |p| {
            ring_check_paracount!(p, #param_count_i32);
            #(#checks)*
            #(#gets)*
            #return_code
        });
    };

    (original, generated, vec![(ring_name, ring_fn_name)])
}

fn generate_field_getter(
    fn_name: &syn::Ident,
    struct_name: &syn::Ident,
    type_const: &syn::Ident,
    field_name: &syn::Ident,
    field_type: &Type,
) -> TokenStream2 {
    let type_str = quote!(#field_type).to_string();
    let return_expr = if is_number_type(&type_str) {
        quote! { ring_ret_number!(p, obj.#field_name as f64); }
    } else if is_string_type(&type_str) {
        quote! { ring_ret_string!(p, &obj.#field_name); }
    } else if type_str == "bool" {
        quote! { ring_ret_number!(p, if obj.#field_name { 1.0 } else { 0.0 }); }
    } else {
        quote! { ring_ret_number!(p, obj.#field_name as f64); }
    };

    quote! {
        ring_func!(#fn_name, |p| {
            ring_check_paracount!(p, 1);
            ring_check_cpointer!(p, 1);
            if let Some(obj) = ring_get_pointer!(p, 1, #struct_name, #type_const) {
                #return_expr
            } else {
                ring_error!(p, concat!("Invalid ", stringify!(#struct_name), " pointer"));
            }
        });
    }
}

fn generate_field_setter(
    fn_name: &syn::Ident,
    struct_name: &syn::Ident,
    type_const: &syn::Ident,
    field_name: &syn::Ident,
    field_type: &Type,
) -> TokenStream2 {
    let type_str = quote!(#field_type).to_string();

    let (check, set_expr) = if is_number_type(&type_str) {
        let cast = get_number_cast(&type_str);
        (
            quote! { ring_check_number!(p, 2); },
            quote! { obj.#field_name = ring_get_number!(p, 2) as #cast; },
        )
    } else if is_string_type(&type_str) {
        (
            quote! { ring_check_string!(p, 2); },
            quote! { obj.#field_name = ring_get_string!(p, 2).to_string(); },
        )
    } else if type_str == "bool" {
        (
            quote! { ring_check_number!(p, 2); },
            quote! { obj.#field_name = ring_get_number!(p, 2) != 0.0; },
        )
    } else {
        (
            quote! { ring_check_number!(p, 2); },
            quote! { obj.#field_name = ring_get_number!(p, 2) as _; },
        )
    };

    quote! {
        ring_func!(#fn_name, |p| {
            ring_check_paracount!(p, 2);
            ring_check_cpointer!(p, 1);
            #check
            if let Some(obj) = ring_get_pointer!(p, 1, #struct_name, #type_const) {
                #set_expr
            } else {
                ring_error!(p, concat!("Invalid ", stringify!(#struct_name), " pointer"));
            }
        });
    }
}

fn generate_custom_new(
    struct_name: &syn::Ident,
    struct_name_lower: &str,
    type_const: &syn::Ident,
    method: &ImplItemFn,
    prefix: &str,
) -> (TokenStream2, String, syn::Ident) {
    let fn_name = format_ident!("ring_{}{}_new", prefix, struct_name_lower);
    let ring_name = format!("{}{}_new", prefix, struct_name_lower);

    let params: Vec<_> = method
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat) = arg {
                let name = if let Pat::Ident(ident) = &*pat.pat {
                    ident.ident.clone()
                } else {
                    return None;
                };
                Some((name, (*pat.ty).clone()))
            } else {
                None
            }
        })
        .collect();

    let param_count = params.len() as i32;
    let mut checks = Vec::new();
    let mut gets = Vec::new();
    let mut args = Vec::new();

    for (i, (name, ty)) in params.iter().enumerate() {
        let idx = (i + 1) as i32;
        let type_str = quote!(#ty).to_string();

        if is_number_type(&type_str) {
            checks.push(quote! { ring_check_number!(p, #idx); });
            let cast = get_number_cast(&type_str);
            gets.push(quote! { let #name = ring_get_number!(p, #idx) as #cast; });
        } else if is_string_type(&type_str) {
            checks.push(quote! { ring_check_string!(p, #idx); });
            gets.push(quote! { let #name = ring_get_string!(p, #idx); });
        } else if type_str == "bool" {
            checks.push(quote! { ring_check_number!(p, #idx); });
            gets.push(quote! { let #name = ring_get_number!(p, #idx) != 0.0; });
        } else {
            checks.push(quote! { ring_check_number!(p, #idx); });
            gets.push(quote! { let #name = ring_get_number!(p, #idx) as _; });
        }
        args.push(name.clone());
    }

    let code = quote! {
        ring_func!(#fn_name, |p| {
            ring_check_paracount!(p, #param_count);
            #(#checks)*
            #(#gets)*
            let obj = Box::new(#struct_name::new(#(#args),*));
            ring_ret_cpointer!(p, Box::into_raw(obj), #type_const);
        });
    };

    (code, ring_name, fn_name)
}

fn generate_method(
    struct_name: &syn::Ident,
    struct_name_lower: &str,
    type_const: &syn::Ident,
    method: &ImplItemFn,
    prefix: &str,
) -> (TokenStream2, String, syn::Ident) {
    let method_name = &method.sig.ident;
    let fn_name = format_ident!("ring_{}{}_{}", prefix, struct_name_lower, method_name);
    let ring_name = format!("{}{}_{}", prefix, struct_name_lower, method_name);

    let params: Vec<_> = method
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat) = arg {
                let name = if let Pat::Ident(ident) = &*pat.pat {
                    ident.ident.clone()
                } else {
                    return None;
                };
                Some((name, (*pat.ty).clone()))
            } else {
                None
            }
        })
        .collect();

    let param_count = (params.len() + 1) as i32;
    let mut checks = Vec::new();
    let mut gets = Vec::new();
    let mut args = Vec::new();

    for (i, (name, ty)) in params.iter().enumerate() {
        let idx = (i + 2) as i32;
        let type_str = quote!(#ty).to_string();

        if is_number_type(&type_str) {
            checks.push(quote! { ring_check_number!(p, #idx); });
            let cast = get_number_cast(&type_str);
            gets.push(quote! { let #name = ring_get_number!(p, #idx) as #cast; });
        } else if is_string_type(&type_str) {
            checks.push(quote! { ring_check_string!(p, #idx); });
            gets.push(quote! { let #name = ring_get_string!(p, #idx); });
        } else if type_str == "bool" {
            checks.push(quote! { ring_check_number!(p, #idx); });
            gets.push(quote! { let #name = ring_get_number!(p, #idx) != 0.0; });
        } else {
            checks.push(quote! { ring_check_number!(p, #idx); });
            gets.push(quote! { let #name = ring_get_number!(p, #idx) as _; });
        }
        args.push(name.clone());
    }

    let return_code =
        generate_return_code(&method.sig.output, quote! { obj.#method_name(#(#args),*) });

    let code = quote! {
        ring_func!(#fn_name, |p| {
            ring_check_paracount!(p, #param_count);
            ring_check_cpointer!(p, 1);
            #(#checks)*
            if let Some(obj) = ring_get_pointer!(p, 1, #struct_name, #type_const) {
                #(#gets)*
                #return_code
            } else {
                ring_error!(p, concat!("Invalid ", stringify!(#struct_name), " pointer"));
            }
        });
    };

    (code, ring_name, fn_name)
}

fn generate_return_code(output: &ReturnType, call: TokenStream2) -> TokenStream2 {
    match output {
        ReturnType::Default => quote! { #call; },
        ReturnType::Type(_, ty) => {
            let type_str = quote!(#ty).to_string();
            if is_number_type(&type_str) {
                quote! {
                    let __result = #call;
                    ring_ret_number!(p, __result as f64);
                }
            } else if is_string_type(&type_str) {
                quote! {
                    let __result = #call;
                    ring_ret_string!(p, &__result);
                }
            } else if type_str == "bool" {
                quote! {
                    let __result = #call;
                    ring_ret_number!(p, if __result { 1.0 } else { 0.0 });
                }
            } else {
                quote! {
                    let __result = #call;
                    ring_ret_number!(p, __result as f64);
                }
            }
        }
    }
}

fn is_number_type(ty: &str) -> bool {
    let ty = ty.trim();
    matches!(
        ty,
        "i8" | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
            | "f32"
            | "f64"
    )
}

fn is_string_type(ty: &str) -> bool {
    let ty = ty.trim();
    ty == "String" || ty == "& str" || ty.contains("str")
}

fn get_number_cast(ty: &str) -> TokenStream2 {
    let ty = ty.trim();
    match ty {
        "i8" => quote!(i8),
        "i16" => quote!(i16),
        "i32" => quote!(i32),
        "i64" => quote!(i64),
        "i128" => quote!(i128),
        "isize" => quote!(isize),
        "u8" => quote!(u8),
        "u16" => quote!(u16),
        "u32" => quote!(u32),
        "u64" => quote!(u64),
        "u128" => quote!(u128),
        "usize" => quote!(usize),
        "f32" => quote!(f32),
        "f64" => quote!(f64),
        _ => quote!(f64),
    }
}
