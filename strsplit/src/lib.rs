//! <- haha this is doc for crate
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

// impl StrSplit<'_> {
// anon lifetime '_
//  - guess what lifetime, if there is only one possible guess.

// Pointers we give in, live at least as long as StrSplit
impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    // hmm, what is it?
    // an alias i think
    type Item = &'a str;
    // Basically what we say, is that this `Item` is valid
    // as long as `remainder` is in valid,
    // even if `StrSplit` was already dropped.

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len()..)];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            // TODO: bug
            None
        } else {
            let rest = self.remainder;
            self.remainder = "";
            //   &'a str     &' static str
            Some(rest)
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}
