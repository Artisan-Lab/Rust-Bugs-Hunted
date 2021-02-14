extern crate time;

fn main() {
    let date_ = time::Date::try_from_ymd(-36 ,11 ,1).unwrap();
    let _ = time::Date::weekday(date_);
}

//-36 11 1
//-62976 2 28
//-128 9 1
//-128 1 20
//-79 10 8