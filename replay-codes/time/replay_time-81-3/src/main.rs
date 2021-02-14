extern crate time;

fn main() {
    let date_ = time::Date::try_from_yo(-60, 64).unwrap();
    let date2_ = time::Date::previous_day(date_);
    let _ = time::Date::week(date2_);
}

//-60 64
//-1536 48
//-27382 256
//-128 169