use std::cmp::min;
use std::fmt::{Debug, Display};

pub fn restricted<T, U>(t1: T, t2: T, u: U)
where
    T: Debug + Ord,
    U: Display,
{
    println!("minor: <{:?}>", min(t1, t2));
    println!("u: <{}>", u);
}
