extern crate url;

fn main() {
    let _local1 = url::Url::parse("s:/0/..//").unwrap();
    let _ = url::quirks::username(&_local1);
}

//other inputs should cause crash:
//"t:/.//"