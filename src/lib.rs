extern crate proc_macro;
use proc_macro::*;

mod space;
use space::*;

use lazy_static::lazy_static;
use std::{env, fs};

lazy_static! {
    static ref TEXT_RESULT: Result<String, std::io::Error> = fs::read_to_string("space.json");

    static ref SPACE: (usize, usize, usize) = (3, 0, 0); // 3D Vectorspace Geometric Algebra
}

#[proc_macro]
pub fn len(_tokens: TokenStream) -> TokenStream {
    format!("{}", (2 as usize).pow((SPACE.0 + SPACE.1 + SPACE.2) as u32))
        .as_str()
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn dims(_tokens: TokenStream) -> TokenStream {
    format!("{}", SPACE.0 + SPACE.1 + SPACE.2)
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
    eq_macro_logic(*SPACE, tokens)
}

#[proc_macro]
pub fn eq_peek(tokens: TokenStream) -> TokenStream {
    eq_macro_logic_peek(*SPACE, tokens)
}

#[proc_macro]
pub fn test_file_get(_tokens: TokenStream) -> TokenStream {
    let temp = String::from("NONE");
    let text = if let Ok(str) = &*TEXT_RESULT { str } else { &temp };

    format!("\"{}\"", text.replace("\"", "\\\"")).as_str().parse().unwrap()
}