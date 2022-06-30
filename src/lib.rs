extern crate proc_macro;
use proc_macro::*;

mod algebra;
use algebra::*;

use lazy_static::lazy_static;
use std::{fs, collections::HashMap};

extern crate serde_json;

lazy_static! {
    static ref ALGEBRA: (usize, usize, usize) = {
        if let Ok(str) = fs::read_to_string("algebra.json") {
            let json: HashMap<&str, Vec<usize>> = serde_json::from_str(str.as_str()).expect("The json file couldn't be parsed");

            (json["algebra"][0], json["algebra"][1], json["algebra"][2])
        } else {    
            (3, 0, 0) // 3D Vectorspace Geometric Algebra is the default
        }
    };
}

#[proc_macro]
pub fn len(_tokens: TokenStream) -> TokenStream {
    format!("{}", (2 as usize).pow((ALGEBRA.0 + ALGEBRA.1 + ALGEBRA.2) as u32))
        .as_str()
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn dims(_tokens: TokenStream) -> TokenStream {
    format!("{}", ALGEBRA.0 + ALGEBRA.1 + ALGEBRA.2)
        .as_str()
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn get_tokens(tokens: TokenStream) -> TokenStream {
    let token_str = format!("{:?}", tokens)
        .replace("\"", "\\\"")
        .replace("]", "\\n]")
        .replace("Literal", "\\n    Literal")
        .replace("Punct", "\\n    Punct")
        .replace("Ident", "\\n    Ident")
        .replace("Group", "\\n    Group");

    format!("\"{}\"", token_str).as_str().parse().unwrap()
}

#[proc_macro]
pub fn eq(tokens: TokenStream) -> TokenStream {
    eq_macro_logic(*ALGEBRA, tokens)
}

#[proc_macro]
pub fn eq_peek(tokens: TokenStream) -> TokenStream {
    eq_macro_logic_peek(*ALGEBRA, tokens)
}