extern crate tui;


fn main() {
    let rect1 = tui::layout::Rect::new(12336 ,12336 ,12336 ,12336);
    let rect2 = tui::layout::Rect::new(57648 ,12336 ,65328 ,48);
    let _ = tui::layout::Rect::intersects(rect1 ,rect2);
}

//other data may cause panic:
//12336 12336 12336 12336 12349 12336 0 65328
//12336 12336 65328 0 12336 12336 12336 12336