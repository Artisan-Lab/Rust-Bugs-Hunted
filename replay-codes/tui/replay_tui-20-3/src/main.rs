extern crate tui;

fn main() {
    let rect1 = tui::layout::Rect::new(12336 ,12336 ,12336 ,12336);
    let rect2 = tui::layout::Rect::new(58928 ,12336 ,12336 ,0);
    let _ = tui::layout::Rect::union(rect1 ,rect2);
}

//other data may cause panic:
//12336 65328 12336 12336 12336 12336 12336 12336
//12336 12336 12336 12336 12336 32816 0 43312