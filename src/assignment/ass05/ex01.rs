pub trait Printable {
    fn print(&self);
}

impl Printable for i32 {
    fn print(&self) {
        print!("{}", self);
    }
}

impl Printable for String {
    fn print(&self) {
        print!("{}", self);
    }
}

impl<T> Printable for Vec<T>
where
    T: Printable,
{
    fn print(&self) {
        for item in self {
            item.print();
            print!(" ");
        }
    }
}

pub fn print<T>(a: T)
where
    T: Printable,
{
    a.print();
}
