use strum::{Display, EnumIter};
use zoon::named_color::{GREEN_7, GREEN_8};
use zoon::*;

pub(crate) mod view;

enum Structure {
    Vec3d(f64, f64, f64),
    Vec2d(f64, f64),
    Scalar(f64),
}

#[static_ref]
fn add_position() -> &'static Mutable<(i32, i32)> {
    Mutable::new((0, 0))
}

#[static_ref]
fn show_add() -> &'static Mutable<bool> {
    Mutable::new(false)
}
