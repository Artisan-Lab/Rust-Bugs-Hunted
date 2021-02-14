extern crate regex;

fn main() {
    let regex_ = regex::Regex::new("0").unwrap();
    let capture_location = regex::Regex::capture_locations(&regex_);
    let _ = regex::CaptureLocations::get(&capture_location ,9236935819261915184);

}

//other data may cause panic:
//"((()(()())))" 9236935819261915184
//"䊀䊀䊀" 9236935819261915184
//"0" 9236935819261915184
//"(()(()\\[))" 17595616727661555760
//"()" 10822202888096329776