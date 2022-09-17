#[derive(Copy, Clone)]
pub(crate) enum Data {
    Vec3d(f64, f64, f64),
    Vec2d(f64, f64),
    Scalar(f64),
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
pub(crate) struct Node {
    data: Data,
    pos: Position,
}

impl Node {
    pub(crate) fn new(data: Data, x: i32, y: i32) -> Self {
        Self {
            data,
            pos: Position { x, y },
        }
    }
}
