extern crate time;

fn main() {
    let date_ = time::Date::try_from_yo(-90 ,8).unwrap();
    let datetime_ = time::Date::try_with_hms_micro(date_ ,0 ,48 ,48 ,12336).unwrap();
    let _ = time::PrimitiveDateTime::sunday_based_week(datetime_);
}

//-32440 157 0 48 48 12336
//-120 200 0 48 48 12336
//-120 340 3 48 48 12336
//-90 8 0 48 48 12336