use crate::app::{Structure, StructureIter};
use strum::IntoEnumIterator;
use zoon::named_color::*;
use zoon::{println, *};

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
        .child(add_menu())
}

fn add_menu() -> impl Element {
    El::new()
        .s(RoundedCorners::all(9))
        .s(Width::exact(100))
        .s(Height::exact(100))
        .s(Background::new().color(GREEN_0))
        .s(Transform::with_signal(
            super::add_position()
                .signal()
                .map(|(x, y)| Transform::new().move_right(x).move_down(y)),
        ))
}

fn add_node_button(name: &str) -> impl Element {
    Button::new()
        .s(RoundedCorners::all(9))
        .label(Text::new(name))
        .on_click(|| {})
}
