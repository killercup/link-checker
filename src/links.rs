use std::io::Read;
use std::collections::HashSet;

use tendril::{ByteTendril, ReadExt, StrTendril};
use html5ever::tokenizer::{TokenSink, Token, Tokenizer, TokenizerOpts};

use errors::LinkCheckerError;

#[derive(Debug)]
pub struct Links {
    urls: HashSet<StrTendril>,
    anchors: HashSet<StrTendril>,
    ids: HashSet<StrTendril>,
}

impl Links {
    pub fn check_missing_anchors(&self) -> Result<(), LinkCheckerError> {
        if self.anchors.is_subset(&self.ids) {
            Ok(())
        } else {
            Err(self.anchors
                    .difference(&self.ids)
                    .cloned()
                    .collect::<HashSet<StrTendril>>())
                .map_err(LinkCheckerError::MissingLinks)
        }
    }

    pub fn get_external_links<'a>(&'a self) -> &'a HashSet<StrTendril> {
        &self.urls
    }
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

pub fn collect_from_html<R: Read>(html: &mut R) -> Result<Links, LinkCheckerError> {
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

    Ok(tok.unwrap())
}
