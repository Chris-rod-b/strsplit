#![warn(missing_debug_implementations, rust_2018_idioms)]

#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>, //short lifetime 
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> { //type inference
    //haystack, usually the thing you are search in
    //delimiter, the thing we are spliting by
    pub fn new (haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }  
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        // impl<T> Option<T> { fn as_mut(&mut self) -> Option<&mut T> }
        
        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delimiter = &remainder[..next_delim];
            //extract the stuff until the next row delimiter and then we set 
            //the remainder to be everything passed that remainder
            *remainder = &remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
            // impl<T> Option<T> {fn take(&mut self) -> Option<T> }
        }
    }
}
/* 
    self.remainder = ""; 
    static lifetime, during until the end of the program
    
    &'a str -> &'static str => ok, that's fine
    &'static str -> &'a str => not ok

*/
            
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
