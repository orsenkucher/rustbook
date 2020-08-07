//! <- haha this is doc for crate
// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

// impl StrSplit<'_> {
// anon lifetime '_
//  - guess what lifetime, if there is only one possible guess.

// Pointers we give in, live at least as long as StrSplit
impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
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
        // without ref, it will move remainder
        // out of self.remainder.
        // But here we're getting a mutable ref to self.remainder.
        // To modify the existing value.
        /*                  &mut &'a str      Option<&'a str> */
        if let Some(ref mut remainder) = self.remainder {
            // if let Some(&mut remainder) would mean try to match:
            // what is inside self.remainder with &mut remainder pattern.
            // let Some(&mut remainder) will match Option<&mut T> and remainder will be T
            //
            // ALSO *NEW MAGIC SYNTAX*
            // if let Some(remainder) = &mut self.remainder {
            //                          ^^^^
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..next_delim];
                *remainder = &remainder[(next_delim + self.delimiter.len()..)];
                Some(until_delimiter)
            } else {
                self.remainder.take()
                // impl<T> Option<T> { fn take(&mut self) -> Option<T> }
            }
        } else {
            None
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));

    // or we can
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
