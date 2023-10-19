use std::collections::HashMap;
use std::hash::Hash;

pub fn string_reverse(s : &str) -> String {
    let mut ret : String = String::from("");
    let mut index = s.len() -1;

    while (index > 0) {
        let c = (s.chars().nth(index));
        ret.push(c.unwrap());
        index -= 1;
    }

    let c = s.chars().nth(index);
    ret.push(c.unwrap());

    return ret;
}

pub fn bigger(a: i32, b: i32) -> i32 {
    if (a > b) {
        return a;
    }
    else {
        return b;
    }
}

pub fn multiply(a : i32, b : f32, c : f64) -> f64 {
    let a_f = a as f64;
    let b_f = b as f64;

    return (a_f*b_f*c);
}

const LIGHT : i64 = 299792458;
pub fn e_equals_mc_squared(m : i32) -> i64 {
    return (m as i64) * LIGHT;
}

pub fn max_min(v : Vec<i32>) -> (i32, i32) {
    let mut min = i32::MAX;
    let mut max = i32::MIN;

    for val in &v {
        if (val < &min) {
            min = *val;
        } else if (val > &max) {
            max = *val;
        }
    }

    return (max, min);
}

pub fn lord_farquaad(s : String) -> String{
    let mut ret = String::from("");
    for c in s.chars() {
        if (c == 'e') {
            ret.push('💥')
        } else {
            ret.push(c);
        }
    }

    return ret;
}

pub fn hashmap_main() {
    let mut furniture : HashMap<String, f32> = HashMap::new();
    furniture.insert(String::from("Sofa"), 299.99);
    furniture.insert(String::from("Cheap chair"), 9.99);
    furniture.insert(String::from("Designer lamp"), 999.73);

    println!("{}", price(&furniture, String::from("Sof")));
}

pub fn price(furniture : &HashMap<String, f32>, f_name : String) -> f32 {
    let val = furniture.get(f_name.as_str());
    if val.is_none() {
        return -1.0;
    } else {
        return *val.unwrap();
    }
}

pub fn append_main() {
    let s : String = String::from("banano");
    res = append(s);

    println!("Original: {}, Appended: {}", s, res);
}

pub fn append(s : String) -> String {
    let mut res = s;
    res.push_str("foobar");

    return res;
}

//Questa sotto non funziona perché abbiamo spostato il valore di s, ma sembra volessero proprio questo
pub fn append_main() {
    let s : String = String::from("banano");
    let res = append(s);

    println!("Original: {}, Appended: {}", s, res);
}

pub fn append(s : String) -> String {
    let mut res = s;
    res.push_str("foobar");

    return res;
}

pub fn is_armstrong(n : i32) -> bool {
    let s : String = n.to_string();
    let len = s.len() as u32;

    let mut number = 0;
    for c in s.chars() {
        let tmp : i32 = (c.to_string().parse().unwrap());
        number += tmp.pow(len);
    }
    println!("{}", number);

    if number == n {
        return true;
    } else {
        return false;
    }
}

