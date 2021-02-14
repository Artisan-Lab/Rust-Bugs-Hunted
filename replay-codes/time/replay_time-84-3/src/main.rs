extern crate time;

fn main() {
    let date_ = time::Date::try_from_yo(-208 ,99).unwrap();
    let date2_ = time::Date::try_with_hms_milli(date_, 22, 48,48 ,48).unwrap();
    let _ = time::PrimitiveDateTime::week(date2_);
}

//-19144 128 0 48 48 48
//-19144 57 22 48 48 48
//-19144 289 22 48 48 48
//-208 99 22 48 48 48
//-1280 256 22 48 48 304