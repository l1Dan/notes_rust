pub mod chat;
pub mod tools;
pub mod ui;

use crate::{
    chat::message::Message,
    tools::geometry::{Point, Rect, Size},
    ui::view::View,
};

fn main() {
    let mut message = Message::new();
    message.print_message();
    message.set_content("hello programmer".to_string());
    message.print_message();

    let rect = Rect {
        origin: Point::new(10.0, 10.0),
        size: Size::new(10.0, 10.0),
    };
    let mut view = View::default();
    println!("view: {:?}", view);

    view.set_frame(rect);
    println!("view: {:?}", view);

    let rect = Rect::default();
    view.set_bounds(rect);
    println!("view: {:?}", view);
}
