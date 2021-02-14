extern crate tui;

fn main() {
    let rect1 = tui::layout::Rect::new(12336 ,12336 ,12336 ,12336);
    let mut buf1 = tui::buffer::Buffer::empty(rect1);
    let rect2 = tui::layout::Rect::new(12336 ,12336 ,12336 ,12336);
    let buf2 = tui::buffer::Buffer::empty(rect2);
    let _ = tui::buffer::Buffer::merge(&mut buf1 ,&buf2);

}
