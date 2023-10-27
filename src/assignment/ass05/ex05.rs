use std::ops::*;
use std::os::unix::raw::pid_t;

#[derive(Debug, Clone)]
pub struct Pair {
    pair: (i32, String),
}

impl Add<i32> for Pair {
    type Output = Pair;

    fn add(self, rhs: i32) -> Self::Output {
        Pair {pair: (self.pair.0 + rhs, self.pair.1)}
    }
}

impl Sub<i32> for Pair {
    type Output = Pair;

    fn sub(self, rhs: i32) -> Self::Output {
        Pair {pair: (self.pair.0 - rhs, self.pair.1)}
    }
}

impl Add<&str> for Pair {
    type Output = Pair;

    fn add(self, rhs: &str) -> Self::Output {
        let mut new_str = self.pair.1.clone();
        new_str.push_str(rhs);
        Pair {pair: (self.pair.0, new_str)}
    }
}

impl Sub<&str> for Pair {
    type Output = Pair;

    fn sub(self, rhs: &str) -> Self::Output {
        if self.pair.1.contains(rhs) {
            let str = self.pair.1.clone();
            let new_str = str.replace(rhs, "");
            return Pair{pair: (self.pair.0, new_str)}
        } else {
            return Pair{pair: (self.pair.0, self.pair.1)}
        }
    }
}

impl Add<Pair> for Pair {
    type Output = Pair;

    fn add(self, rhs: Pair) -> Self::Output {
        let new_num = (self.clone() + rhs.pair.0).pair.0;
        let new_str = (self.clone() + rhs.pair.1.as_str()).pair.1;
        return Pair{pair: (new_num, new_str)}
    }
}

impl Sub<Pair> for Pair {
    type Output = Pair;

    fn sub(self, rhs: Pair) -> Self::Output {
        let new_num = (self.clone() - rhs.pair.0).pair.0;
        let new_str = (self.clone() - rhs.pair.1.as_str()).pair.1;
        return Pair{pair: (new_num, new_str)}
    }
}

impl Mul<i32> for Pair {
    type Output = Pair;

    fn mul(self, rhs: i32) -> Self::Output {
        let new_num = i32::pow(self.pair.0.clone(), rhs as u32);
        let mut new_str = self.pair.1.clone();
        let old_str = self.pair.1.clone();
        for i in 0..rhs {
            new_str += old_str.clone().as_str();
        }
        return Pair{pair: (new_num, new_str)}
    }
}

pub fn main_es5(){
    let pair = Pair{pair: (2, "C".to_string())};
    let muted_pair = pair * 5;
    println!("{:?}", muted_pair);
}