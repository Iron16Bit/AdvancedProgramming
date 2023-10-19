struct Point<T, U> {
    x: T,
    y: U,
}

struct Circle<T, U> {
    radius: T,
    center: U,
}

struct Rectangle<T, U> {
    top_left: T,
    bottom_right: U,
}

impl Default for Point {
    fn default()
}