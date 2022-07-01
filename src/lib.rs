extern crate proc_macro;
use proc_macro::*;

mod algebra;
use algebra::*;

use lazy_static::lazy_static;
use std::fs;

extern crate serde_json;

use serde::Deserialize;

#[derive(Deserialize)]
struct JSONFILE {
    algebra: (usize, usize, usize),
    labels: Option<Vec<String>>
}

lazy_static! {
    static ref INFO: JSONFILE = {

        if let Ok(str) = fs::read_to_string("algebra.json") {
            let json: JSONFILE = serde_json::from_str(str.as_str()).expect("The json file couldn't be parsed");

            json
        } else {    
            JSONFILE { algebra: (3, 0, 0), labels: None } // 3D Vectorspace Geometric Algebra is the default
        }
    };
}

#[proc_macro]
pub fn len(_tokens: TokenStream) -> TokenStream {
    format!("{}", (2 as usize).pow((INFO.algebra.0 + INFO.algebra.1 + INFO.algebra.2) as u32))
        .as_str()
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn dims(_tokens: TokenStream) -> TokenStream {
    format!("{}", INFO.algebra.0 + INFO.algebra.1 + INFO.algebra.2)
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
    eq_macro_logic(INFO.algebra, tokens, &INFO.labels)
}

#[proc_macro]
pub fn eq_peek(tokens: TokenStream) -> TokenStream {
    eq_macro_logic_peek(INFO.algebra, tokens, &INFO.labels)
}