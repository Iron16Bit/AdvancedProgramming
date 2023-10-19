pub fn modify_odd(arr : &mut [i32]) {
    for n in arr {
        if *n%2 != 0 {
            *n = 0;
        }
    }
}

pub fn create_vec() {
    let mut v : Vec<i32> = Vec::new();
    let mut i = 0;
    while i <= 100 {
        v.push(i);
        i += 1;
    }

    modify_odd(&mut v);

    println!("{:?}", v);
}

pub fn count_character(s : String) -> HashMap<char, u32> {
    let mut ret : HashMap<char, u32> = HashMap::new();

    for c in s.chars() {
        if ret.contains_key(&c) {
            ret.insert(c, *ret.get(&c).unwrap()+1);
        } else {
            ret.insert(c, 1);
        }
    }
    return ret;
}

pub fn split_at_value(slice : &[i32], value : i32) -> Option<(&[i32], &[i32])> {
    let mut done : bool = false;
    let mut index : usize = 0;
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
        let ret : Option<(&[i32], &[i32])> = Option::None;
        return ret;
    }
}

pub fn sub_slice(a: &Vec<i32>, b: &Vec<i32>) {

    let mut start : usize = a.len()+1;
    let mut finish : usize = a.len()+1;

    for a_n in 0..a.len() {
        if a[a_n] == b[0] {
            let mut found : bool = true;
            for b_n in 0..b.len() {
                if a_n+b_n >= a.len() || b[b_n] != a[a_n+b_n] {
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
        let ret : &[i32] = &a[start..finish];
        println!("{:?}", ret);
    }

}