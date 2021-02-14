extern crate url;

fn main() {
    let mut _local0 = url::Url::parse("o://").unwrap();
    let mut _local1 = url::Url::path_segments_mut(&mut _local0).unwrap();
    let _ = url::PathSegmentsMut::pop(&mut _local1);
}
//other inputs should cause crash:
//"o://\u{e71d}\u{e71d}"
//"o://ڎ"
//"o://\u{5c71d}"
//"o://"
//"o://\u{e71d}"
//"o://\u{e71d}\u{e71d}ڝ\u{e71d}"
//"O://\u{e71d}\u{e71d}\u{e71d}\u{e71d}\u{e71d}"
//"o://ڎڎ"