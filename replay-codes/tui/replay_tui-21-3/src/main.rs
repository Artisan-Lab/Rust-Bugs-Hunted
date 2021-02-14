extern crate tui;

fn main() {
    let rect1 = tui::layout::Rect::new(12336 ,12336 ,12336 ,12336);
    let rect2 = tui::layout::Rect::new(12336 ,65328 ,12336 ,12336);
    let _ = tui::layout::Rect::intersection(rect1 ,rect2);
}

//another data may also cause panics:
//12336 12336 12336 12336 13360 12336 12336 12336
//12336 12336 12336 12336 12336 15152 12336 12336