use crate::geometry::Rect;

pub fn collide(r1: &Rect, r2: &Rect) -> Option<Rect> {
    if r1.x0() < r2.x0() && r2.x0() > r1.x1() {
        return None;
    }

    if r2.x0() < r1.x0() && r1.x0() > r2.x1() {
        return None;
    }

    if r1.y0() < r2.y0() && r2.y0() > r1.y1() {
        return None;
    }

    if r2.y0() < r1.y0() && r1.y0() > r2.y1() {
        return None;
    }

    let mut x_array = [r1.x0(), r2.x0(), r1.x1(), r2.x1()];
    let mut y_array = [r1.y0(), r2.y0(), r1.y1(), r2.y1()];
    x_array.sort_by(|a, b| a.partial_cmp(b).unwrap());
    y_array.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let x0 = x_array[1];
    let x1 = x_array[2];
    let y0 = y_array[1];
    let y1 = y_array[2];

    Some(Rect::from_2_points(x0, y0, x1, y1))
}