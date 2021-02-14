extern crate time;

fn main() {
    let date_ = time::Date::try_from_yo(-26 ,96).unwrap();
    let _ = time::Date::weekday(date_);
}

//-26 96
//-106 356
//-256 250