extern crate url;

fn main() {
    let mut _local0 = url::Url::parse("m://").unwrap();
    let mut _local1 = url::Url::path_segments_mut(&mut _local0).unwrap();
    let _ = url::PathSegmentsMut::pop_if_empty(&mut _local1);
}
//other inputs should cause crash:
//"m://"
//"d://\u{bffff}\u{bffff}"
//"m://궶궶궶궶"
//"l://ԶԶ"
//"l://Զ"
//"m://궶"
//"d://\u{bffff}"
