extern crate regex;

fn main() {
    let regex_ = regex::Regex::new("(?-u)0|\\W").unwrap();
    let capture_ = regex::Regex::captures(&regex_ ,"〧000000").unwrap();
    let mut escape_ = regex::escape("000000000");
    let _ = regex::Captures::expand(&capture_ ,"0$0000000" ,&mut escape_);
}
