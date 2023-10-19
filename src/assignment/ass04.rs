use std::collections::LinkedList;
use std::fmt::{Debug, Display, Formatter};
use std::os::unix::raw::pid_t;

// Es1
use rand::distributions::{Distribution, Uniform};
use rand::{random, Rng};

pub fn find_equal<'a, 'b>(s1: &'a str, s2: &'b str) -> Option<(&'a str, &'b str)>{
    let mut index1 = 0;
    while index1 < s1.len()-1 {
        let (a1, b1) = s1.split_at(index1);
        let (c1, d1) = b1.split_at(2);
        let mut index2 = 0;
        while index2 < s2.len() -1 {
            let (a2, b2) = s2.split_at(index2);
            let (c2, d2) = b2.split_at(2);
            if c1.contains(c2) {
                return Some((c1.clone(), c2.clone()));
            }
            index2 += 1;
        }
        index1 += 1;
    }

    return None;
}

pub fn lucky_slice<'a>(input_str: &'a str) -> Option<&'a str> {
    let mut string : String = String::new();
    let mut index1 = 0;
    while index1 < input_str.len() {
        let mut n : u8 = random();
        while n < 'a' as u8 {
            n += 'a' as u8;
        }
        while n > 'z' as u8 {
            n -= ('z' as u8 - 'a' as u8);
        }
        let c = n as char;
        string.push(c);
        index1 += 1;
    }

    match find_equal(input_str, string.as_str()) {
        None => {return None}
        Some((s, _)) => {
            println!("{}", s);
            let ret = s.clone();
            return Some(ret);
        }
    }
}

// Es2
pub struct Person<'a> {
    name: String,
    father: Option<&'a Person<'a>>,
    mother: Option<&'a Person<'a>>,
}

impl Display for Person<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Debug for Person<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Person<'_> {
    pub fn new<'a>(name: &str, father: Option<&'a Person>, mother: Option<&'a Person>) -> Person<'a> {
        return Person{name: name.to_string(), father: father, mother: mother};
    }

    pub fn find_relatives<'a>(&'a self, generations: u32) -> Vec<&'a Person> {
        let mut vec : Vec<&'a Person> = Vec::new();
        if generations == 0 {
            vec.push(&self);
            return vec;
        } else {
            vec.push(&self);
            let mut v1 : Vec<&Person> = Vec::new();
            let mut v2 : Vec<&Person> = Vec::new();
            if self.father.is_some() {
                v1 = self::Person::find_relatives(self.father.unwrap(), generations - 1);
                for p in v1.iter() {
                    vec.push(p);
                }
            }
            if self.mother.is_some() {
                v2 = self::Person::find_relatives(self.mother.unwrap(), generations - 1);
                for p in v2.iter() {
                    vec.push(p);
                }
            }
            return vec;
        }
    }

    pub fn find_roots<'a>(&'a self) -> Vec<&'a Person> {
        let mut vec : Vec<&Person> = Vec::new();
        let mut insert : bool = false;

        if self.mother.is_some() {
            let v1 = Person::find_roots(self.mother.unwrap());
            for p in v1.iter() {
                vec.push(p);
            }
        } else {
            insert = true;
        }

        if self.father.is_some() {
            let v2 = Person::find_roots(self.father.unwrap());
            for p in v2.iter() {
                vec.push(p);
            }
        } else {
            insert = true;
        }

        if insert {
            vec.push(self);
        }
        return vec;
    }
}

// fn main() {
//     let nonno_mamma = Person::new("Severino", None, None);
//     let nonna_mamma = Person::new("Augusta", None, None);
//     let nonno_papà = Person::new("Marco", None, None);
//     let nonna_papà = Person::new("Maddalena", None, None);
//     let papà = Person::new("Massimo", Some(&nonna_papà), Some(&nonno_papà));
//     let mamma = Person::new("Bruna", Some(&nonna_mamma), Some(&nonno_mamma));
//     let io = Person::new("Federico", Some(&papà), Some(&mamma));
//     println!("{:?}",(nonno_mamma.find_relatives(3)));
//     println!("{:?}",(io.find_relatives(3)));
//     println!("{:?}",(io.find_roots()));
// }

// Es5
pub trait SplitCustom<'a, ReturnType: 'a> {
    fn split_trait(&'a self) -> (ReturnType, ReturnType);
}

impl<'a> SplitCustom<'a, &'a str> for String {
    fn split_trait(&self) -> (&str, &str) {
        return self.split_at(self.len()/2);
    }
}

impl<'a> SplitCustom<'a, &'a [i32]> for &[i32] {
    fn split_trait(&self) -> (&[i32], &[i32]) {
        let mid = self.len()/2;
        let (a, b) = self.split_at(mid);
        return (a, b);
    }
}

impl<'a> SplitCustom<'a, LinkedList<f64>> for LinkedList<f64> {
    fn split_trait(&self) -> (LinkedList<f64>, LinkedList<f64>) {
        //let len = self.len()/2;
        let mut first : LinkedList<f64> = LinkedList::new();
        let mut second : LinkedList<f64> = LinkedList::new();
        let mut index : usize = 0;
        for n in self.iter() {
            if index < self.len()/2 {
                first.push_back(*n);
            } else {
                second.push_back(*n);
            }
            index += 1;
        }

        return (first, second)
    }
}

// fn main() {
//     let string = "0123456789".to_string();
//     println!("{:?}" , string.split_trait());

//     let slice = &[0,1,2,3,4,5,6,7,8,9][..];
//     println!("{:?}", slice.split_trait());
    
//     let mut linked_list:LinkedList<f64> = LinkedList::new();
//     linked_list.push_back(0.0);
//     linked_list.push_back(1.0);
//     linked_list.push_back(2.0);
//     linked_list.push_back(3.0);
//     linked_list.push_back(4.0);
//     linked_list.push_back(5.0);
//     linked_list.push_back(6.0);
//     linked_list.push_back(6.0);
//     linked_list.push_back(8.0);
//     linked_list.push_back(9.0);
//     println!("{:?}", linked_list.split_trait());
// }