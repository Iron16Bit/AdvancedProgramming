use std::fmt;
use std::process::exit;

pub fn is_it_luhn(n: String) -> bool {
    if n.len() <= 1 {
        return false;
    }

    let mut vec: Vec<i32> = Vec::new();
    for c in n.chars() {
        if c != ' ' {
            vec.push(c.to_string().parse::<i32>().unwrap());
        }
    }

    for i in 0..vec.len() {
        if (vec.len() - i) % 2 == 0 {
            let mut tmp = vec.get_mut(i).unwrap();
            *tmp *= 2;
            if *tmp > 9 {
                *tmp -= 9;
            }
            vec[i] = *tmp;
            //println!("Doubled item at {}", i);
        }
        //print!("{}", vec[i]);
    }
    //println!();

    let mut tot = 0;
    for i in 0..vec.len() {
        tot += vec.get(i).unwrap();
    }
    //println!("{}", tot);

    if tot % 10 != 0 {
        return false;
    }

    return true;
}

//Struct for es2:
enum Fuel {
    Diesel,
    Gasoline,
    LPG,
    Methane,
    Electric,
}

//#[derive(Debug)]
pub struct IpV4 {
    pub values: [u32; 3],
}

pub trait Ip4 {
    fn new(&mut self, group0: u32, group1: u32, group2: u32);
    fn change(&mut self, group_number: u32, value: u32) -> Result<(), String>;
}

impl Ip4 for IpV4 {
    fn new(&mut self, group0: u32, group1: u32, group2: u32) {
        if group1 < 256 && group2 < 256 && group0 < 256 {
            self.values[0] = group0;
            self.values[1] = group1;
            self.values[2] = group2;
        } else {
            println!("Group values must be between 0 and 255");
            exit(1);
        }
    }
    fn change(&mut self, group_number: u32, value: u32) -> Result<(), String> {
        if group_number <= 2 {
            if value <= 255 {
                self.values[group_number as usize] = value;
                return Ok(());
            } else {
                return Err(String::from("Group values must be between o and 255"));
            }
        } else {
            return Err(String::from("Group number must be between 0 and 2"));
        }
    }
}

impl fmt::Display for IpV4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            self.values[0], self.values[1], self.values[2]
        )
    }
}

pub struct IpV6 {
    pub(crate) values: [u32; 8],
}

pub trait Ip6 {
    fn new(&mut self, arr: &[u32]);
}

impl Ip6 for IpV6 {
    fn new(&mut self, arr: &[u32]) {
        if arr.len() != 8 {
            println!("Array must contain 8 values");
            exit(1);
        }

        for n in arr {
            if *n > 15 {
                println!("Values must be hexadecimals -> between 0 and 15");
                exit(1);
            }
        }

        let mut i = 0;
        while i < 8 {
            self.values[i] = arr[i];
            i += 1;
        }
    }
}

impl fmt::Display for IpV6 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::from("");
        for n in self.values {
            if n <= 9 {
                str.push(char::from_digit(n, 10).unwrap());
            } else {
                if n == 10 {
                    str.push('A');
                } else if n == 11 {
                    str.push('B');
                } else if n == 12 {
                    str.push('C');
                } else if n == 13 {
                    str.push('D');
                } else if n == 14 {
                    str.push('E');
                } else {
                    str.push('F');
                }
            }
        }
        write!(f, "{}", str)
    }
}

struct Point {
    x: f64,
    y: f64,
    z: f64,
}
