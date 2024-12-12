//! Module to tokenize raw markdown file input.

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

pub fn tokenize<'a>(text: &'a str) -> TokenStream {
    todo!()
}
