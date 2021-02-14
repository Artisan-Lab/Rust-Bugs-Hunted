extern crate time;

fn main() {
    let date_ = time::Date::try_from_yo(-31 ,136).unwrap();
    let date2_ = time::Date::next_day(date_);
    let _ = time::Date::iso_year_week(date2_);

}

//-2768 94
//-31 136
//-118 48