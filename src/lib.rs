extern crate proc_macro;
use proc_macro::*;

mod space;
use space::*;

const SPACE: (usize, usize, usize) = (3, 0, 0); // 3D Vectorspace Geometric Algebra

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
    eq_macro_logic(SPACE, tokens)
}

#[proc_macro]
pub fn eq_peek(tokens: TokenStream) -> TokenStream {
    eq_macro_logic_peek(SPACE, tokens)
}