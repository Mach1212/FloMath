use crate::lib::node::Data;
use zoon::named_color::*;
use zoon::{println, *};
use crate::app::nodes;

pub(crate) fn root() -> impl Element {
    El::new()
        .s(Width::percent(100))
        .s(Height::percent(100))
        .s(Background::new().color(GRAY_0))
        .on_pointer_up(|| super::show_add().set_neq(false))
        .on_pointer_down(|| super::show_add().set_neq(true))
        .on_pointer_move_event(|event| {
            if super::show_add().get() == true {
                super::add_position().set((event.x(), event.y()))
            }
        })
        .child(add_menu()).child(zoon::).child_signal(nodes().lock_mut().iter().map(|node| {

    }))
}

fn add_menu() -> impl Element {
    Column::new()
        .s(RoundedCorners::all(9))
        .s(Width::exact(100))
        .s(Height::exact(100))
        .s(Background::new().color(GREEN_0))
        .s(Transform::with_signal(
            super::add_position()
                .signal()
                .map(|(x, y)| Transform::new().move_right(x).move_down(y)),
        ))
        .item(add_node_button("Vec3d", Data::Vec3d(0.0, 0.0, 0.0)))
}

fn add_node_button(name: &str, data: Data) -> impl Element {
    Button::new()
        .s(RoundedCorners::all(9))
        .label(Text::new(name))
        .on_click(|| )
}
