extern crate regex;

fn main() {
    let regex_ = regex::bytes::Regex::new("0").unwrap();
    let capture_location = regex::bytes::Regex::capture_locations(&regex_);
    let _ = regex::bytes::CaptureLocations::get(&capture_location ,18388250262078763056);

}

//"()()\\[()()" 17163271163433988144
//"()" 9236935819261915184
//"桶" 10533972511944618032
//"\\{()()" 15866234470751285296
//"()()()" 18316192668040835120
//"(())" 9957511759641194544
//"()()()()" 16154464846902997040
//"𓀀" 18388250262078763056
//"\\{" 11686894016551465008
//"ŀ()" 17379443945547771952
//"0" 18388250262078763056
//"(((((((((((((((((((((((((((())))))))))))))))))))))))))))" 11398663640399753264
//"(ŀ)" 14497140184030654512
//"()()()\\[" 17595616727661555760
//"(()(()))" 10750145294058401840
//"()\\[" 11398663640399753264
//"()𓄀𓀀" 18027962291889123376
//"𓀀𓀈" 9236935819261915184