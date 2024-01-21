struct Wrapper {
    v: Vec<i32>,
}

impl Wrapper {
    fn iter(self) -> WrapperIterator {
        return self.into_iter();
    }
}

impl IntoIterator for Wrapper {
    type Item = i32;
    type IntoIter = WrapperIterator;
    fn into_iter(self) -> Self::IntoIter {
        WrapperIterator {
            v: self.v,
            index: -1,
        }
    }
}

struct WrapperIterator {
    v: Vec<i32>,
    index: i32,
}

impl Iterator for WrapperIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if &self.index % 2 == 0 && self.v.len() > self.index as usize {
            Some(self.v[self.index as usize])
        } else if self.index % 2 != 0 {
            self.next()
        } else {
            None
        }
    }
}

pub fn test1() {
    let w = Wrapper {
        v: vec![1, 2, 3, 4, 5, 6, 7, 8],
    };
    let mut iw = w.iter();
    println!("first: {}", iw.next().unwrap());
    for el in iw {
        println!("evens: {}", el);
    }
}

pub fn test2() {
    let w = Wrapper {
        v: vec![11, 12, 13, 14, 15, 16, 17, 18],
    };
    let mut iw = w.iter();
    println!("first: {}", iw.next().unwrap());
    for el in iw {
        println!("evens: {}", el);
    }
}
