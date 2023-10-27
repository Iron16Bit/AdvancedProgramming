use std::marker::PhantomData;
use rand::random;

#[derive(Debug)]
struct Gate<S> {
   state: PhantomData<S>,
   reason: Option<String>,
}

#[derive(Debug)]
struct Open{}
#[derive(Debug)]
struct Closed{}
#[derive(Debug)]
struct Stopped{}

impl Gate<Closed> {
    fn open(self) -> Result<Gate<Open>, Gate<Stopped>> {
        let prob = random::<u32>();
        if prob%5 == 0 {
            let ret : Gate<Open> = Gate{state: PhantomData{}, reason: None};
            return Ok(ret)
        } else {
            let ret : Gate<Stopped> = Gate{ state: PhantomData{} , reason: Some("Too heavy".to_string())};
            return Err(ret)
        }
    }
}

impl Gate<Open> {
    fn close(self) -> Result<Gate<Closed>, Gate<Stopped>> {
        let prob = random::<u32>();
        if prob%2 == 0 {
            let ret : Gate<Closed> = Gate{ state: PhantomData{}, reason: None };
            return Ok(ret)
        } else {
            let ret : Gate<Stopped> = Gate{state: PhantomData{}, reason: Some("Too heavy".to_string())};
            return Err(ret)
        }
    }
}

impl Gate<Stopped> {
    fn open(self) -> Result<Gate<Open>, Gate<Stopped>> {
        let prob = random::<u32>();
        if prob%3 == 0 {
            let ret : Gate<Open> = Gate{state: PhantomData{}, reason: None};
            return Ok(ret)
        } else {
            let ret : Gate<Stopped> = Gate{ state: PhantomData{}, reason: Some("Too heavy".to_string()) };
            return Err(ret)
        }
    }

    fn close(self) -> Result<Gate<Closed>, Gate<Stopped>> {
        let prob = random::<u32>();
        if prob%3 == 0 {
            let ret : Gate<Closed> = Gate{ state: PhantomData{}, reason: None };
            return Ok(ret)
        } else {
            let ret : Gate<Stopped> = Gate{state: PhantomData{}, reason: Some("Too heavy".to_string())};
            return Err(ret)
        }
    }
}

pub fn gate_main() {
    let castle_gate : Gate<Closed> = Gate{ state: PhantomData{} , reason: None };
    let new_gate_state = castle_gate.open();
    match new_gate_state {
        Ok(g) => {
            println!("{:?}", g)
        }
        Err(g) => {
            println!("{:?}", g)
        }
    }
}