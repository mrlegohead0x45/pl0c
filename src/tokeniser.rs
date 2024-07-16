use std::io::Read;

use anyhow::Result;

use crate::token::Token;

// #[derive(Debug)]

pub(crate) struct Tokeniser<'a> {
    input: &'a dyn Read,
}

impl<'a> Tokeniser<'a> {
    pub(crate) fn new(r: &'a dyn Read) -> Self {
        Self { input: r }
    }
}

impl Iterator for Tokeniser<'_> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = &[0u8];
        // match self.input.read(c) {}
        unimplemented!()
    }
}
