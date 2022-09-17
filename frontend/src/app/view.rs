use crate::lib::node::{Data, Node};
use zoon::named_color::*;
use zoon::{println, *};

pub(crate) fn root() -> impl Element {
    El::new()
        .s(Width::percent(100))
        .s(Height::percent(100))
        .s(Background::new().color(GRAY_0))
        .on_pointer_up(|| super::add_menu_visible().set_neq(false))
        .on_pointer_down(|| super::add_menu_visible().set_neq(true))
        .on_pointer_move_event(|event| super::add_menu_position().set((event.x(), event.y())))
        .child(add_menu())
}

fn add_menu() -> impl Element {
    Column::new()
        .s(RoundedCorners::all(9))
        .s(Width::exact(50))
        .s(Borders::all(Border::new().solid().color(GRAY_9)))
        .s(Background::new().color(GRAY_0))
        .s(Clip::both())
        .s(Visible::with_signal(super::add_menu_visible().signal()))
        .s(Transform::with_signal(
            super::add_menu_visible().signal().map_true(|| {
                let (x, y) = super::add_menu_position().get();
                Transform::new().move_right(x).move_down(y)
            }),
        ))
        .item(add_node_button("Scalar", Data::Scalar(0.0)))
        .item(add_node_button("Vec2d", Data::Vec2d(0.0, 0.0)))
        .item(add_node_button("Vec3d", Data::Vec3d(0.0, 0.0, 0.0)))
}

fn add_node_button(name: &str, data: Data) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Background::new().color_signal(hovered_signal.map_bool(|| GRAY_0, || GRAY_7)))
        .label(name)
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .on_pointer_up(move || {
            let (x, y) = super::add_menu_position().get();
            super::nodes().lock_mut().push(Node::new(data, x, y))
        })
}
