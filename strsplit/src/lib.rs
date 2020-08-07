//! <- haha this is doc for crate
// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}
// str  -similar> [char] // it doesn't know it's length
// &str -> &[char] // fat (not narrow) pointer, knows where slice start is
//                 // as well as it's length.
// String -> Vec<char> // heap alloc, can shrink and grow
//
// String -to> &str   (cheap -- AsRef)
// &str   -to> String (expensive -- memcpy)

// impl StrSplit<'_> {
// anon lifetime '_
//  - guess what lifetime, if there is only one possible guess.

// Pointers we give in, live at least as long as StrSplit
impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

// impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter>
//
// We don't care about delimiter lifetime
impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
    //
    //
    // We do not need this, in fact it's opposite of what we need
    // Just for example
    // where
    //     'delimiter: 'haystack, // basically 'delimiter > 'haystack
{
    // hmm, what is it?
    // an alias i think
    type Item = &'haystack str;
    // Basically what we say, is that this `Item` is valid
    // as long as `remainder` is in valid,
    // even if `StrSplit` was already dropped.

    fn next(&mut self) -> Option<Self::Item> {
        // without ref, it will move remainder
        // out of self.remainder.
        // But here we're getting a mutable ref to self.remainder.
        // To modify the existing value.
        /*                  &mut &'a str      Option<&'a str> */
        let remainder = self.remainder.as_mut()?;
        // if let Some(&mut remainder) would mean try to match:
        // what is inside self.remainder with &mut remainder pattern.
        // let Some(&mut remainder) will match Option<&mut T> and remainder will be T
        //
        // ALSO *NEW MAGIC SYNTAX*
        // if let Some(remainder) = &mut self.remainder {
        //                          ^^^^
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            let until_delimiter = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
            // impl<T> Option<T> { fn take(&mut self) -> Option<T> }
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

pub fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
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
