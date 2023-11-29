use std::fmt;
use std::fmt::{Formatter, write};
use crate::provette::prov1::AirplaneCompany::Boeing;

//Es1:
pub fn prev_str(s: &str) -> String {
    let mut ret : String = String::from("");
    for c in s.chars() {
        let mut tmp = c.clone();
        if c.is_alphabetic() {
            if c != 'A' && c != 'a' {
                tmp = char::from((c as u8) - 1);
            }
        }
        ret.push(tmp);
    }

    return ret;
}

//Es2:
pub struct X {
    s: Option<String>,
    i: i32,
}

pub trait Y {
    fn new(st: &str, num: i32) -> X;
    fn take_str(&mut self) -> String;
}

impl Y for X {
    fn new(st: &str, num: i32) -> X {
        let val : X = X{s: Option::from(st.to_string()), i: num};
        return val;
    }

    fn take_str(&mut self) -> String {
        let str = self.s.clone().unwrap();
        self.s = None;
        return str;
    }
}

pub fn main() {
    let lollo : &str = "ciao";
    let mut val : X = X::new(lollo, 2);
    println!("{}", t = val.take_str());
}

//Es3:
pub struct NameSurname {
    name: String,
    surname: String,
}

pub fn replace_surname(mut person: NameSurname, mut surname: String) -> String {
    std::mem::swap(&mut person.surname, &mut surname);
    return surname;
}

//Es4:
struct Student {
    name: String,
    id: u32,
}

impl Student {
    fn new(str: &str, num: u32) -> Student {
        let stud : Student = Student{name: str.to_string(), id: num};
        return stud;
    }
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {},\n Id: {}", self.name, self.id)
    }
}

impl Clone for Student {
    fn clone(&self) -> Self {
        let name = &self.name;
        return Student{name: name.to_string(), id: self.id};
    }
}

struct University {
    name: String,
    students: Vec<Student>,
}

trait Esse3 {
    fn new(str: &str, students: &[Student]) -> University;
    fn remove_student(&mut self, id: u32) -> Result<Student, &str>;
}

impl Esse3 for University {
    fn new(str: &str, students: &[Student]) -> University {
        let mut vec : Vec<Student> = Vec::new();
        let mut iter = students.iter();
        for stud in iter {
            let tmp : Student = Student::new(stud.name.as_str(), stud.id);
            vec.push(tmp);
        }
        let uni : University = University{name: str.to_string(), students: vec};
        return uni;
    }

    fn remove_student(&mut self, id: u32) -> Result<Student, &str> {
        let mut index : usize = 0;
        for stud in self.students.iter() {
            if stud.id == id {
                let ret = stud.clone();
                self.students.remove(index);
                return Ok(ret);
            }
            index += 1;
        }
        return Err("Student not found")
    }
}

impl fmt::Display for University {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut str = String::from("");
        for stud in self.students.iter() {
            let tmp = stud.clone();
            str += (tmp.name + " -> " + tmp.id.to_string().as_str() + "\n").as_str();
        }
        write!(f, "University name: {},\n\n List of students:\n {}", self.name, str)
    }
}

pub fn main_uni() {
    let s01 : Student = Student::new("Alberto", 226899);
    let s02 : Student = Student::new("Salvatore", 227202);
    let arr : [Student; 2] = [s01, s02];
    let mut uni : University = University::new("UniTn", &arr);
    println!("{}", uni);
    let goodbye = uni.remove_student(227202).unwrap();
    println!("Killed in action: {}", goodbye);
}

//Es5
#[derive(Debug)]
enum AirplaneCompany {
    Airbus,
    Boeing,
}
#[derive(Debug)]
struct Airplane {
    company: AirplaneCompany,
    model: String,
}
#[derive(Debug)]
struct AirFleet {
    planes: Vec<Airplane>,
}

impl Clone for Airplane {
    fn clone(&self) -> Self {
        let model = &self.model;
        let comp = &self.company;
        match comp {
            AirplaneCompany::Airbus => {Airplane{company: AirplaneCompany::Airbus, model: model.to_string()}}
            AirplaneCompany::Boeing => {Airplane{company: Boeing, model: model.to_string()}}
        }
    }
}

impl AirFleet {
    fn remove_boeing(&mut self) {
        let mut end : bool = false;
        let mut found : bool = false;
        while !end {
            let mut index : usize = 0;
            let mut to_remove : usize = 0;
            for plane in self.planes.iter() {
                match plane.company {
                    AirplaneCompany::Airbus => {}
                    Boeing => {to_remove = index.clone();
                                found = true;}
                }
                index += 1;
            }
            if !found {
                end = true;
            } else {
                found = false;
                self.planes.remove(to_remove);
            }
        }
    }

    fn add_airplane(&mut self, comp: AirplaneCompany, model: String) {
        self.planes.push(Airplane{company: comp, model: model});
    }

    fn search_airplane(&self, model: String) -> Result<AirplaneCompany, String> {
        for plane in self.planes.iter() {
            let tmp = plane.clone();
            if tmp.model == model {
                return Ok(tmp.company);
            }
        }
        return Err("Airplane not found".to_string());
    }
}

pub(crate) fn main_airplane() {
    let plane1 = Airplane{company: Boeing, model: "737".to_string()};
    let plane2 = Airplane{company: Boeing, model: "777".to_string()};
    let plane3 = Airplane{company: AirplaneCompany::Airbus, model: "123".to_string()};

    let mut vec : Vec<Airplane> = Vec::new();
    vec.push(plane1);
    vec.push(plane3);
    vec.push(plane2);
    let mut jfk = AirFleet{planes: vec};

    println!("{:?}", jfk);
    jfk.remove_boeing();
    println!("{:?}", jfk);
    jfk.add_airplane(Boeing, "737".to_string());
    println!("{:?}", jfk);
    let ret = jfk.search_airplane("737".to_string());
    println!("{:?}", ret.unwrap());

    modsum::main11();
}

//Es6
mod unumber {
    pub type Unumber = usize;
}

mod hashmaps {
    use std::collections::HashMap;
    use crate::provette::prov1::unumber::Unumber;

    pub struct Maps {
        pub map: HashMap<Unumber, String>,
    }
}

pub mod module {
    use std::collections::HashMap;
    use crate::provette::prov1::unumber::Unumber;
    use super::hashmaps;
    pub fn string_to_tuple(m: hashmaps::Maps) -> HashMap<Unumber, (Unumber, String)> {
        let mut ret = HashMap::new();
        for i in m.map.iter() {
            let key = i.0.clone();
            let str = i.1.clone();
            ret.insert(key, (str.len() as Unumber, str));
        }
        return ret;
    }
}

//Es7
struct Size {
    width: u32,
    height: u32,
}

impl Size {
    fn new(w: u32, h: u32) -> Self {
        return Size{width: w, height: h};
    }

    fn area(&self) -> u32 {
        return self.height*self.width;
    }

    fn compare(&self, s2: &Size) -> Option<bool> {
        let a1 = self.area();
        let a2 = s2.area();

        if a1 == a2 {
            return Option::None;
        } else if a1 > a2 {
            return Option::Some(true);
        } else {
            return Option::Some(false);
        }
    }
}

//Es8
struct MaybePoint {
    x: Option<i32>,
    y: Option<i32>,
}

impl MaybePoint {
    fn new(x: Option<i32>, y: Option<i32>) -> Self {
        return MaybePoint{x: x, y: y};
    }

    fn is_some(&self) -> bool {
        if self.x.is_some() || self.y.is_some() {
            return true;
        }
        return false;
    }

    fn maybe_len(&self) -> Option<f32> {
        if !self.is_some() {
            return Option::None;
        }
        let len : f32 = f32::sqrt(i32::pow(self.x.unwrap(), 2) as f32 + i32::pow(self.y.unwrap(), 2) as f32);
        return Option::Some(len);
    }
}

//Es9
pub fn res1(n: i32) -> Result<i32, String> {
    if n%10 == 0 {
        return Result::Ok(n);
    }
    return Result::Err("error".to_string());
}

pub fn res2(a: Result<i32, String>) -> Result<i32, String> {
    match a {
        Ok(n) => {
            if n%5 == 0 {
                return Ok(n);
            }
        }
        Err(_) => {}
    }
    return Err("error".to_string());
}

pub fn wrapper(n: i32) -> Result<i32, String> {
    let ret = res2(res1(n));
    if ret.is_ok() {
        return Ok(n);
    }
    return Err("error".to_string());
}

//Es10
pub fn order(v: Vec<String>) -> Vec<String> {
    let mut ret : Vec<String> = Vec::new();
    let mut index = 0;
    for s in v.iter() {
        ret.push(index.to_string() + " - " + s.as_str());
        index+=1;
    }
    return  ret;
}

//Es11
mod modx {
    pub(crate) enum S {
        S(char),
        C(String),
    }
}
mod mody {
    pub(crate) enum S {
        F(f64, usize),
    }
}
pub mod modsum {
    use super::modx;
    use super::mody;
    pub(crate) fn sum(XX: modx::S, XY: mody::S) -> f64 {
        match XX {
            modx::S::S(c) => {
                let mut ret = c as u8;
                match XY { mody::S::F(a, b) => {
                    let prod = a * (b as f64);
                    return (ret as f64) + prod;
                } };
            }
            modx::S::C(s) => {
                let mut ret = s.len();
                match XY { mody::S::F(a, b) => {
                    let prod = a * (b as f64);
                    return (ret as f64) + prod;
                } };
            }
        }
    }
    pub(crate) fn main11() {
        let v1 : modx::S = modx::S::S('g');
        let v2 : mody::S = mody::S::F(2.5, 1);
        let v3 : modx::S = modx::S::C("giovanni".to_string());
        println!("{}", sum(v3, v2));
    }
}