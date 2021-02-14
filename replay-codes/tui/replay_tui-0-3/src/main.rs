extern crate tui;

fn main() {
    let rect = tui::layout::Rect::new(56880 ,12336 ,12336 ,12336);
    let mut buffer = tui::buffer::Buffer::empty(rect);
    let _ = tui::buffer::Buffer::get_mut(&mut buffer ,12336 ,12336);
}

//Not real bug! Wrong API use
