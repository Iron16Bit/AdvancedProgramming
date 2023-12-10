use std::fmt::{write, Display, Formatter};

pub struct Chair<'a> {
    pub(crate) color: &'a str,
    pub(crate) quantity: &'a usize,
}

struct Wardrobe<'a> {
    color: &'a str,
    quantity: &'a usize,
}

trait Object {
    fn build(&self) -> &str;
    fn get_quantity(&self) -> String;
}

impl Object for Chair<'_> {
    fn build(&self) -> &str {
        return "Chair has been built";
    }

    fn get_quantity(&self) -> String {
        let mut ret = String::from("There are ");
        ret.push_str(&*self.quantity.to_string());
        ret.push_str(" chairs");
        return ret;
    }
}

impl Object for Wardrobe<'_> {
    fn build(&self) -> &str {
        return "Wardrobe has been built";
    }

    fn get_quantity(&self) -> String {
        let mut ret = String::from("There are ");
        ret.push_str(&*self.quantity.to_string());
        ret.push_str(" wardrobes");
        return ret;
    }
}

impl Display for Chair<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.quantity {
            0 => {
                write!(
                    f,
                    "It's so lonely, there isn't a single {} chair",
                    self.color
                )
            }
            1 => {
                write!(f, "Wow! There's a {} chair", self.color)
            }
            a => {
                write!(f, "There are {} {} chairs", a, self.color)
            }
        }
    }
}

impl Display for Wardrobe<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.quantity {
            0 => {
                write!(
                    f,
                    "It's so lonely, there isn't a single {} wardrobe",
                    self.color
                )
            }
            1 => {
                write!(f, "Wow! There's a {} wardrobe", self.color)
            }
            a => {
                write!(f, "There are {} {} wardrobes", a, self.color)
            }
        }
    }
}
