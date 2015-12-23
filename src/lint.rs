use std::io::Read;
use std::collections::HashSet;

use tendril::{ByteTendril, ReadExt, StrTendril};
use html5ever::tokenizer::{TokenSink, Token, Tokenizer, TokenizerOpts};

use errors::LinkCheckerError;

#[derive(Debug)]
struct Links {
    urls: HashSet<StrTendril>,
    anchors: HashSet<StrTendril>,
    ids: HashSet<StrTendril>,
}

impl TokenSink for Links {
    fn process_token(&mut self, token: Token) {
        if let Token::TagToken(tag) = token {
            for attr in tag.attrs {
                if attr.name.local == "href".into() {
                    if attr.value.starts_with("#") {
                        self.anchors.insert(attr.value[1..].into());
                    } else {
                        self.urls.insert(attr.value);
                    }
                } else if attr.name.local == "id".into() || attr.name.local == "name".into() {
                    self.ids.insert(attr.value);
                }
            }
        }
    }
}

pub fn lint_html_links<R: Read>(html: &mut R) -> Result<(), LinkCheckerError> {
    let mut input = ByteTendril::new();
    try!(html.read_to_tendril(&mut input));
    let input = try!(input.try_reinterpret().map_err(|_| LinkCheckerError::Read));

    let sink = Links {
        urls: HashSet::new(),
        anchors: HashSet::new(),
        ids: HashSet::new(),
    };

    let mut tok = Tokenizer::new(sink, TokenizerOpts::default());
    for s in Some(input) {
        tok.feed(s);
    }
    tok.end();
    let sink = tok.unwrap();

    // println!("{:#?}", sink);

    if sink.anchors.is_subset(&sink.ids) {
        Ok(())
    } else {
        Err(sink.anchors
                .difference(&sink.ids)
                .cloned()
                .collect::<HashSet<StrTendril>>())
            .map_err(LinkCheckerError::MissingLinks)
    }
}
