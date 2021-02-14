extern crate clap;

fn main() {
    let arg_group_ = clap::ArgGroup::with_name("0");
    let _ = clap::ArgGroup::arg(arg_group_ ,"0");
}

//not a bug! wrong use of api