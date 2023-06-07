#![warn(missing_debug_implementations, rust_2018_idioms)]

#[derive(Debug)]
pub struct StrSplit<'heystack, D> {
    remainder: Option<&'heystack str>, //short lifetime 
    delimiter: D,
}
/*  str -> [char] Collection of chars, we don't know how long its
    &str -> &[char] Point for a sequence of characters
    String -> Vec<char> heap-allocated

    String -> &str (cheap -- AsRef)
    &str -> String (expensive -- mempcy)
*/

impl<'heystack, D> StrSplit<'heystack, D> { //type inference
    //haystack, usually the thing you are search in
    //delimiter, the thing we are spliting by
    pub fn new (haystack: &'heystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }  
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'heystack, D> Iterator for StrSplit<'heystack, D> 
where 
    D: Delimiter 
{
    type Item = &'heystack str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        // impl<T> Option<T> { fn as_mut(&mut self) -> Option<&mut T> }
        
        //Try to use match {} 
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(&remainder) { 
            let until_delimiter = &remainder[..delim_start];
            //extract the stuff until the next row delimiter and then we set 
            //the remainder to be everything passed that remainder
            *remainder = &remainder[delim_end..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
            // impl<T> Option<T> {fn take(&mut self) -> Option<T> }
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))    
    }
}

// We could do better
impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))    
    }
}

pub fn until_char(s: &str, c: char) -> &'_ str { //it's not required lifetime 
    let delim = format!("{}", c);
    StrSplit::new(s, &*delim)
        .next()
        .expect("StrSplit always gives at least one result")
}
            
/* 
    self.remainder = ""; 
    static lifetime, during until the end of the program
    
    &'a str -> &'static str => ok, that's fine
    &'static str -> &'a str => not ok

*/
#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world",'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
