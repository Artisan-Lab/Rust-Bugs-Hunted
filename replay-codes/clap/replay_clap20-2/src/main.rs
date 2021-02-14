extern crate clap;

fn main() {
    let param = "0";
    let _ = clap::Arg::from_usage(param);
}
//debug only
//No real bug. Wrong Api use
