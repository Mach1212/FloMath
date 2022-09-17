use crate::lib::node::Node;
use zoon::*;

pub(crate) mod view;

#[static_ref]
fn add_position() -> &'static Mutable<(i32, i32)> {
    Mutable::new((0, 0))
}

#[static_ref]
fn show_add() -> &'static Mutable<bool> {
    Mutable::new(false)
}

#[static_ref]
fn nodes() -> &'static MutableVec<Node> {
    MutableVec::new()
}
