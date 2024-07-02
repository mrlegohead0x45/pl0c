use std::io::Read;

// #[derive(Debug)]
pub(crate) struct Tokeniser<'a> {
    input: &'a dyn Read,
}

impl<'a> Tokeniser<'a> {
    pub(crate) fn new(r: &'a dyn Read) -> Self {
        Self { input: r }
    }
}
