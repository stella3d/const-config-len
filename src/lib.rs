use std::fs;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ExprLit, Lit};

#[proc_macro]
pub fn const_config_len(input: TokenStream) -> TokenStream {
    let args_expr = parse_macro_input!(input as syn::Expr);
    
    // Get file_path and nested_field based on input type.
    let (file_path, nested_field) = if let syn::Expr::Tuple(tuple) = args_expr {
        let elems: Vec<&Expr> = tuple.elems.iter().collect();
        if elems.is_empty() {
            panic!("Expected at least one argument: file path");
        }
        // element 0: file path
        let file_path = if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = elems[0] {
            s.value()
        } else {
            panic!("Expected first argument to be a string literal for file path")
        };
        let nested_field = match elems.len() {
            1 => None,
            2 => {
                let arg = if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = elems[1] {
                    s.value()
                } else {
                    panic!("Expected second argument to be a string literal")
                };
                if arg == "" { None } else { Some(arg) }
            },
            _ => panic!("Too many arguments provided"),
        };
        (file_path, nested_field)
    } else {
        if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = args_expr {
            let file_path = s.value();
            (file_path.clone(), None)
        } else {
            panic!("Expected a string literal or a tuple expression")
        }
    };

    let bytes = fs::read(&file_path)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", file_path));

    let length = {
        // Only JSON supported
        if let Some(ref nested) = nested_field {
            let v: serde_json::Value = serde_json::from_slice(&bytes)
                .unwrap_or_else(|_| panic!("Failed to parse JSON in file: {}", file_path));
            let nested_value = nested.split('.').fold(&v, |acc, key| {
                acc.get(key).unwrap_or_else(|| panic!("Missing nested field '{}' in file: {}", nested, file_path))
            });
            if let serde_json::Value::Array(arr) = nested_value {
                arr.len()
            } else {
                panic!("Nested field '{}' is not an array in file: {}", nested, file_path)
            }
        } else {
            let vec: Vec<serde_json::Value> = serde_json::from_slice(&bytes)
                .unwrap_or_else(|_| panic!("Failed to parse JSON array in file: {}", file_path));
            vec.len()
        }
    };

    TokenStream::from(quote! { #length })
}
