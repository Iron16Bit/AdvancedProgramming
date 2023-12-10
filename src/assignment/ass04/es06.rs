use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

#[derive(Debug)]
pub struct Circle {
    radius: i32,
    center: Point,
}

#[derive(Debug)]
pub struct Rectangle {
    pub(crate) top_left: Point,
    pub(crate) bottom_right: Point,
}

impl Default for Point {
    fn default() -> Self {
        return Point { x: 0, y: 0 };
    }
}

impl Default for Circle {
    fn default() -> Self {
        return Circle {
            radius: 1,
            center: Point::default(),
        };
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        return Rectangle {
            top_left: Point { x: -1, y: 1 },
            bottom_right: Point { x: 1, y: -1 },
        };
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        return Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

pub struct Area {
    area: f32,
}

impl Default for Area {
    fn default() -> Self {
        return Area { area: 0.0 };
    }
}

impl Display for Area {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Area is {}cm^2", self.area)
    }
}

pub trait GetArea<T> {
    fn get_area(&self) -> Area;
}

impl GetArea<Point> for Point {
    fn get_area(&self) -> Area {
        return Area::default();
    }
}

impl GetArea<Circle> for Circle {
    fn get_area(&self) -> Area {
        let pi = 3.14;
        return Area {
            area: (self.radius as f32) * 2.0 * pi,
        };
    }
}

impl GetArea<Rectangle> for Rectangle {
    fn get_area(&self) -> Area {
        let mut base = self.bottom_right.x - self.top_left.x;
        base = base.abs();

        let mut height = self.top_left.y - self.bottom_right.y;
        height = height.abs();

        return Area {
            area: (base * height) as f32,
        };
    }
}

impl Add for Area {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        return Area {
            area: self.area + rhs.area,
        };
    }
}

impl<T> Add<&dyn GetArea<T>> for Area {
    type Output = Self;

    fn add(self, rhs: &dyn GetArea<T>) -> Self::Output {
        return Area {
            area: self.area + rhs.get_area().area,
        };
    }
}

pub fn sum_area<T>(a: &[&dyn GetArea<T>]) -> Area {
    let mut val: f32 = 0.0;
    for f in a.iter() {
        val += f.get_area().area;
    }
    return Area { area: val };
}

// fn main() {
//     let p : Point = Point::default();
//     let p1 : Point = Point{x: 1, y: 2};
//     let p2 : Point = Point{x: 1, y: -2};
//     println!("{:?}", p-p2);
//
//     let r : Rectangle = Rectangle{top_left: p1, bottom_right: Point::default()};
//     println!("{}", r.get_area());
// }
