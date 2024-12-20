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

pub struct TokenSequence<'a> {
    first: Option<Rc<TokenNode<'a>>>,
    last: Option<Rc<TokenNode<'a>>>,
}

impl<'a> TokenSequence<'a> {
    pub fn new() -> Self {
        Self {
            first: None,
            last: None,
        }
    }

    pub fn push(&'a mut self, token: Token<'a>) {
        let new: Rc<TokenNode> = Rc::new(TokenNode::new(token));
        if self.last.is_none() {
            self.first = Some(Rc::clone(&new));
        } else {
            let mut last: Rc<TokenNode<'a>> = self.last.clone().unwrap();
            // we can guarantee that we're not reading tokens while writing them,
            // as tokenization is fully completed before the `TokenStream` is provided
            // to the caller.
            unsafe { Rc::get_mut_unchecked(&mut last).next = Some(Rc::clone(&new)) }
        }
        self.last = Some(new);
    }

    pub fn iter(&self) -> TokenSeqIter<'a> {
        TokenSeqIter {
            current: self.first.clone(),
        }
    }
}

pub struct TokenSeqIter<'a> {
    pub current: Option<Rc<TokenNode<'a>>>,
}

impl<'a> Iterator for TokenSeqIter<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let token_opt: Option<Self::Item> = self.current.clone().map(|rc| rc.token);
        self.current = self.current.clone().unwrap().next.clone();
        token_opt
    }
}

#[derive(Clone, Copy)]
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

pub fn tokenize(text: &str) -> TokenSequence {
    let mut stream: TokenSequence = TokenSequence::new();
    todo!()
}
