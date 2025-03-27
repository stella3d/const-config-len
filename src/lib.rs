use std::{fs, path::Path};

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ExprLit, Lit};

#[proc_macro]
pub fn const_config_len(input: TokenStream) -> TokenStream {
    // Parse input as an expression; support tuple or single literal.
    let args_expr = parse_macro_input!(input as syn::Expr);
    
    // Get file_path and scheme based on input type.
    let (file_path, scheme) = if let syn::Expr::Tuple(tuple) = args_expr {
        let elems: Vec<&Expr> = tuple.elems.iter().collect();
        if elems.is_empty() {
            panic!("Expected at least one argument: file path");
        }
        let file_path = if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = elems[0] {
            s.value()
        } else {
            panic!("Expected first argument to be a string literal for file path")
        };
        let scheme = if elems.len() >= 2 {
            if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = elems[1] {
                let scheme_val = s.value();
                match scheme_val.as_str() {
                    "json" | "postcard" => scheme_val,
                    _ => panic!("Unsupported deserialization scheme: {}", scheme_val),
                }
            } else {
                panic!("Expected scheme to be a string literal")
            }
        } else {
            guess_scheme(&file_path)
        };
        (file_path, scheme)
    } else {
        if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = args_expr {
            let file_path = s.value();
            let scheme = guess_scheme(&file_path);
            (file_path, scheme)
        } else {
            panic!("Expected a string literal or a tuple expression")
        }
    };

    let bytes = fs::read(&file_path)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", file_path));

    let length = match scheme.as_str() {
        "json" => {
            // parsing to Vec<Value> to get the length, instead of Vec<T>, 
            // is faster and avoids the needs for struct type info T
            let vec: Vec<serde_json::Value> = serde_json::from_slice(&bytes)
                .unwrap_or_else(|_| panic!("Failed to parse JSON array in file: {}", file_path));
            vec.len()
        },
        "postcard" => {
            let vec: Vec<serde_json::Value> = postcard::from_bytes(&bytes)
                .unwrap_or_else(|_| panic!("Failed to parse postcard array in file: {}", file_path));
            vec.len()
        },
        _ => panic!("Unsupported deserialization scheme: {}", scheme),
    };

    TokenStream::from(quote! { #length })
}

/// guess scheme based on file extension
fn guess_scheme<S: AsRef<str>>(file_path: S) -> String {
    let ext = Path::new(file_path.as_ref())
        .extension()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("");
    let scheme = match ext {
        "postcard" | "pc" => "postcard".to_string(),
        "json" => "json".to_string(),
        _ => "json".to_string(),
    };
    scheme
}