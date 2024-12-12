//! Module to tokenize raw markdown file input.

use std::rc::Rc;

pub struct TokenNode<'a> {
    pub token: Token<'a>,
    pub next: Option<Rc<TokenNode<'a>>>,
}

impl<'a> TokenNode<'a> {
    pub fn new(token: Token<'a>) -> Self {
        Self { token, next: None }
    }
}

pub struct TokenStream<'a> {
    first: Rc<Option<TokenNode<'a>>>,
    last: Rc<Option<TokenNode<'a>>>,
}

impl<'a> TokenStream<'a> {
    pub fn new() -> Self {
        Self {
            first: Rc::new(None),
            last: Rc::new(None),
        }
    }
}

pub enum Token<'a> {
    ToggleItalicBold,
    ToggleBold,
    ToggleItalic,
    Super(&'a str),
    Sub(&'a str),
    Header1,
    Header2,
    Header3,
    Header4,
    Header5,
    UlItem,
    OlItem,
}

pub fn tokenize(text: &str) -> TokenStream {
    todo!()
}
