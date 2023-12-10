use std::collections::HashMap;
use std::slice::Iter;

pub fn modify_odd(arr: &mut [i32]) {
    for n in arr {
        if *n % 2 != 0 {
            *n = 0;
        }
    }
}

pub fn create_vec() {
    let mut v: Vec<i32> = Vec::new();
    let mut i = 0;
    while i <= 100 {
        v.push(i);
        i += 1;
    }

    modify_odd(&mut v);

    println!("{:?}", v);
}

pub fn count_character(s: String) -> HashMap<char, u32> {
    let mut ret: HashMap<char, u32> = HashMap::new();

    for c in s.chars() {
        if ret.contains_key(&c) {
            ret.insert(c, *ret.get(&c).unwrap() + 1);
        } else {
            ret.insert(c, 1);
        }
    }
    return ret;
}

pub fn split_at_value(slice: &[i32], value: i32) -> Option<(&[i32], &[i32])> {
    let mut done: bool = false;
    let mut index: usize = 0;
    for n in slice {
        if !done {
            if value == *n {
                done = true;
            } else {
                index += 1;
            }
        }
    }

    if done {
        let (a, b) = slice.split_at(index);
        let ret: Option<(&[i32], &[i32])> = Option::from((a, b));
        return ret;
    } else {
        let ret: Option<(&[i32], &[i32])> = Option::None;
        return ret;
    }
}

pub fn sub_slice(a: &Vec<i32>, b: &Vec<i32>) {
    let mut start: usize = a.len() + 1;
    let mut finish: usize = a.len() + 1;

    for a_n in 0..a.len() {
        if a[a_n] == b[0] {
            let mut found: bool = true;
            for b_n in 0..b.len() {
                if a_n + b_n >= a.len() || b[b_n] != a[a_n + b_n] {
                    found = false;
                }
            }
            if found {
                start = a_n;
                finish = a_n + b.len();
                break;
            }
        }
    }

    if finish > a.len() {
        println!("Not found");
    } else {
        let ret: &[i32] = &a[start..finish];
        println!("{:?}", ret);
    }
}

pub fn max(v: &Vec<i32>) -> i32 {
    let mut ret: i32 = i32::MIN;
    for n in v {
        if *n > ret {
            ret = *n;
        }
    }

    return ret;
}

pub fn swap(v: &mut Vec<i32>) {
    let len = v.len();
    v.swap(0, len - 1);
}

pub fn is_sorted(v: &Vec<i32>) -> bool {
    let mut index: usize = 0;
    while index < v.len() - 1 {
        if v[index] < v[index + 1] {
            return false;
        }
    }

    return true;
}

pub fn insert_if_longer(mut vec: Vec<String>, string: String) -> Vec<String> {
    if string.len() > 10 {
        vec.push(string);
    }
    return vec;
}

pub fn build_vector(iter: Iter<i32>) -> Vec<&i32> {
    let mut ret: Vec<&i32> = Vec::new();
    for n in iter {
        ret.push(n);
    }

    return ret;
}

pub fn flip(vec: &mut Vec<i32>, i: i32) {
    let mut start: i32 = 0;
    let mut finish = i.clone();
    while start < finish {
        vec.swap(start as usize, finish as usize);
        start += 1;
        finish -= 1;
    }
}

pub fn pancake_sorting(vec: &mut Vec<i32>) {
    let mut end: usize = vec.len() - 1;
    while end > 0 {
        let mut max = i32::MIN;
        let mut max_pos: usize = 0;
        for i in 0..end {
            if vec[i] > max {
                max = *vec.get(i).unwrap();
                max_pos = i.clone();
            }
        }
        flip(vec, max_pos as i32);
        flip(vec, (end - 1) as i32);
        end -= 1;
    }
}

pub fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    let mut index_a: usize = 0;
    let mut index_b: usize = 0;
    while index_a < a.len() && index_b < b.len() {
        if a[index_a] < b[index_b] {
            vec.push(a[index_a]);
            index_a += 1;
        } else {
            vec.push(b[index_b]);
            index_b += 1;
        }
    }

    while index_a < a.len() {
        vec.push(a[index_a]);
        index_a += 1;
    }

    while index_b < b.len() {
        vec.push(b[index_b]);
        index_b += 1;
    }

    return vec;
}

pub fn new_vec() {
    #[derive(Debug)]
    enum VecValue {
        Num(i32),
        Str(String),
    }

    let mut vec: Vec<VecValue> = Vec::new();
    vec.push(VecValue::Num(33));
    vec.push(VecValue::Str(String::from("Trentini")));
    vec.push(VecValue::Str(String::from("Entrarono a Trento tutti e")));
    vec.push(VecValue::Num(33));
    vec.push(VecValue::Str(String::from("Trotterellando")));

    println!("{:?}", vec);
}

pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum Expression {
    Number(i32),
    Operation(Box<Expression>, Box<Expression>, Operation),
}

pub fn expr_main() {
    let n1 = Expression::Number(3);
    let n2 = Expression::Number(5);
    let op1 = Operation::Add;
    let exp1 = Expression::Operation(Box::new(n1), Box::new(n2), op1);

    let n3 = Expression::Number(8);
    let op2 = Operation::Mul;
    let exp2 = Expression::Operation(Box::new(exp1), Box::new(n3), op2);

    println!("{:?}", evaluate_expression(exp2));
}

pub fn evaluate_expression(e: Expression) -> Result<i32, String> {
    match e {
        Expression::Number(a) => {
            return Result::Ok(a);
        }
        Expression::Operation(l, r, op) => {
            let left = evaluate_expression(*l)?;
            let right = evaluate_expression(*r)?;
            match op {
                Operation::Add => return Result::Ok(left + right),
                Operation::Sub => return Result::Ok(left - right),
                Operation::Mul => return Result::Ok(left * right),
                Operation::Div => {
                    if right == 0 {
                        return Result::Err(String::from("Division by 0"));
                    } else {
                        return Result::Ok(left / right);
                    }
                }
            }
        }
    }
}
