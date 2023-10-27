use crate::assignment::ass05::ex07::CarrotState::{Burnt, Cooked, Fried, Raw};

trait Heatable {
    fn cook(&mut self);
}

trait Heater {
    fn heat<T>(&self, obj: &mut T)
    where
        T: Heatable;
}

trait Friable {
    fn cook(&mut self);
}

trait Frier {
    fn fry<T>(&self, obj: &mut T)
        where
            T: Friable;
}

struct Oven {}

impl Heater for Oven {
    fn heat<T>(&self, obj: &mut T) where T: Heatable {
        obj.cook();
    }
}

struct Pan {}

impl Frier for Pan {
    fn fry<T>(&self, obj: &mut T) where T: Friable {
        obj.cook();
    }
}

struct Pie {
    ready: bool,
}

struct Carrot {
    state: CarrotState,
}

enum CarrotState {
    Raw,
    Cooked,
    Fried,
    Burnt
}

trait Edible {
    fn eat(&self);
}

impl Heatable for Pie {
    fn cook(&mut self) {
        if self.ready {
            println!("You burned the pie!")
        } else {
            self.ready = true;
        }
    }
}

impl Heatable for Carrot {
    fn cook(&mut self) {
        match self.state {
            CarrotState::Raw => {self.state = Cooked}
            _ => {self.state = Burnt}
        }
    }
}

impl Friable for Carrot {
    fn cook(&mut self) {
        match self.state {
            CarrotState::Fried => {self.state = Burnt}
            Burnt => {}
            _ => {self.state = Fried}
        }
    }
}

impl Edible for Pie {
    fn eat(&self) {
        match self.ready {
            true => {println!("yummy")}
            false => {println!("you got stomach ache")}
        }
    }
}

impl Edible for Carrot {
    fn eat(&self) {
        match self.state {
            CarrotState::Raw => {println!("mmh, crunchy")}
            Cooked => {println!("mmh, yummy")}
            Fried => {println!("mmh, crispy")}
            Burnt => {println!("blah, burnt")}
        }
    }
}

pub fn food_main() {
    let mut carrot1 : Carrot = Carrot{state: Raw};
    let mut carrot2 : Carrot = Carrot{state: Fried};
    carrot1.eat();
    carrot2.eat();

    let frier : Pan = Pan{};
    frier.fry(&mut carrot1);
    frier.fry(&mut carrot2);
    carrot1.eat();
    carrot2.eat();

    let mut pie : Pie = Pie{ready: false};
    let mut carrot3 : Carrot = Carrot{state: Cooked};
    pie.eat();
    let oven : Oven = Oven{};
    oven.heat(&mut carrot3);
    oven.heat(&mut pie);
    pie.eat();
    carrot3.eat();
}