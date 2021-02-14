extern crate url;

fn main() {
    let mut _local0 = url::Url::parse("u:/0/..//").unwrap();
    let _ = url::quirks::set_host(&mut _local0 ,"/00000000");
}
//other inputs should cause crash:
//"l:/.//" ":00000"